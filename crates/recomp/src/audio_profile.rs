//! Host callback diagnostics, owned under the existing audio-ring mutex.
//! Only allocated when requested; reporting runs after the stream is dropped.
use crate::profile_window::FrameWindow;
use std::time::{Duration, Instant};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(usize)]
pub(crate) enum Context {
    Playing,
    Priming,
    Paused,
    FastForward,
}

impl Context {
    pub fn of(paused: bool, primed: bool, fast_forward: bool) -> Self {
        if paused {
            Self::Paused
        } else if fast_forward {
            Self::FastForward
        } else if !primed {
            Self::Priming
        } else {
            Self::Playing
        }
    }

    fn name(self) -> &'static str {
        match self {
            Self::Playing => "playing",
            Self::Priming => "priming",
            Self::Paused => "paused",
            Self::FastForward => "fast_forward",
        }
    }
}

const CONTEXTS: [Context; 4] = [
    Context::Playing,
    Context::Priming,
    Context::Paused,
    Context::FastForward,
];

struct Sample {
    lock: Duration,
    render: Duration,
    total: Duration,
    interval: Option<Duration>,
}

/// Raw source demand, not an audible-underflow or driver-XRUN count.
#[derive(Clone, Copy, Default, Debug, PartialEq, Eq)]
pub(crate) struct SourceReads {
    pub requests: u64,
    pub empty: u64,
}

#[derive(Clone, Copy, Default)]
struct SourceTotals {
    reads: SourceReads,
    target_reads: SourceReads,
    callbacks_with_empty: u64,
    target_callbacks: u64,
}

struct Series {
    samples: Vec<Sample>,
    limit: usize,
    callbacks: u64,
    frames: u64,
    holds: u64,
    callbacks_with_holds: u64,
    over_period: u64,
    dropped_items: u64,
    min_mixed_pairs: usize,
    max_mixed_pairs: usize,
    sources: [SourceTotals; 4],
}

pub(crate) struct Profile {
    context: Context,
    series: [Series; 4],
    previous: Option<Instant>,
    callback_builds: u64,
    rate: u32,
    channels: usize,
    pub device_errors: u64,
    window: Option<FrameWindow>,
    excluded_callbacks: u64,
    excluded_dropped_items: u64,
}

impl Profile {
    pub fn new() -> Self {
        // Pause/priming cannot consume the playing sample allowance.
        Self::with_limits([9_000, 3_000, 3_000, 3_000])
    }

    pub fn with_frame_window(value: Option<&str>) -> Result<Self, String> {
        let mut profile = Self::new();
        profile.window = value
            .map(|v| FrameWindow::parse(v, "FFTA_AUDIO_PROFILE_FRAMES"))
            .transpose()?;
        Ok(profile)
    }

    fn capturing(&self) -> bool {
        self.window.as_ref().is_none_or(FrameWindow::active)
    }

    pub fn publish_frame(&mut self, frame: u64) {
        let was_active = self.capturing();
        if let Some(window) = &mut self.window {
            window.publish(frame);
        }
        if self.capturing() != was_active {
            self.previous = None;
        }
    }

    pub fn restored_frame(&mut self, frame: u64) {
        let was_active = self.capturing();
        if let Some(window) = &mut self.window {
            window.restore(frame);
        }
        if self.capturing() != was_active {
            self.previous = None;
        }
    }

    fn with_limits(limits: [usize; 4]) -> Self {
        Self {
            context: Context::Priming,
            series: limits.map(|limit| Series {
                samples: Vec::with_capacity(limit),
                limit,
                callbacks: 0,
                frames: 0,
                holds: 0,
                callbacks_with_holds: 0,
                over_period: 0,
                dropped_items: 0,
                min_mixed_pairs: usize::MAX,
                max_mixed_pairs: 0,
                sources: [SourceTotals::default(); 4],
            }),
            previous: None,
            callback_builds: 0,
            rate: 0,
            channels: 0,
            device_errors: 0,
            window: None,
            excluded_callbacks: 0,
            excluded_dropped_items: 0,
        }
    }

