#![no_std]

pub mod eadk;
pub mod mallocator;

use alloc::format;
use eadk::{display::{self, Rect, Color, Font, Point, Bitmap}, input::{self, Key}, timing};
use mallocator::Mallocator;

extern crate alloc;

#[global_allocator]
static MALLOCATOR: Mallocator = Mallocator;

#[no_mangle]
pub extern "C" fn rs_main() {
    // Rectangle test
    display::fill(Rect::SCREEN, Color::WHITE);
    display::fill(
        Rect { x: 10, y: 10, width: 10, height: 10 },
        Color(0xe426),
    );

    timing::msleep(1000);

    // Text test
    display::write_string(
        "Hello from Rust!",
        Point { x: 50, y: 50 },
        Font::Large,
        Color::BLACK,
        Color::WHITE,
    );

    timing::msleep(1000);

    // Bitmap writing test
    let mut bitmap_write_data = [Color::RED; 4 * 4];
    bitmap_write_data[0] = Color::BLUE;
    bitmap_write_data[(4 * 4) - 1] = Color::GREEN;
    display::draw_bitmap(
        Point { x: 2, y: 2 },
        Bitmap {
            width: 4,
            height: 4,
            data: &bitmap_write_data,
        },
    );

    // Input test
    loop {
        let colour =
            if input::keyboard_scan().is_pressed(Key::Exe) {
                Color::BLACK
            } else {
                Color::WHITE
            };
        display::fill(
            Rect { x: 50, y: 100, width: 20, height: 20 },
            colour,
        );
    }
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
