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

pub struct NumWorksKeypad;
impl Keypad for NumWorksKeypad {
    async fn wait_key(&mut self) -> delta_radix_hal::Key {
        // Wait forever!        
        loop {
            if input::keyboard_scan().is_pressed(Key::One) {
                eadk::timing::msleep(100);
                return delta_radix_hal::Key::Digit(1)
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

