//! Explicit diagnostic feature: source addresses only, no guest state or clocks.
use std::{
    cell::{Cell, RefCell},
    collections::BTreeSet,
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
    words: BTreeSet<u32>,
    reads: BTreeSet<SiteRead>,
}

thread_local! {
    static READS: RefCell<Option<BTreeSet<Read>>> = const { RefCell::new(None) };
    static FETCH_DEPTH: Cell<usize> = const { Cell::new(0) };
    static SITE_READS: RefCell<Option<SiteReads>> = const { RefCell::new(None) };
    static EXECUTION_SITE: Cell<Option<ExecutionSite>> = const { Cell::new(None) };
}

/// Separate, thread-bound word-filtered diagnostic. Existing address-only
/// Capture keys and results are unchanged; no raw bytes enter this collection.
pub struct SiteCapture(PhantomData<Rc<()>>);
impl SiteCapture {
    pub fn start(words: impl IntoIterator<Item = u32>) -> Self {
        let words: BTreeSet<_> = words.into_iter().collect();
        assert!(words.iter().all(|w| w & 3 == 0 && *w < 0x1000000));
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
pub(crate) fn record(reader: Reader, address: u32, length: usize) {
    if length == 0
        || !(0x08..=0x0d).contains(&(address >> 24))
        || FETCH_DEPTH.with(|depth| depth.get() != 0)
    {
        return;
    }
    let read = Read {
        reader,
        guest_address: address,
        physical_offset: address & 0x01ff_ffff,
        length,
    };
    READS.with(|cell| {
        if let Some(reads) = cell.borrow_mut().as_mut() {
            reads.insert(read);
        }
    });
    SITE_READS.with(|cell| {
        if let Some(capture) = cell.borrow_mut().as_mut() {
            let first_word = read.physical_offset & !3;
            let end = u64::from(read.physical_offset) + length as u64;
            if capture
                .words
                .range(first_word..)
                .next()
                .is_some_and(|word| u64::from(*word) < end)
            {
                capture.reads.insert(SiteRead {
                    read,
                    instruction: EXECUTION_SITE.with(Cell::get),
                });
            }
        }
    });
}
