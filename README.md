# Sample Rust app for Epsilon

This is a sample EADK app for the [NumWorks calculator](https://www.numworks.com) which uses a C
"harness" to run a Rust application.

## Why?

NumWorks provide a [pure Rust sample application](https://github.com/numworks/epsilon-sample-app-rust)
themselves, but it had some problems for me:

- Calculator crash and reset when trying to call `eadk_display_draw_string`
- Intermittent crashes or unexpected behaviour when drawing to the screen

On the contrary, I had no issues using the [Epsilon C example app](https://github.com/numworks/epsilon-sample-app-c).

My best guesses are either: Rust's compiler toolchain making decisions about memory allocation which
NumWorks doesn't like; or an EABI incompatibility between Epsilon's EADK libraries and Rust.

So I decided to create a new sample application based on that C example, which compiles and links a
Rust static library. EADK APIs are usable through a provided `eadk_bridge`, which wraps EADK
functions with "simpler" signatures to reduce the chance of EABI problems.

```c
// Original EADK API
typedef struct {
  uint16_t x;
  uint16_t y;
  uint16_t width;
  uint16_t height;
} eadk_rect_t;
typedef uint16_t eadk_color_t;
eadk_display_push_rect_uniform(eadk_rect_t rect, eadk_color_t color);

// Wrapped `eadk_bridge` API
eadk_bridge__display_push_rect_uniform(uint16_t x, uint16_t y, uint16_t width, uint16_t height, uint16_t color);
```

## Features

Most of the important EADK APIs are broken out into nice, high-level Rust APIs. For example:

```rust
display::write_string(
  "Hello from Rust!",
  Point { x: 50, y: 50 },
  Font::Large,
  Color::BLACK,
  Color::WHITE,
);

timing::msleep(1000);
```

There is also a `GlobalAlloc` implementation, so you can use the `alloc` crate.

## Compatibility

This was developed/tested on:

- `nwlink` 0.0.17, running on Node 18.19.0
- Rust 1.71.0-nightly (c609da59d 2023-04-18)
- NumWorks N0120, running Epsilon 21.3.0

## Usage

The same dependencies apply as the [Epsilon C example app](https://github.com/numworks/epsilon-sample-app-c)
which this is derived from.

Once installed, you can build an NWA and run it on your calculator:

```shell
make build  # Build Rust and C parts into NWA
make check  # Ensure that NWA links successfully
make run    # Upload to calculator
```

## Layout

`src` includes C source code, including the `main` and `eadk_bridge` definitions. You probably don't
need to edit any of the C files in here, besides the app name in `main.c`.

`rs` contains the Rust code. In `lib.rs` you'll find the entry point, `rs_main`.
