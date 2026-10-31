//! Presentation-only hints follow the last intentional input, not the game profile.
use super::*;

impl Menu {
    /// Returns whether hints changed. This never remaps or saves input bindings.
    pub fn note_input(&self, device: Device) -> bool {
        let device = if device == Device::Gamepad {
            Device::Gamepad
        } else {
            Device::Keyboard
        };
        self.hint_device.replace(Some(device)) != Some(device)
    }

    pub(super) fn hints_gamepad(&self) -> bool {
        self.hint_device.get() == Some(Device::Gamepad)
    }

    pub(super) fn hint_button(&self, token: &str, keyboard: &str) -> String {
        if self.hints_gamepad() {
            gamepad_label(token, self.controller_layout.get())
        } else {
            keyboard.into()
        }
    }

    pub(super) fn control_hints(&self) -> [String; 2] {
        if self.hints_gamepad() {
            let pad = |token| self.hint_button(token, "");
            [
                format!(
                    "D-pad · {} OK · {} indietro · {} focus · {} dettagli",
                    pad("South"),
                    pad("East"),
                    pad("West"),
                    pad("RightThumb")
                ),
                format!(
                    "VOL {}− / {}+ / {}+{} mute · FAST {}− / {}+ / {}+{} ON",
                    pad("LeftTrigger"),
                    pad("RightTrigger"),
                    pad("LeftTrigger"),
                    pad("RightTrigger"),
                    pad("LeftTrigger2"),
                    pad("RightTrigger2"),
                    pad("LeftTrigger2"),
                    pad("RightTrigger2")
                ),
            ]
        } else {
            [
                "Frecce · Invio OK · Esc indietro · Tab focus · F1 dettagli".into(),
                if self.debug_enabled(){"Volume / Fast: voce + Invio · Shift+Tab: vista Debug"}else{"Volume / Fast: scegli la voce + Invio"}.into(),
            ]
        }
    }
}

#[cfg(test)]
mod tests;
