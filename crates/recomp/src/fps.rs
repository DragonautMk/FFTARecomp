//! Wall-clock rates: completed guest frames and successful host presentations.
use std::sync::OnceLock;
use std::time::{Duration, Instant};

pub struct Meter {
    mode: String,
    start: Instant,
    internal: u64,
    external: u64,
    pixels: Vec<[u8; 4]>,
}

impl Meter {
    pub fn new(now: Instant) -> Self {
        Self {
            mode: "off".into(),
            start: now,
            internal: 0,
            external: 0,
            pixels: Vec::new(),
        }
    }

    pub fn reset(&mut self, now: Instant) {
        self.start = now;
        self.internal = 0;
        self.external = 0;
        if self.mode != "off" {
            menu::fps::render(&mut self.pixels, &self.mode, None);
        }
    }

    pub fn configure(&mut self, mode: &str, now: Instant) {
        if self.mode != mode {
            self.mode = mode.into();
            self.reset(now);
        }
    }

    pub fn emulated(&mut self, frames: u64) {
        if self.mode != "off" {
            self.internal += frames;
        }
    }

    pub fn presented(&mut self, now: Instant) {
        if self.mode == "off" {
            return;
        }
        self.external += 1;
        let elapsed = now.saturating_duration_since(self.start);
        if elapsed >= Duration::from_secs(1) {
            let rates = (
                self.internal as f64 / elapsed.as_secs_f64(),
                self.external as f64 / elapsed.as_secs_f64(),
            );
            menu::fps::render(&mut self.pixels, &self.mode, Some(rates));
            self.start = now;
            self.internal = 0;
            self.external = 0;
        }
    }

    pub fn frame(&self, window: (u32, u32), dpi: f64) -> Option<screen::ui_layer::Frame<'_>> {
        if self.mode == "off" || self.pixels.is_empty() {
            return None;
        }
        // Keep the HUD legible and independent of the game's zoom/aspect/LUT.
        let scale = (2. * dpi as f32).round().clamp(1., 4.);
        let margin = 4. * scale;
        static COLORS: OnceLock<Vec<[u8; 4]>> = OnceLock::new();
        let colors = COLORS.get_or_init(|| {
            let mut colors = vec![[255, 255, 255, 255]; 32768];
            colors[0] = [16, 20, 24, 255];
            colors
        });
        Some(screen::ui_layer::Frame {
            pixels: &self.pixels,
            raw_world: &[0],
            world_dims: (1, 1),
            lut: colors,
            layout: screen::ui_layer::Layout {
                rect: [margin, margin, 480. * scale, 160. * scale],
                clip: [
                    margin,
                    margin,
                    (menu::fps::WIDTH as f32 * scale).min((window.0 as f32 - margin).max(0.)),
                    (menu::fps::HEIGHT as f32 * scale).min((window.1 as f32 - margin).max(0.)),
                ],
            },
            world_view: [0., 0., 1., 1.],
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counts_fast_forward_frames_separately_and_resets_after_pause() {
        let start = Instant::now();
        let mut meter = Meter::new(start);
        assert!(meter.frame((1280, 720), 1.).is_none());
        meter.configure("both", start);
        for i in 1..=60 {
            meter.emulated(4);
            meter.presented(start + Duration::from_secs_f64(i as f64 / 60.));
        }
        let mut expected = Vec::new();
        menu::fps::render(&mut expected, "both", Some((240., 60.)));
        assert_eq!(meter.pixels, expected);
        // A paused interval contributes no frames or time to the next sample.
        let resumed = start + Duration::from_secs(30);
        meter.reset(resumed);
        meter.emulated(30);
        meter.presented(resumed + Duration::from_secs(1));
        menu::fps::render(&mut expected, "both", Some((30., 1.)));
        assert_eq!(meter.pixels, expected);
        meter.configure("off", resumed);
        assert!(meter.frame((1280, 720), 1.).is_none());
    }

    #[test]
    fn overlay_is_clipped_legible_and_independent_of_game_zoom() {
        let start = Instant::now();
        let mut meter = Meter::new(start);
        meter.configure("both", start);
        meter.emulated(60);
        meter.external = 59;
        meter.presented(start + Duration::from_secs(1));
        let frame = meter.frame((640, 160), 1.).unwrap();
        assert!(frame.sample(0., 0.).is_none());
        assert_eq!(frame.sample(8.5, 8.5), Some([16, 20, 24, 255]));
        assert!(frame.sample(249., 8.).is_none());
        let colors: Vec<_> = (0..52)
            .flat_map(|y| {
                (0..240).filter_map(move |x| frame.sample(x as f32 + 8.5, y as f32 + 8.5))
            })
            .collect();
        assert!(colors.contains(&[255, 255, 255, 255]));
        // Optional visual artifact for inspecting the actual CPU sampler.
        if let Ok(path) = std::env::var("FFTA_FPS_PREVIEW") {
            let mut ppm = b"P6\n640 160\n255\n".to_vec();
            for y in 0..160 {
                for x in 0..640 {
                    let color = frame
                        .sample(x as f32 + 0.5, y as f32 + 0.5)
                        .unwrap_or([70, 90, 110, 255]);
                    ppm.extend_from_slice(&color[..3]);
                }
            }
            std::fs::write(path, ppm).unwrap();
        }
    }
}
