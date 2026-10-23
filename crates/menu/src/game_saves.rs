//! Controller-friendly selection of numbered, raw cartridge saves.
use super::{canvas::Canvas, rows_of, Layout, Menu, Page, Row};

#[derive(Clone)]
pub struct GameSaveSlot {
    pub name: String,
    pub status: String,
    pub occupied: bool,
    pub available: bool,
    pub current: bool,
}

pub(super) fn layout(n: usize, w: usize, h: usize) -> Layout {
    let scale = (w as f32 / 560.).min(h as f32 / 480.).max(0.05);
    let pad = (14. * scale).round().max(2.) as usize;
    let line_h = (38. * scale).round().max(3.) as usize;
    let panel_w = ((500. * scale) as usize).min(w.saturating_sub(pad * 2));
    let header = (115. * scale) as usize;
    let panel_h = (pad * 2 + header + n * line_h + (75. * scale) as usize).min(h);
    let panel_x = w.saturating_sub(panel_w) / 2;
    let panel_y = h.saturating_sub(panel_h) / 2;
    Layout {
        panel_x,
        panel_y,
        panel_w,
        panel_h,
        pad,
        line_h,
        rows_y: panel_y + pad + header,
        visible_rows: n,
    }
}

pub(super) fn draw(menu: &Menu, px: &mut [[u8; 4]], w: usize, h: usize) {
    for p in px.iter_mut().take(w * h) {
        for c in &mut p[..3] {
            *c = (*c as u16 * 5 / 16) as u8;
        }
    }
    let rows = rows_of(menu.page);
    let l = layout(rows.len(), w, h);
    let scale = l.line_h as f32 / 38.;
    let mut c = Canvas::new(px, w, h);
    let ink = [35, 24, 16];
    let x = (l.panel_x + l.pad) as f32;
    let width = l.panel_w.saturating_sub(l.pad * 2) as f32;
    c.rect(
        l.panel_x as f32,
        l.panel_y as f32,
        l.panel_w as f32,
        l.panel_h as f32,
        7. * scale,
        [251, 244, 226],
        [235, 220, 185],
        [184, 146, 65],
    );
    let title = match menu.page {
        Page::SwitchGameSave => "SWITCH GAME SAVE?",
        Page::ReplaceGameSave => "REPLACE GAME SAVE?",
        _ => "GAME SAVES",
    };
    let Some(slot) = menu.game_save_slots.get(menu.game_save_slot) else {
        c.finish(px);
        return;
    };
    let top = (l.panel_y + l.pad) as f32;
    c.text(x, top, title, ink, 18. * scale, true, width);
    c.text(
        x,
        top + 29. * scale,
        &format!(
            "Slot {:02} / 99 - {}{}",
            menu.game_save_slot + 1,
            slot.status,
            if slot.current { " (active)" } else { "" }
        ),
        ink,
        14. * scale,
        false,
        width,
    );
    c.text(
        x,
        top + 54. * scale,
        &slot.name,
        ink,
        14. * scale,
        true,
        width,
    );
    c.text(
        x,
        top + 79. * scale,
        "Copies contain progress saved inside the game.",
        ink,
        12. * scale,
        false,
        width,
    );
    for (i, row) in rows.iter().enumerate() {
        let y = (l.rows_y + i * l.line_h) as f32;
        let selected = menu.selected == i;
        c.rect(
            x,
            y,
            width,
            l.line_h as f32 - 3. * scale,
            4. * scale,
            if selected {
                [84, 49, 17]
            } else {
                [254, 252, 245]
            },
            if selected {
                [35, 20, 9]
            } else {
                [235, 220, 185]
            },
            [184, 146, 65],
        );
        let label = match row {
            Row::GameSaveSlot => format!("<   Slot {:02} / 99   >", menu.game_save_slot + 1),
            Row::Back if menu.page != Page::GameSaves => "Cancel".into(),
            _ => row.label().into(),
        };
        c.text(
            x + 10. * scale,
            y + 8. * scale,
            &label,
            if selected { [255, 244, 211] } else { ink },
            16. * scale,
            true,
            width - 20. * scale,
        );
    }
    let footer = (l.rows_y + rows.len() * l.line_h) as f32 + 8. * scale;
    let message = match menu.page {
        Page::SwitchGameSave => "Save current progress, then restart on this slot?",
        Page::ReplaceGameSave => "Replace this file with the current game's save?",
        _ if !menu.game_save_message.is_empty() => &menu.game_save_message,
        _ => "Left/Right: slot   Confirm: select   Back: return",
    };
    c.text(x, footer, message, ink, 12. * scale, false, width);
    c.text(
        x,
        footer + 22. * scale,
        "Empty slots start a new game. Use the title's Load option.",
        ink,
        12. * scale,
        false,
        width,
    );
    c.finish(px);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn game_save_picker_draws_and_mouse_actions_match_at_supported_sizes() {
        for (w, h) in [(240, 160), (640, 480), (1280, 720), (3840, 2160)] {
            let mut menu = Menu::new(true, true);
            menu.set_game_saves(
                (1..=99)
                    .map(|slot| GameSaveSlot {
                        name: format!("ffta-slot-{slot:02}.sav"),
                        status: "Saved".into(),
                        occupied: true,
                        available: true,
                        current: slot == 99,
                    })
                    .collect(),
            );
            menu.open(Page::GameSaves);
            assert_eq!(menu.game_save_slot, 98);
            let l = layout(4, w, h);
            assert!(l.rows_y + 4 * l.line_h <= h);
            for i in 0..4 {
                assert_eq!(
                    menu.row_at(
                        l.panel_x + l.pad + 1,
                        l.rows_y + i * l.line_h + l.line_h / 2,
                        w,
                        h
                    ),
                    Some(i)
                );
            }
            let mut pixels = vec![[0, 0, 0, 255]; w * h];
            draw(&menu, &mut pixels, w, h);
            assert!(pixels.iter().any(|p| *p != [0, 0, 0, 255]));
        }
    }
}