    pub fn set_context(&mut self, context: Context) {
        if self.context != context {
            self.previous = None;
            self.context = context;
        }
    }

    pub fn attach(&mut self, rate: u32, channels: usize) {
        self.callback_builds += 1;
        self.rate = rate;
        self.channels = channels;
        // A replacement can start delivering before the watchdog has updated
        // the app's primed flag. Never label its first empty buffer as playing.
        if self.context == Context::Playing {
            self.context = Context::Priming;
        }
        self.previous = None;
    }

    pub fn dropped(&mut self, items: u64) {
        if !self.capturing() {
            self.excluded_dropped_items += items;
            return;
        }
        self.series[self.context as usize].dropped_items += items;
    }

    pub fn sources(&mut self, reads: [SourceReads; 4], configured_target: [bool; 4]) {
        if !self.capturing() {
            return;
        }
        for ((total, reads), target) in self.series[self.context as usize]
            .sources
            .iter_mut()
            .zip(reads)
            .zip(configured_target)
        {
            total.reads.requests += reads.requests;
            total.reads.empty += reads.empty;
            total.callbacks_with_empty += u64::from(reads.empty != 0);
            if target {
                total.target_reads.requests += reads.requests;
                total.target_reads.empty += reads.empty;
                total.target_callbacks += 1;
            }
        }
    }

    pub fn callback(
        &mut self,
        clocks: [Instant; 3],
        frames: usize,
        holds: u64,
        mixed_pairs_before: usize,
    ) {
        if !self.capturing() {
            self.excluded_callbacks += 1;
            self.previous = None;
            return;
        }
        let [begin, acquired, end] = clocks;
        let interval = self.previous.replace(begin).map(|last| begin - last);
        let s = &mut self.series[self.context as usize];
        s.callbacks += 1;
        s.frames += frames as u64;
        s.holds += holds;
        s.callbacks_with_holds += u64::from(holds != 0);
        let total = end - begin;
        if self.rate != 0 && frames != 0 {
            let period = Duration::from_secs_f64(frames as f64 / f64::from(self.rate));
            s.over_period += u64::from(total > period);
        }
        s.min_mixed_pairs = s.min_mixed_pairs.min(mixed_pairs_before);
        s.max_mixed_pairs = s.max_mixed_pairs.max(mixed_pairs_before);
        if s.samples.len() < s.limit {
            s.samples.push(Sample {
                lock: acquired - begin,
                render: end - acquired,
                total,
                interval,
            });
        }
    }

    pub fn report(&self) {
        if let Some(window) = &self.window {
            println!(
                "FFTA_AUDIO_WINDOW start={} end_exclusive={} last_published={} closed={} interrupted={} excluded_callbacks={} excluded_dropped_items={}",
                window.start, window.end,
                window.frame.map_or_else(|| "none".into(), |f| f.to_string()),
                window.closed, window.interrupted, self.excluded_callbacks, self.excluded_dropped_items
            );
        }
        println!(
            "FFTA_AUDIO_DEVICE callback_builds={} last_rate={} last_channels={} device_errors={}",
            self.callback_builds, self.rate, self.channels, self.device_errors
        );
        for context in CONTEXTS {
            let s = &self.series[context as usize];
            let name = context.name();
            for (source, total) in ["psg", "hle", "fifo_a", "fifo_b"]
                .into_iter()
                .zip(s.sources)
            {
                println!(
                    "FFTA_AUDIO_SOURCE context={name} source={source} requests={} empty_requests={} configured_target_requests={} configured_target_empty={} callbacks_with_empty={} configured_target_callbacks={}",
                    total.reads.requests, total.reads.empty, total.target_reads.requests,
                    total.target_reads.empty, total.callbacks_with_empty, total.target_callbacks
                );
            }
            println!(
                "FFTA_AUDIO_COUNTS context={name} callbacks={} output_frames={} faithful_source_holds={} callbacks_with_holds={} callback_over_period={} producer_dropped_items={} stored={} omitted={} mixed_pairs_min={} mixed_pairs_max={}",
                s.callbacks,
                s.frames,
                s.holds,
                s.callbacks_with_holds,
                s.over_period,
                s.dropped_items,
                s.samples.len(),
                s.callbacks - s.samples.len() as u64,
                if s.callbacks == 0 {
                    0
                } else {
                    s.min_mixed_pairs
                },
                s.max_mixed_pairs
            );
            for (phase, values) in [
                ("lock_wait", s.samples.iter().map(|v| v.lock).collect()),
                ("render", s.samples.iter().map(|v| v.render).collect()),
                ("total", s.samples.iter().map(|v| v.total).collect()),
                (
                    "start_interval",
                    s.samples.iter().filter_map(|v| v.interval).collect(),
                ),
            ] {
                report_metric(name, phase, values);
            }
        }
    }
}

