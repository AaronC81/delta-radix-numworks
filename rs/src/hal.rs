use alloc::string::ToString;
use delta_radix_hal::{Hal, Time, Keypad, Display};

use crate::eadk::{self, display::{self, Rect, Color, Point, Font}, input::{self, Key}, timing};

pub struct NumWorksDisplay {
    pub x: u8,
    pub y: u8,
}
impl Display for NumWorksDisplay {
    fn init(&mut self) {}
    fn clear(&mut self) {
        display::fill(
            Rect { x: 0, y: 0, width: 320, height: 240 },
            Color::WHITE,
        );
    }

    fn print_char(&mut self, c: char) {
        display::write_string(
            &c.to_string(),
            Point { x: 15 * (self.x as u16), y: 15 * (self.y as u16) },
            Font::Large,
            Color::BLACK,
            Color::WHITE,
        );

        self.x += 1;
    }

    fn set_position(&mut self, x: u8, y: u8) {
        self.x = x;
        self.y = y;
    }

    fn get_position(&mut self) -> (u8, u8) {
        (self.x, self.y)
    }
}

pub struct NumWorksKeypad {
    pub pressed_key: Option<delta_radix_hal::Key>,
}
impl NumWorksKeypad {
    fn poll_mapped_key() -> Option<delta_radix_hal::Key> {
        let scan = input::keyboard_scan();

        let mapping = [
            (Key::Zero,  delta_radix_hal::Key::Digit(0)),
            (Key::One,   delta_radix_hal::Key::Digit(1)),
            (Key::Two,   delta_radix_hal::Key::Digit(2)),
            (Key::Three, delta_radix_hal::Key::Digit(3)),
            (Key::Four,  delta_radix_hal::Key::Digit(4)),
            (Key::Five,  delta_radix_hal::Key::Digit(5)),
            (Key::Six,   delta_radix_hal::Key::Digit(6)),
            (Key::Seven, delta_radix_hal::Key::Digit(7)),
            (Key::Eight, delta_radix_hal::Key::Digit(8)),
            (Key::Nine,  delta_radix_hal::Key::Digit(9)),

            // These are the letter keys for A-F
            (Key::Exp,       delta_radix_hal::Key::Digit(0xa)),
            (Key::Ln,        delta_radix_hal::Key::Digit(0xb)),
            (Key::Log,       delta_radix_hal::Key::Digit(0xc)),
            (Key::Imaginary, delta_radix_hal::Key::Digit(0xd)),
            (Key::Comma,     delta_radix_hal::Key::Digit(0xe)),
            (Key::Power,     delta_radix_hal::Key::Digit(0xf)),

            (Key::Shift, delta_radix_hal::Key::Shift),
            (Key::Exe,   delta_radix_hal::Key::Exe),

            (Key::Plus,             delta_radix_hal::Key::Add),
            (Key::Minus,            delta_radix_hal::Key::Subtract),
            (Key::Multiplication,   delta_radix_hal::Key::Multiply),
            (Key::Division,         delta_radix_hal::Key::Divide),

            (Key::Toolbox, delta_radix_hal::Key::Menu),
            (Key::Var,     delta_radix_hal::Key::Variable),

            (Key::Left,      delta_radix_hal::Key::Left),
            (Key::Right,     delta_radix_hal::Key::Right),
            (Key::Backspace, delta_radix_hal::Key::Delete),

            (Key::Ee,  delta_radix_hal::Key::HexBase),
            (Key::Dot, delta_radix_hal::Key::BinaryBase),
            (Key::Ans, delta_radix_hal::Key::FormatSelect),
        ];

        for (from, to) in mapping {
            if scan.is_pressed(from) {
                return Some(to);
            }
        }
        None
    }
}
impl Keypad for NumWorksKeypad {
    async fn wait_key(&mut self) -> delta_radix_hal::Key {
        loop {
            if let Some(key) = NumWorksKeypad::poll_mapped_key() {
                if self.pressed_key.is_some() && key == self.pressed_key.unwrap() {
                    // Debounce, do nothing
                } else {
                    self.pressed_key = Some(key);
                    return key;
                }
            } else {
                self.pressed_key = None;
            }
        }
    }
}

pub struct NumWorksTime;
impl Time for NumWorksTime {
    async fn sleep(&mut self, dur: core::time::Duration) {
        timing::msleep(dur.as_millis() as u32);
    }
}

pub struct NumWorksHal {
    pub display: NumWorksDisplay,
    pub keypad: NumWorksKeypad,
    pub time: NumWorksTime,
}

impl Hal for NumWorksHal {
    type D = NumWorksDisplay;
    type K = NumWorksKeypad;
    type T = NumWorksTime;

    fn display(&self) -> &Self::D { &self.display }
    fn display_mut(&mut self) -> &mut Self::D { &mut self.display }

    fn keypad(&self) -> &Self::K { &self.keypad }
    fn keypad_mut(&mut self) -> &mut Self::K { &mut self.keypad }

    fn time(&self) -> &Self::T { &self.time }
    fn time_mut(&mut self) -> &mut Self::T { &mut self.time }

    fn common_mut(&mut self) -> (&mut Self::D, &mut Self::K, &mut Self::T) {
        (&mut self.display, &mut self.keypad, &mut self.time)
    }

    async fn enter_bootloader(&mut self) {
        // Not supported
    }
}

