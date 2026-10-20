//! Scoped, thread-local host timing. Never part of a machine snapshot or guest clock.
use std::{
    cell::Cell,
    marker::PhantomData,
    rc::Rc,
    time::{Duration, Instant},
};

#[derive(Clone, Copy, Debug, Default)]
pub struct Timings {
    pub frame: Duration,
    pub scanlines: Duration,
    pub zoom: Duration,
    pub scanline_calls: u32,
    pub zoom_calls: u32,
}

thread_local! {
    static ACTIVE: Cell<Option<Timings>> = const { Cell::new(None) };
}

/// Captures one core interval on this thread. Drop restores any enclosing
/// capture, including on errors/panics. Disabled scopes never read a clock.
/// Keep this guard on the emulation thread; parallel zoom is timed at its caller.
pub struct Capture {
    start: Option<Instant>,
    previous: Option<Timings>,
    _thread: PhantomData<Rc<()>>,
}

impl Capture {
    pub fn start(enabled: bool) -> Self {
        let start = enabled.then(Instant::now);
        let previous = if enabled {
            ACTIVE.with(|slot| slot.replace(Some(Timings::default())))
        } else {
            None
        };
        Self {
            start,
            previous,
            _thread: PhantomData,
        }
    }

    pub fn finish(mut self) -> Option<Timings> {
        let start = self.start.take()?;
        let mut result = ACTIVE
            .with(|slot| slot.replace(self.previous))
            .expect("active PPU capture");
        result.frame = start.elapsed();
        Some(result)
    }
}

impl Drop for Capture {
    fn drop(&mut self) {
        if self.start.is_some() {
            ACTIVE.with(|slot| slot.set(self.previous));
        }
    }
}

#[derive(Clone, Copy)]
enum Stage {
    Scanlines,
    Zoom,
}

pub(crate) struct StageTimer {
    start: Instant,
    stage: Stage,
    _thread: PhantomData<Rc<()>>,
}

fn timer(stage: Stage) -> Option<StageTimer> {
    ACTIVE.with(|slot| {
        slot.get().map(|_| StageTimer {
            start: Instant::now(),
            stage,
            _thread: PhantomData,
        })
    })
}

pub(crate) fn scanline_timer() -> Option<StageTimer> {
    timer(Stage::Scanlines)
}
pub(crate) fn zoom_timer() -> Option<StageTimer> {
    timer(Stage::Zoom)
}

impl Drop for StageTimer {
    fn drop(&mut self) {
        let elapsed = self.start.elapsed();
        ACTIVE.with(|slot| {
            if let Some(mut timings) = slot.get() {
                match self.stage {
                    Stage::Scanlines => {
                        timings.scanlines += elapsed;
                        timings.scanline_calls += 1;
                    }
                    Stage::Zoom => {
                        timings.zoom += elapsed;
                        timings.zoom_calls += 1;
                    }
                }
                slot.set(Some(timings));
            }
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn disabled_capture_and_other_threads_do_not_collect() {
        assert!(scanline_timer().is_none());
        assert!(Capture::start(false).finish().is_none());
        let capture = Capture::start(true);
        std::thread::spawn(|| assert!(scanline_timer().is_none()))
            .join()
            .unwrap();
        drop(scanline_timer());
        drop(zoom_timer());
        let result = capture.finish().unwrap();
        assert_eq!((result.scanline_calls, result.zoom_calls), (1, 1));
        assert!(result.frame >= result.scanlines + result.zoom);
        assert!(scanline_timer().is_none());
    }

    #[test]
    fn real_scanline_capture_preserves_complete_machine_state() {
        for dispcnt in [0x80u16, 0x403] {
            let mut machine = crate::Machine::new(0xeafffffeu32.to_le_bytes().to_vec());
            machine.bus.io[..2].copy_from_slice(&dispcnt.to_le_bytes());
            let checkpoint = machine.save_state().unwrap();
            machine.run_frame(100_000);
            let reference = machine.save_state().unwrap();
            machine.load_state(&checkpoint).unwrap();
            let capture = Capture::start(true);
            machine.run_frame(100_000);
            let timings = capture.finish().unwrap();
            assert_eq!(machine.bus.frames, 1);
            assert_eq!((timings.scanline_calls, timings.zoom_calls), (160, 0));
            assert!(timings.frame >= timings.scanlines);
            assert!(
                machine.save_state().unwrap() == reference,
                "profiling changed guest state"
            );
        }
    }

    #[test]
    fn nested_and_unwinding_scopes_restore_the_previous_capture() {
        let outer = Capture::start(true);
        drop(scanline_timer());
        let inner = Capture::start(true);
        drop(zoom_timer());
        let result = inner.finish().unwrap();
        assert_eq!((result.scanline_calls, result.zoom_calls), (0, 1));
        assert!(std::panic::catch_unwind(|| {
            let _capture = Capture::start(true);
            drop(zoom_timer());
            panic!("test cleanup");
        })
        .is_err());
        drop(scanline_timer());
        let result = outer.finish().unwrap();
        assert_eq!((result.scanline_calls, result.zoom_calls), (2, 0));
        assert!(scanline_timer().is_none());
    }
}
