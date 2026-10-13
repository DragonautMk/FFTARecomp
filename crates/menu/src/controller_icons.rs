//! Own vector controller symbols, matching the HTML reference's silhouettes.
use super::canvas::Canvas;
const EDGE: [u8; 3] = [26, 18, 9];
const WHITE: [u8; 3] = [255, 255, 255];
const GREY: [u8; 3] = [75, 79, 88];
pub(super) fn supported(v: &str) -> bool {
    matches!(
        v,
        "A" | "B"
            | "X"
            | "Y"
            | "LB"
            | "RB"
            | "LT"
            | "RT"
            | "BACK"
            | "START"
            | "GUIDE"
            | "LS CLICK"
            | "RS CLICK"
            | "DPAD UP"
            | "DPAD DOWN"
            | "DPAD LEFT"
            | "DPAD RIGHT"
            | "LS UP"
            | "LS DOWN"
            | "LS LEFT"
            | "LS RIGHT"
            | "RS UP"
            | "RS DOWN"
            | "RS LEFT"
            | "RS RIGHT"
    ) || v.starts_with("BUTTON ")
}
pub(super) fn draw(c: &mut Canvas, v: &str, x: f32, y: f32, size: f32) {
    let s = size / 32.;
    let cx = x + 16. * s;
    let cy = y + 16. * s;
    let mut label = None;
    match v {
        "A" | "B" | "X" | "Y" => {
            let color = match v {
                "A" => [76, 175, 80],
                "B" => [224, 69, 58],
                "X" => [47, 111, 208],
                _ => [242, 193, 46],
            };
            c.circle(cx, cy, 13. * s, color, EDGE);
            c.text(
                x + 10. * s,
                y + 7. * s,
                v,
                if v == "Y" { EDGE } else { WHITE },
                20. * s,
                false,
                22. * s,
            );
        }
        "LB" | "RB" => {
            c.rect(
                x - 4. * s,
                y + 8. * s,
                40. * s,
                18. * s,
                8. * s,
                GREY,
                [45, 48, 55],
                EDGE,
            );
            label = Some((v, 9., 11., 14.));
        }
        "LT" | "RT" => {
            let mut p = tiny_skia::PathBuilder::new();
            let flip = v == "RT";
            let tx = |n: f32| x + if flip { 32. - n } else { n } * s;
            p.move_to(tx(3.), y + 27. * s);
            p.line_to(tx(3.), y + 13. * s);
            p.quad_to(tx(3.), y + 3. * s, tx(15.), y + 3. * s);
            p.line_to(tx(29.), y + 3. * s);
            p.line_to(tx(29.), y + 28. * s);
            p.close();
            let path = p.finish().unwrap();
            c.path(path.clone(), [58, 61, 69], 0.);
            c.path(path, EDGE, 1.5 * s);
            label = Some((v, 9., 11., 14.));
        }
        "BACK" | "START" | "GUIDE" => {
            c.circle(
                cx,
                cy,
                13. * s,
                if v == "GUIDE" { [46, 139, 62] } else { GREY },
                EDGE,
            );
            if v == "START" {
                for yy in [11., 16., 21.] {
                    c.line(
                        x + 10. * s,
                        y + yy * s,
                        x + 22. * s,
                        y + yy * s,
                        WHITE,
                        2. * s,
                    );
                }
            } else if v == "BACK" {
                for (xx, yy) in [(9., 10.), (13., 14.)] {
                    c.rect(
                        x + xx * s,
                        y + yy * s,
                        10. * s,
                        8. * s,
                        s,
                        GREY,
                        GREY,
                        WHITE,
                    );
                }
            } else {
                c.line(
                    x + 10. * s,
                    y + 10. * s,
                    x + 22. * s,
                    y + 22. * s,
                    WHITE,
                    2.5 * s,
                );
                c.line(
                    x + 22. * s,
                    y + 10. * s,
                    x + 10. * s,
                    y + 22. * s,
                    WHITE,
                    2.5 * s,
                );
            }
        }
        v if v.starts_with("DPAD ") => {
            let mut p = tiny_skia::PathBuilder::new();
            for (i, (xx, yy)) in [
                (12., 3.),
                (20., 3.),
                (20., 12.),
                (29., 12.),
                (29., 20.),
                (20., 20.),
                (20., 29.),
                (12., 29.),
                (12., 20.),
                (3., 20.),
                (3., 12.),
                (12., 12.),
            ]
            .iter()
            .enumerate()
            {
                if i == 0 {
                    p.move_to(x + xx * s, y + yy * s);
                } else {
                    p.line_to(x + xx * s, y + yy * s);
                }
            }
            p.close();
            let path = p.finish().unwrap();
            c.path(path.clone(), GREY, 0.);
            let (a, b, w, h) = match v {
                "DPAD UP" => (12., 3., 8., 9.),
                "DPAD DOWN" => (12., 20., 8., 9.),
                "DPAD LEFT" => (3., 12., 9., 8.),
                _ => (20., 12., 9., 8.),
            };
            c.rect(
                x + a * s,
                y + b * s,
                w * s,
                h * s,
                0.,
                [255, 184, 31],
                [255, 184, 31],
                [255, 184, 31],
            );
            c.path(path, EDGE, 1.5 * s);
        }
        v if v.starts_with("LS ") || v.starts_with("RS ") => {
            c.circle(cx, cy, 13. * s, [43, 46, 53], EDGE);
            c.circle(cx, cy, 9. * s, [91, 96, 107], [170, 176, 187]);
            label = Some((if v.starts_with("LS") { "L3" } else { "R3" }, 9., 10., 14.));
            if !v.ends_with("CLICK") {
                let (a, b) = if v.ends_with("UP") {
                    (16., 2.)
                } else if v.ends_with("DOWN") {
                    (16., 30.)
                } else if v.ends_with("LEFT") {
                    (2., 16.)
                } else {
                    (30., 16.)
                };
                c.circle(x + a * s, y + b * s, 3. * s, [255, 184, 31], EDGE);
            }
        }
        _ => {
            c.circle(cx, cy, 13. * s, GREY, EDGE);
            label = Some((v.strip_prefix("BUTTON ").unwrap_or("?"), 9., 10., 14.));
        }
    }
    if let Some((t, xx, yy, fs)) = label {
        c.text(x + xx * s, y + yy * s, t, WHITE, fs * s, false, 28. * s);
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn all_xinput_controls_have_icons() {
        for value in [
            "A",
            "B",
            "X",
            "Y",
            "LB",
            "RB",
            "LT",
            "RT",
            "BACK",
            "START",
            "GUIDE",
            "LS CLICK",
            "RS CLICK",
            "DPAD UP",
            "DPAD DOWN",
            "DPAD LEFT",
            "DPAD RIGHT",
            "LS UP",
            "LS DOWN",
            "LS LEFT",
            "LS RIGHT",
            "RS UP",
            "RS DOWN",
            "RS LEFT",
            "RS RIGHT",
        ] {
            assert!(supported(value));
            let pixels = vec![[0, 0, 0, 255]; 64 * 64];
            let mut c = Canvas::new(&pixels, 64, 64);
            draw(&mut c, value, 16., 16., 32.);
            assert!(
                c.pix.data().chunks_exact(4).any(|p| p[..3] != [0, 0, 0]),
                "{value}"
            );
        }
    }
}
