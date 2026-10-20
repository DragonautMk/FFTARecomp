//! Opt-in, bounded frame-time capture. No clock reads or allocations when off.
use std::time::{Duration, Instant};

const MAX_SAMPLES: usize = 360_000;
const FRAME_BUDGET: Duration = Duration::from_nanos(16_742_706);

pub struct FrameTimes {
    enabled: bool,
    samples: Vec<Duration>,
    omitted: u64,
    limit: usize,
}

impl FrameTimes {
    pub fn new(enabled: bool) -> Self {
        Self::with_limit(enabled, MAX_SAMPLES)
    }

    fn with_limit(enabled: bool, limit: usize) -> Self {
        Self {
            enabled,
            // Reserve outside the measured loop; capture itself never reallocates.
            samples: if enabled {
                Vec::with_capacity(limit)
            } else {
                Vec::new()
            },
            omitted: 0,
            limit,
        }
    }

    pub fn start(&self, measuring: bool) -> Option<Instant> {
        (self.enabled && measuring).then(Instant::now)
    }

    pub fn finish(&mut self, start: Option<Instant>) {
        if let Some(start) = start {
            self.record(start.elapsed());
        }
    }

    fn record(&mut self, duration: Duration) {
        if self.samples.len() < self.limit {
            self.samples.push(duration);
        } else {
            self.omitted += 1;
        }
    }

    /// Report after timing ends; sorting/logging must not enter the frame loop.
    pub fn report(&mut self) {
        self.report_named("FFTA_FRAME_TIMES");
    }

    fn report_named(&mut self, prefix: &str) {
        if !self.enabled || self.samples.is_empty() {
            return;
        }
        self.samples.sort_unstable();
        let n = self.samples.len();
        let percentile =
            |p: usize| self.samples[(n * p).div_ceil(100).saturating_sub(1)].as_secs_f64() * 1000.0;
        let over_budget = self.samples.iter().filter(|&&d| d > FRAME_BUDGET).count();
        let over_double = self
            .samples
            .iter()
            .filter(|&&d| d > FRAME_BUDGET * 2)
            .count();
        let total_ms = self.samples.iter().map(Duration::as_secs_f64).sum::<f64>() * 1000.0;
        println!(
            "{prefix} samples={n} omitted={} p50_ms={:.6} p95_ms={:.6} p99_ms={:.6} max_ms={:.6} over_budget={over_budget} over_double_budget={over_double} total_ms={total_ms:.6} mean_ms={:.6}",
            self.omitted, percentile(50), percentile(95), percentile(99), percentile(100), total_ms / n as f64,
        );
    }
}

/// Per-frame PPU wall time inside core execution. Parallel zoom includes the
/// caller's wait for workers; durations are not summed CPU usage across threads.
pub struct PpuProfile {
    series: [FrameTimes; 3],
    frames: u64,
    scanlines: u64,
    zoom: u64,
    invalid: u64,
}

impl PpuProfile {
    pub fn new(enabled: bool) -> Self {
        Self::with_limit(enabled, MAX_SAMPLES)
    }

    fn with_limit(enabled: bool, limit: usize) -> Self {
        Self {
            series: std::array::from_fn(|_| FrameTimes::with_limit(enabled, limit)),
            frames: 0,
            scanlines: 0,
            zoom: 0,
            invalid: 0,
        }
    }

    pub fn enabled(&self) -> bool {
        self.series[0].enabled
    }

    pub fn record(&mut self, timings: Option<gba_core::ppu_profile::Timings>) {
        if !self.enabled() {
            return;
        }
        if let Some(t) = timings {
            let ppu = t.scanlines + t.zoom;
            self.frames += 1;
            self.scanlines += t.scanline_calls as u64;
            self.zoom += t.zoom_calls as u64;
            self.invalid += u64::from(ppu > t.frame);
            self.series[0].record(t.scanlines);
            self.series[1].record(t.zoom);
            self.series[2].record(t.frame.saturating_sub(ppu));
        }
    }

    pub fn report(&mut self) {
        if !self.enabled() {
            return;
        }
        println!(
            "FFTA_PPU_COUNTS frames={} scanlines={} zoom={} invalid={}",
            self.frames, self.scanlines, self.zoom, self.invalid
        );
        for (series, name) in
            self.series
                .iter_mut()
                .zip(["hardware_scanlines", "zoom_rows", "core_other"])
        {
            series.report_named(&format!("FFTA_PPU_FRAME phase={name}"));
        }
    }
}

