//! Explicit diagnostic feature: source addresses only, no guest state or clocks.
use std::{
    cell::{Cell, RefCell},
    collections::{BTreeMap, BTreeSet},
    marker::PhantomData,
    rc::Rc,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, serde::Serialize)]
pub enum Reader {
    Bus,
    AudioValue,
    SpritePeek,
    SpriteList,
    ObjSourceTable,
    KeyboardWidths,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, serde::Serialize)]
pub struct Read {
    pub reader: Reader,
    pub guest_address: u32,
    pub physical_offset: u32,
    pub length: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, serde::Serialize)]
pub struct ExecutionSite {
    pub pc: u32,
    pub thumb: bool,
}

/// A read during an interpreter instruction/HLE scope. Nested accesses such as
/// DMA inherit that scope: validate the opcode before attributing its own load.
/// None means outside interpreter execution, not an invented guest instruction.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, serde::Serialize)]
pub struct SiteRead {
    pub read: Read,
    pub instruction: Option<ExecutionSite>,
}

struct SiteReads {
    words: Option<BTreeSet<u32>>,
    reads: BTreeSet<SiteRead>,
}

thread_local! {
    static READS: RefCell<Option<BTreeSet<Read>>> = const { RefCell::new(None) };
    static FETCH_DEPTH: Cell<usize> = const { Cell::new(0) };
    static SITE_READS: RefCell<Option<SiteReads>> = const { RefCell::new(None) };
    static TRACE: RefCell<Option<TraceState>> = const { RefCell::new(None) };
    static EXECUTION_SITE: Cell<Option<ExecutionSite>> = const { Cell::new(None) };
}

/// Separate, thread-bound word-filtered diagnostic. Existing address-only
/// Capture keys and results are unchanged; no raw bytes enter this collection.
pub struct SiteCapture(PhantomData<Rc<()>>);
impl SiteCapture {
    pub fn start(words: impl IntoIterator<Item = u32>) -> Self {
        let words: BTreeSet<_> = words.into_iter().collect();
        assert!(words.iter().all(|w| w & 3 == 0 && *w < 0x1000000));
        Self::begin(Some(words))
    }
    /// Address inventory with execution scope before target words are known.
    /// An explicit all-address mode keeps an empty word filter meaning empty.
    pub fn start_all() -> Self {
        Self::begin(None)
    }
    fn begin(words: Option<BTreeSet<u32>>) -> Self {
        SITE_READS.with(|cell| {
            let mut capture = cell.borrow_mut();
            assert!(capture.is_none(), "ROM site capture is already active");
            *capture = Some(SiteReads {
                words,
                reads: BTreeSet::new(),
            });
        });
        Self(PhantomData)
    }
    pub fn finish(self) -> BTreeSet<SiteRead> {
        SITE_READS.with(|cell| {
            cell.borrow_mut()
                .take()
                .expect("active ROM site capture")
                .reads
        })
    }
}
impl Drop for SiteCapture {
    fn drop(&mut self) {
        SITE_READS.with(|cell| *cell.borrow_mut() = None);
    }
}

pub(crate) struct InstructionExecution(Option<ExecutionSite>);
impl InstructionExecution {
    pub(crate) fn enter(pc: u32, thumb: bool) -> Self {
        Self(EXECUTION_SITE.with(|cell| cell.replace(Some(ExecutionSite { pc, thumb }))))
    }
}
impl Drop for InstructionExecution {
    fn drop(&mut self) {
        EXECUTION_SITE.with(|cell| cell.set(self.0));
    }
}

/// Thread-bound, non-nestable capture. Dropping also resets capture after panic.
/// No machine pointer, ROM contents, mutable hardware or timing enters the set.
pub struct Capture(PhantomData<Rc<()>>);
impl Capture {
    pub fn start() -> Self {
        READS.with(|cell| {
            let mut reads = cell.borrow_mut();
            assert!(reads.is_none(), "ROM read capture is already active");
            *reads = Some(BTreeSet::new());
        });
        Self(PhantomData)
    }

    pub fn finish(self) -> BTreeSet<Read> {
        READS.with(|cell| cell.borrow_mut().take().expect("active ROM read capture"))
    }
}
impl Drop for Capture {
    fn drop(&mut self) {
        READS.with(|cell| *cell.borrow_mut() = None);
    }
}

