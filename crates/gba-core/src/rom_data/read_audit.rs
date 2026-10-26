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

thread_local! {
    static READS: RefCell<Option<BTreeSet<Read>>> = const { RefCell::new(None) };
    static FETCH_DEPTH: Cell<usize> = const { Cell::new(0) };
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
    READS.with(|cell| {
        if let Some(reads) = cell.borrow_mut().as_mut() {
            reads.insert(Read {
                reader,
                guest_address: address,
                physical_offset: address & 0x01ff_ffff,
                length,
            });
        }
    });
}