#[derive(Clone, Copy)]
#[repr(usize)]
pub enum Phase {
    Core,
    UiPrepare,
    TemporalPush,
    TemporalCompose,
    AudioHandoff,
    Autosave,
    Present,
    PacingWait,
    Tick,
    UiUpload,
    GpuSetupUpload,
    GpuAcquire,
    GpuEncode,
    GpuSubmit,
    GpuPresent,
}

const PHASE_NAMES: [&str; 15] = [
    "core_including_ppu",
    "ui_prepare",
    "temporal_push",
    "temporal_compose",
    "audio_handoff",
    "autosave",
    "present_including_acquire_upload_submit",
    "pacing_wait",
    "tick_after_input_including_work_and_wait",
    "ui_layout_upload",
    "gpu_cpu_setup_upload",
    "gpu_cpu_acquire",
    "gpu_cpu_encode",
    "gpu_cpu_submit",
    "gpu_cpu_present",
];

/// CPU wall times, with explicit inclusive scopes. This does not measure GPU
/// execution. The core includes PPU work; a tick includes the other stages.
pub struct PlayProfile {
    series: [FrameTimes; PHASE_NAMES.len()],
    ui_payload_bytes: u64,
    ui_updates: u64,
    ppu: PpuProfile,
}

impl PlayProfile {
    pub fn new(enabled: bool) -> Self {
        Self {
            series: std::array::from_fn(|_| FrameTimes::with_limit(enabled, 18_000)),
            ui_payload_bytes: 0,
            ui_updates: 0,
            ppu: PpuProfile::with_limit(enabled, 18_000),
        }
    }

    pub fn start(&self) -> Option<Instant> {
        self.series[0].start(true)
    }

    pub fn enabled(&self) -> bool {
        self.series[0].enabled
    }

    pub fn record_ppu(&mut self, timings: Option<gba_core::ppu_profile::Timings>) {
        self.ppu.record(timings);
    }

    pub fn record_ui_upload(&mut self, bytes: usize) {
        if self.enabled() {
            self.ui_payload_bytes += bytes as u64;
            self.ui_updates += 1;
        }
    }

    pub fn record(&mut self, phase: Phase, duration: Option<Duration>) {
        if self.enabled() {
            if let Some(duration) = duration {
                self.series[phase as usize].record(duration);
            }
        }
    }

    pub fn finish(&mut self, phase: Phase, start: Option<Instant>) {
        self.series[phase as usize].finish(start);
    }

    pub fn report(&mut self) {
        if !self.series[0].enabled {
            return;
        }
        self.ppu.report();
        println!(
            "FFTA_UI_UPLOAD samples={} payload_bytes={}",
            self.ui_updates, self.ui_payload_bytes
        );
        for (series, name) in self.series.iter_mut().zip(PHASE_NAMES) {
            series.report_named(&format!("FFTA_PLAY_PROFILE phase={name}"));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ppu_profile_records_disjoint_frame_totals_and_bounds_storage() {
        let mut profile = PpuProfile::with_limit(true, 1);
        let timings = gba_core::ppu_profile::Timings {
            frame: Duration::from_micros(10),
            scanlines: Duration::from_micros(6),
            zoom: Duration::from_micros(3),
            scanline_calls: 160,
            zoom_calls: 1,
        };
        profile.record(Some(timings));
        profile.record(Some(timings));
        assert_eq!(
            (
                profile.frames,
                profile.scanlines,
                profile.zoom,
                profile.invalid
            ),
            (2, 320, 2, 0)
        );
        for (series, micros) in profile.series.iter().zip([6, 3, 1]) {
            assert_eq!(series.samples, [Duration::from_micros(micros)]);
            assert_eq!(series.omitted, 1);
        }
        profile.record(Some(gba_core::ppu_profile::Timings {
            frame: Duration::ZERO,
            ..timings
        }));
        assert_eq!(profile.invalid, 1);
        let mut disabled = PpuProfile::new(false);
        disabled.record(Some(timings));
        assert_eq!(disabled.frames, 0);
        assert!(disabled.series.iter().all(|s| s.samples.capacity() == 0));
    }

    #[test]
    fn disabled_and_warmup_never_start_a_clock() {
        let mut off = FrameTimes::new(false);
        assert!(off.start(true).is_none());
        off.finish(None);
        assert!(off.samples.is_empty());
        assert!(FrameTimes::new(true).start(false).is_none());
    }

    #[test]
    fn capture_is_bounded_and_counts_omitted_samples() {
        let mut times = FrameTimes::new(true);
        for _ in 0..MAX_SAMPLES + 3 {
            times.record(Duration::from_millis(2));
        }
        assert_eq!(times.samples.len(), MAX_SAMPLES);
        assert_eq!(times.omitted, 3);
    }
}