/// Only surrounds the interpreter's instruction-byte fetch, never execution,
/// HLE handling or ticking. This guard has no effect in normal builds.
pub(crate) struct InstructionFetch;
impl InstructionFetch {
    pub(crate) fn enter() -> Self {
        FETCH_DEPTH.with(|depth| depth.set(depth.get() + 1));
        Self
    }
}
impl Drop for InstructionFetch {
    fn drop(&mut self) {
        FETCH_DEPTH.with(|depth| depth.set(depth.get() - 1));
    }
}

/// Called only for successful physical-image/typed-source reads. The caller
/// preserves its existing region/bounds/RTC/EEPROM decisions and return value.
#[allow(dead_code)] // Legacy unclassified hook for unselected host integrations.
pub(crate) fn record(reader: Reader, address: u32, length: usize) {
    record_with_service(reader, address, length, Service::Unclassified);
}

pub(crate) fn record_served(reader: Reader, address: u32, length: usize, typed: bool) {
    record_with_service(
        reader,
        address,
        length,
        if typed { Service::Typed } else { Service::Raw },
    );
}

fn record_with_service(reader: Reader, address: u32, length: usize, service: Service) {
    if FETCH_DEPTH.with(|depth| depth.get() != 0) {
        TRACE.with(|trace| {
            if let Some(state) = trace.borrow_mut().as_mut().filter(|s| s.frame.is_some()) {
                state.fetch_requests_excluded = state.fetch_requests_excluded.saturating_add(1);
            }
        });
        return;
    }
    if length == 0 || !(0x08..=0x0d).contains(&(address >> 24)) {
        return;
    }
    let read = Read {
        reader,
        guest_address: address,
        physical_offset: address & 0x01ff_ffff,
        length,
    };
    TRACE.with(|trace| {
        if let Some(state) = trace.borrow_mut().as_mut() {
            state.record(read, service);
        }
    });
    READS.with(|cell| {
        if let Some(reads) = cell.borrow_mut().as_mut() {
            reads.insert(read);
        }
    });
    SITE_READS.with(|cell| {
        if let Some(capture) = cell.borrow_mut().as_mut() {
            let first_word = read.physical_offset & !3;
            let end = u64::from(read.physical_offset) + length as u64;
            if capture.words.as_ref().is_none_or(|words| {
                words
                    .range(first_word..)
                    .next()
                    .is_some_and(|word| u64::from(*word) < end)
            }) {
                capture.reads.insert(SiteRead {
                    read,
                    instruction: EXECUTION_SITE.with(Cell::get),
                });
            }
        }
    });
}

/// Actual serving path at the instrumented access, independent of known ownership.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, serde::Serialize)]
pub enum Service {
    Typed,
    Raw,
    Unclassified,
}
#[derive(Clone, Debug)]
pub struct TraceFilter {
    /// Empty means every instrumented reader.
    pub readers: Vec<Reader>,
    /// Physical half-open ranges; empty means the whole image.
    pub ranges: Vec<(u32, u32)>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, serde::Serialize)]
