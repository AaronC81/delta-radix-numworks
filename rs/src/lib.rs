#![no_std]
#![feature(async_fn_in_trait)]

pub mod eadk;
pub mod mallocator;
pub mod executor;
pub mod hal;

use alloc::format;
use eadk::{display::{self, Rect, Color, Font, Point, Bitmap}, input::{self, Key}, timing};
use hal::*;
use mallocator::Mallocator;

extern crate alloc;

#[global_allocator]
static MALLOCATOR: Mallocator = Mallocator;

#[no_mangle]
pub extern "C" fn rs_main() {
    // Run Radix app
    let mut hal = NumWorksHal {
        display: NumWorksDisplay {
            x: 0,
            y: 0,
        },
        keypad: NumWorksKeypad {
            pressed_key: None,
        },
        time: NumWorksTime,
    };
    executor::execute(delta_radix_os::main(&mut hal))
}

#[panic_handler]
fn panic_handler(info: &core::panic::PanicInfo) -> ! {
    // Print heading
    display::fill(Rect::SCREEN, Color::WHITE);
    display::write_string("Panic!", Point { x: 0, y: 0 }, Font::Large, Color::WHITE, Color::RED);

    // Print panic message, chunked into lines
    let panic_message = format!("{info}");
    for (i, line) in panic_message.as_bytes().chunks(45).enumerate() {
        display::write_string(
            unsafe { core::str::from_utf8_unchecked(line) },
            Point { x: 0, y: 50 + (i as u16) * 20 }, Font::Small, Color::BLACK, Color::WHITE
        );
    }

    loop {
        input::keyboard_scan();
    }
}
