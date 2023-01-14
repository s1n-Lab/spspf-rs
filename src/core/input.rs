use psp::sys::{
    sceCtrlReadBufferPositive, sceCtrlSetSamplingCycle, sceCtrlSetSamplingMode, CtrlButtons,
    CtrlMode, SceCtrlData,
};

use crate::core::Vec2;

/// Reference to all available buttons in the PSP, to be used with `InputManager`
#[derive(Clone)]
pub enum Buttons {
    Select,
    Start,
    Up,
    Right,
    Down,
    Left,
    LTrigger,
    RTrigger,
    Triangle,
    Circle,
    Cross,
    Square,
    Home,
    Hold,
    MusicNote,
    Screen,
    VolumeUp,
    VolumeDown,
}

impl Buttons {
    fn as_ctrl_buttons(self) -> CtrlButtons {
        match self {
            Buttons::Select => CtrlButtons::SELECT,
            Buttons::Start => CtrlButtons::START,
            Buttons::Up => CtrlButtons::UP,
            Buttons::Right => CtrlButtons::RIGHT,
            Buttons::Down => CtrlButtons::DOWN,
            Buttons::Left => CtrlButtons::LEFT,
            Buttons::LTrigger => CtrlButtons::LTRIGGER,
            Buttons::RTrigger => CtrlButtons::RTRIGGER,
            Buttons::Triangle => CtrlButtons::TRIANGLE,
            Buttons::Circle => CtrlButtons::CIRCLE,
            Buttons::Cross => CtrlButtons::CROSS,
            Buttons::Square => CtrlButtons::SQUARE,
            Buttons::Home => CtrlButtons::HOME,
            Buttons::Hold => CtrlButtons::HOLD,
            Buttons::MusicNote => CtrlButtons::NOTE,
            Buttons::Screen => CtrlButtons::SCREEN,
            Buttons::VolumeUp => CtrlButtons::VOL_UP,
            Buttons::VolumeDown => CtrlButtons::VOL_DOWN,
        }
    }
}

/// Wrapper for the PSP input.
pub struct InputManager {
    pad_data: SceCtrlData,
    last_pad_data: SceCtrlData,
}

impl InputManager {
    /// The `InputManager` It must be instatiated once with `let input_manager = InputManager::new()`.
    pub fn new() -> InputManager {
        unsafe {
            sceCtrlSetSamplingCycle(0);
            sceCtrlSetSamplingMode(CtrlMode::Analog);
        }
        InputManager {
            pad_data: SceCtrlData::default(),
            last_pad_data: SceCtrlData::default(),
        }
    }

    /// To parse the current status of the PSP's buttons you must call the `::update()` function every tick.
    pub fn update(&mut self) {
        self.last_pad_data = self.pad_data.clone();
        unsafe {
            sceCtrlReadBufferPositive(&mut self.pad_data, 1);
        }
    }

    /// Returns a boolean stating if the specified button is down. Repeats if continually pressed.
    pub fn is_key_down(&mut self, button: Buttons) -> bool {
        self.pad_data.buttons.contains(button.as_ctrl_buttons())
    }

    /// Returns a boolean stating if the specified button is up. Repeats if continually released.
    pub fn is_key_up(&mut self, button: Buttons) -> bool {
        !self.pad_data.buttons.contains(button.as_ctrl_buttons())
    }

    /// Returns a boolean stating if the specified button is down and changed from the last check. Does not repeat if continually pressed.
    pub fn is_key_down_changed(&mut self, button: Buttons) -> bool {
        self.pad_data
            .buttons
            .contains(button.clone().as_ctrl_buttons())
            && !self
                .last_pad_data
                .buttons
                .contains(button.as_ctrl_buttons())
    }

    /// Returns a boolean stating if the specified button is up and changed from the last check. Does not repeat if continually pressed.
    pub fn is_key_up_changed(&mut self, button: Buttons) -> bool {
        !self
            .pad_data
            .buttons
            .contains(button.clone().as_ctrl_buttons())
            && self
                .last_pad_data
                .buttons
                .contains(button.as_ctrl_buttons())
    }

    /// Returns a `Vec2<i32>` containing the X and Y coordinates of the current analog position. Ranges from 0 (stopped or on the deadzone)
    /// to 5.
    pub fn get_analog_pos(&mut self) -> Vec2<i32> {
        let x = convert_analog_to_delta_with_sensitivity_deadzone(self.pad_data.lx);
        let y = convert_analog_to_delta_with_sensitivity_deadzone(self.pad_data.ly);
        Vec2::new(x, y)
    }
}


// Analog management by Glenn Hope's and Paul Sajna's implementation on [psp-paint-mode](https://github.com/overdrivenpotato/rust-psp/commit/ba16f08d16b39dcdefd022c9fb738d40545d6cc3)
const DEADZONE: i32 = 10;
const MAX_SPEED: i32 = 5;
const SPEED_MODIFIER: i32 = 127 / MAX_SPEED;

fn convert_analog_to_delta_with_sensitivity_deadzone(raw_val: u8) -> i32 {
    let delta_val = (raw_val as i32) - 127;

    // Zero out a "deadzone" around 0,0 to adjust for joysticks that sit off-center.
    let distance_without_deadzone = if delta_val > -DEADZONE && delta_val < DEADZONE {
        0
    } else if delta_val < -DEADZONE {
        delta_val + DEADZONE
    } else {
        delta_val - DEADZONE
    };

    distance_without_deadzone / SPEED_MODIFIER
}