pub struct TraceKey {
    pub read: Read,
    pub service: Service,
    /// Interpreter scope, possibly inherited by HLE/DMA. Never a native PC guess.
    pub instruction: Option<ExecutionSite>,
}
#[derive(Clone, Debug, serde::Serialize)]
pub struct TraceEvent {
    pub key: TraceKey,
    pub first_guest_frame: u64,
    pub last_guest_frame: u64,
    pub requests: u64,
}
#[derive(serde::Serialize)]
pub struct TraceReport {
    pub events: Vec<TraceEvent>,
    pub matched_requests: u64,
    pub dropped_requests: u64,
    pub fetch_requests_excluded: u64,
    pub truncated: bool,
}
struct TraceState {
    filter: TraceFilter,
    limit: usize,
    frame: Option<u64>,
    events: BTreeMap<TraceKey, TraceEvent>,
    matched: u64,
    dropped: u64,
    fetch_requests_excluded: u64,
}
impl TraceState {
    fn record(&mut self, read: Read, service: Service) {
        let Some(frame) = self.frame else {
            return;
        };
        if !self.filter.readers.is_empty() && !self.filter.readers.contains(&read.reader) {
            return;
        }
        let end = u64::from(read.physical_offset) + read.length as u64;
        if !self.filter.ranges.is_empty()
            && !self
                .filter
                .ranges
                .iter()
                .any(|&(a, b)| u64::from(a) < end && b > read.physical_offset)
        {
            return;
        }
        self.matched = self.matched.saturating_add(1);
        let key = TraceKey {
            read,
            service,
            instruction: EXECUTION_SITE.with(Cell::get),
        };
        if let Some(event) = self.events.get_mut(&key) {
            event.requests = event.requests.saturating_add(1);
            event.last_guest_frame = frame;
            return;
        }
        if self.events.len() >= self.limit {
            self.dropped = self.dropped.saturating_add(1);
            return;
        }
        self.events.insert(
            key,
            TraceEvent {
                key,
                first_guest_frame: frame,
                last_guest_frame: frame,
                requests: 1,
            },
        );
    }
}
/// Bounded, filtered and thread-bound; independent of the two legacy collectors.
pub struct TraceCapture(PhantomData<Rc<()>>);
impl TraceCapture {
    pub fn start(filter: TraceFilter, limit: usize) -> Result<Self, String> {
        if !(1..=50000).contains(&limit)
            || filter.ranges.len() > 32
            || filter.ranges.iter().any(|&(a, b)| a >= b || b > 0x02000000)
        {
            return Err("Invalid ROM trace bounds".into());
        }
        TRACE.with(|trace| {
            let mut state = trace.borrow_mut();
            if state.is_some() {
                return Err("ROM trace already active on this thread".into());
            }
            *state = Some(TraceState {
                filter,
                limit,
                frame: None,
                events: BTreeMap::new(),
                matched: 0,
                dropped: 0,
                fetch_requests_excluded: 0,
            });
            Ok(Self(PhantomData))
        })
    }
    pub fn frame(&self, frame: Option<u64>) {
        TRACE.with(|trace| {
            if let Some(s) = trace.borrow_mut().as_mut() {
                s.frame = frame;
            }
        });
    }
    pub fn finish(self) -> TraceReport {
        let state = TRACE.with(|trace| trace.borrow_mut().take().expect("active trace"));
        TraceReport {
            events: state.events.into_values().collect(),
            matched_requests: state.matched,
            dropped_requests: state.dropped,
            fetch_requests_excluded: state.fetch_requests_excluded,
            truncated: state.dropped > 0,
        }
    }
}
impl Drop for TraceCapture {
    fn drop(&mut self) {
        TRACE.with(|trace| *trace.borrow_mut() = None);
    }
}

#[cfg(test)]
mod bounded_tests {
    use super::*;
    #[test]
    fn typed_raw_unclassified_and_inherited_sites_stay_separate() {
        let trace = TraceCapture::start(
            TraceFilter {
                readers: Vec::new(),
                ranges: Vec::new(),
            },
            8,
        )
        .unwrap();
        trace.frame(Some(17));
        record_served(Reader::Bus, 0x08000100, 4, true);
        {
            let _site = InstructionExecution::enter(0x08000010, true);
            record_served(Reader::AudioValue, 0x0a000100, 2, false);
            record(Reader::SpritePeek, 0x08000100, 2);
        }
        let r = trace.finish();
        assert_eq!(r.events.len(), 3);
        assert!(r
            .events
            .iter()
            .any(|e| e.key.service == Service::Typed && e.key.instruction.is_none()));
        assert!(r.events.iter().any(|e| e.key.service == Service::Raw
            && e.key.instruction
                == Some(ExecutionSite {
                    pc: 0x08000010,
                    thumb: true
                })));
        assert!(r
            .events
            .iter()
            .any(|e| e.key.service == Service::Unclassified));
    }
    #[test]
    fn trace_drop_nested_refusal_and_fetch_suppression_are_bounded() {
        let filter = TraceFilter {
            readers: Vec::new(),
            ranges: Vec::new(),
        };
        let trace = TraceCapture::start(filter.clone(), 1).unwrap();
        trace.frame(Some(0));
        assert!(TraceCapture::start(filter.clone(), 1).is_err());
        {
            let _fetch = InstructionFetch::enter();
            record_served(Reader::Bus, 0x08000000, 4, false);
        }
        let result = trace.finish();
        assert_eq!(result.events.len(), 0);
        assert_eq!(result.fetch_requests_excluded, 1);
        drop(TraceCapture::start(filter.clone(), 1).unwrap());
        assert!(TraceCapture::start(filter, 1).is_ok());
    }
}