fn report_metric(context: &str, phase: &str, mut values: Vec<Duration>) {
    if values.is_empty() {
        return;
    }
    values.sort_unstable();
    let n = values.len();
    let percentile =
        |p: usize| values[(n * p).div_ceil(100).saturating_sub(1)].as_secs_f64() * 1000.;
    let total = values.iter().map(Duration::as_secs_f64).sum::<f64>() * 1000.;
    println!(
        "FFTA_AUDIO_PROFILE context={context} phase={phase} samples={n} p50_ms={:.6} p95_ms={:.6} p99_ms={:.6} max_ms={:.6} mean_ms={:.6}",
        percentile(50),
        percentile(95),
        percentile(99),
        percentile(100),
        total / n as f64
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn actual_callback_labels_configured_targets_without_claiming_audibility() {
        use std::sync::{Arc, Mutex};
        for (enhanced, hle, route, backlog, expected) in [
            (false, true, [3, 3], false, [false; 4]),
            (true, true, [3, 0], false, [true, true, false, false]),
            (true, false, [0, 3], false, [true, false, false, true]),
            (true, true, [0, 0], false, [true, false, false, false]),
            (true, true, [3, 3], true, [false; 4]),
        ] {
            let streams = Arc::new(Mutex::new(crate::AudioStreams {
                profile: Some(Profile::new()),
                enhanced_on: enhanced,
                hle_on: hle,
                route,
                muted: true, // configured target is independent of output audibility
                normal_audio: backlog.then(|| crate::ff_audio::NormalAudio::new().unwrap()),
                ..Default::default()
            }));
            let mut render = crate::output_callback::<f32>(48_000., 2, streams.clone());
            render(&mut [0.; 1024]);
            let st = streams.lock().unwrap();
            let s = &st.profile.as_ref().unwrap().series[Context::Priming as usize];
            for (total, target) in s.sources.iter().zip(expected) {
                assert!(total.reads.requests > 0);
                assert_eq!(total.reads.requests, total.reads.empty);
                assert_eq!(total.target_callbacks, u64::from(target));
                assert_eq!(
                    total.target_reads,
                    if target {
                        total.reads
                    } else {
                        SourceReads::default()
                    }
                );
            }
        }
    }

    #[test]
    fn source_totals_separate_configured_targets_and_ignore_outside_window() {
        let mut p = Profile::with_limits([1; 4]);
        p.window = Some(FrameWindow::parse("100:103", "test").unwrap());
        p.set_context(Context::Playing);
        let reads = [
            SourceReads {
                requests: 4,
                empty: 1,
            },
            SourceReads {
                requests: 2,
                empty: 2,
            },
            SourceReads {
                requests: 3,
                empty: 0,
            },
            SourceReads::default(),
        ];
        p.sources(reads, [true; 4]); // no published frame
        let t = Instant::now();
        for (frame, target) in [
            (100, [true, false, true, false]),
            (101, [false, true, false, false]),
            (103, [true; 4]),
        ] {
            p.publish_frame(frame);
            p.callback([t, t, t], 48, 0, 10);
            p.sources(reads, target);
        }
        let s = &p.series[Context::Playing as usize];
        assert_eq!((s.callbacks, s.samples.len()), (2, 1));
        assert_eq!(
            s.sources[0].reads,
            SourceReads {
                requests: 8,
                empty: 2
            }
        );
        assert_eq!(s.sources[0].target_reads, reads[0]);
        assert_eq!(
            s.sources[1].reads,
            SourceReads {
                requests: 4,
                empty: 4
            }
        );
        assert_eq!(s.sources[1].target_reads, reads[1]);
        assert_eq!(
            (
                s.sources[0].callbacks_with_empty,
                s.sources[0].target_callbacks
            ),
            (2, 1)
        );
        assert_eq!(
            s.sources[2].reads,
            SourceReads {
                requests: 6,
                empty: 0
            }
        );
        assert_eq!(
            p.series[Context::Paused as usize].sources[0].reads,
            SourceReads::default()
        );
    }

    #[test]
    fn frame_window_rejects_bad_ranges_and_excludes_boundaries_and_rewinds() {
        for value in ["", "10", "10:", ":20", "20:10", "10:10", "-1:20", "1:2:3"] {
            assert!(Profile::with_frame_window(Some(value)).is_err(), "{value}");
        }
        let mut p = Profile::with_frame_window(Some("100:103")).unwrap();
        p.attach(48_000, 2);
        p.set_context(Context::Playing);
        let t = Instant::now();
        let call = |p: &mut Profile| {
            p.callback([t, t, t], 48, 2, 10);
            p.dropped(3);
        };
        call(&mut p); // no published frame
        for frame in [99, 100, 101, 102, 103, 100] {
            p.publish_frame(frame);
            call(&mut p);
        }
        let s = &p.series[Context::Playing as usize];
        assert_eq!((s.callbacks, s.holds, s.dropped_items), (3, 6, 9));
        assert_eq!((p.excluded_callbacks, p.excluded_dropped_items), (4, 12));
        assert!(s.samples[0].interval.is_none());
        let w = p.window.as_ref().unwrap();
        assert!(w.closed && !w.interrupted);
        let mut p = Profile::with_frame_window(Some("100:200")).unwrap();
        p.publish_frame(100);
        call(&mut p);
        p.publish_frame(99);
        call(&mut p);
        p.publish_frame(100);
        call(&mut p);
        assert_eq!(p.series[Context::Priming as usize].callbacks, 1);
        assert!(p.window.as_ref().unwrap().interrupted);
        for restored in [100, 150, 250] {
            let mut p = Profile::with_frame_window(Some("100:200")).unwrap();
            p.restored_frame(50); // scene load before warm-up is allowed
            p.publish_frame(100);
            call(&mut p);
            p.restored_frame(restored);
            call(&mut p);
            assert_eq!(p.series[Context::Priming as usize].callbacks, 1);
            assert!(p.window.as_ref().unwrap().interrupted);
        }
    }

    #[test]
    fn frame_gate_keeps_actual_audio_and_queue_state_identical() {
        use std::sync::{Arc, Mutex};
        let make = |profile| {
            Arc::new(Mutex::new(crate::AudioStreams {
                profile,
                enhanced_on: true,
                hle_on: true,
                route: [3, 3],
                mixed: (0..2000).map(|i| (i % 128) as i16).collect(),
                psg: (0..2000).map(|i| (i % 64) as f32 / 128.).collect(),
                hle: (0..2000).map(|i| (i % 32) as f32 / 256.).collect(),
                ..Default::default()
            }))
        };
        let plain = make(None);
        let gated = make(Some(Profile::with_frame_window(Some("100:103")).unwrap()));
        let mut a = crate::output_callback::<f32>(48_000., 2, plain.clone());
        let mut b = crate::output_callback::<f32>(48_000., 2, gated.clone());
        for (frame, context) in [
            (99, Context::Playing),
            (100, Context::Playing),
            (101, Context::Paused),
            (102, Context::Playing),
            (103, Context::FastForward),
            (100, Context::Playing),
        ] {
            {
                let mut st = gated.lock().unwrap();
                let p = st.profile.as_mut().unwrap();
                p.set_context(context);
                p.publish_frame(frame);
            }
            let mut x = [0.; 1024];
            let mut y = x;
            a(&mut x);
            b(&mut y);
            assert_eq!(x, y);
            let a = plain.lock().unwrap();
            let b = gated.lock().unwrap();
            assert_eq!(
                (&a.mixed, &a.psg, &a.hle, &a.fifo),
                (&b.mixed, &b.psg, &b.hle, &b.fifo)
            );
            assert_eq!(
                (a.holds, a.consumed, a.clipped),
                (b.holds, b.consumed, b.clipped)
            );
        }
        let st = gated.lock().unwrap();
        let p = st.profile.as_ref().unwrap();
        assert_eq!(p.series[Context::Playing as usize].callbacks, 2);
        assert_eq!(p.series[Context::Paused as usize].callbacks, 1);
        assert_eq!(p.excluded_callbacks, 3);
        assert!(p.series[Context::Playing as usize]
            .samples
            .iter()
            .all(|v| v.interval.is_none()));
    }

    #[test]
    fn capped_capture_keeps_totals_and_separates_pause_priming_and_fast_forward() {
        let mut p = Profile::with_limits([1; 4]);
        p.attach(48_000, 2);
        let t = Instant::now();
        let callback = |offset| {
            [
                t + Duration::from_millis(offset),
                t + Duration::from_millis(offset + 1),
                t + Duration::from_millis(offset + 3),
            ]
        };
        p.callback(callback(0), 48, 2, 0);
        p.callback(callback(10), 48, 0, 80);
        let s = &p.series[Context::Priming as usize];
        assert_eq!(
            (
                s.callbacks,
                s.frames,
                s.holds,
                s.callbacks_with_holds,
                s.over_period
            ),
            (2, 96, 2, 1, 2)
        );
        assert_eq!(s.samples.len(), 1);
        assert_eq!((s.min_mixed_pairs, s.max_mixed_pairs), (0, 80));
        assert_eq!(s.samples[0].lock, Duration::from_millis(1));
        assert_eq!(s.samples[0].render, Duration::from_millis(2));
        for context in [Context::Playing, Context::Paused, Context::FastForward] {
            p.set_context(context);
            p.callback(callback(20), 480, 1, 20);
            p.dropped(3);
            let s = &p.series[context as usize];
            assert_eq!(
                (s.callbacks, s.holds, s.dropped_items, s.over_period),
                (1, 1, 3, 0)
            );
            assert!(s.samples[0].interval.is_none());
        }
        p.attach(44_100, 1);
        assert!(p.previous.is_none());
        assert_eq!(p.callback_builds, 2);
        p.set_context(Context::Playing);
        p.attach(48_000, 2);
        assert_eq!(p.context, Context::Priming);
        assert_eq!(Context::of(true, false, true), Context::Paused);
        assert_eq!(Context::of(false, false, true), Context::FastForward);
        assert_eq!(Context::of(false, false, false), Context::Priming);
        assert_eq!(Context::of(false, true, false), Context::Playing);
    }

    #[test]
    fn actual_callback_keeps_holds_across_legacy_resets_and_context_changes() {
        use std::sync::{Arc, Mutex};
        let streams = Arc::new(Mutex::new(crate::AudioStreams {
            profile: Some(Profile::with_limits([1; 4])),
            ..Default::default()
        }));
        let mut output = crate::output_callback::<f32>(48_000., 2, streams.clone());
        let mut samples = [0.; 256];
        let mut expected = 0;
        for _ in 0..3 {
            output(&mut samples);
            let mut st = streams.lock().unwrap();
            expected += st.holds;
            st.holds = 0;
        }
        {
            let mut st = streams.lock().unwrap();
            let p = st.profile.as_mut().unwrap();
            let s = &p.series[Context::Priming as usize];
            assert!(expected > 0);
            assert_eq!((s.callbacks, s.holds, s.samples.len()), (3, expected, 1));
            p.set_context(Context::Paused);
        }
        output(&mut samples);
        let st = streams.lock().unwrap();
        let p = st.profile.as_ref().unwrap();
        assert_eq!(p.series[Context::Priming as usize].holds, expected);
        assert_eq!(p.series[Context::Paused as usize].holds, st.holds);
        assert_eq!(p.series[Context::Playing as usize].callbacks, 0);
        assert_eq!(p.callback_builds, 1);
    }
}
