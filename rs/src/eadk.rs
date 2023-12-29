//! Links against the C-provided `eadk_bridge`.
//!
//! Targets the `eadk_bridge` for `nwlink` 0.0.17.

extern "C" {
    // Keyboard and Events
    fn eadk_bridge__keyboard_scan() -> u64;

    // Display
    fn eadk_bridge__display_push_rect(x: u16, y: u16, width: u16, height: u16, pixels: *const u16);
    fn eadk_bridge__display_push_rect_uniform(x: u16, y: u16, width: u16, height: u16, color: u16);
    fn eadk_bridge__display_wait_for_vblank() -> u8;
    fn eadk_bridge__display_draw_string(str: *const u8, x: u16, y: u16, large_font: u8, text_color: u16, bg_color: u16);
    
    // Timing
    fn eadk_bridge__timing_usleep(us: u32);
    fn eadk_bridge__timing_msleep(ms: u32);
    fn eadk_bridge__timing_millis() -> u64;
}

pub mod input {
    /// The keyboard's keys.
    #[repr(u64)]
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub enum Key {
        Left = 0,
        Up = 1,
        Down = 2,
        Right = 3,
        Ok = 4,
        Back = 5,
        Home = 6,
        OnOff = 8,
        Shift = 12,
        Alpha = 13,
        Xnt = 14,
        Var = 15,
        Toolbox = 16,
        Backspace = 17,
        Exp = 18,
        Ln = 19,
        Log = 20,
        Imaginary = 21,
        Comma = 22,
        Power = 23,
        Sine = 24,
        Cosine = 25,
        Tangent = 26,
        Pi = 27,
        Sqrt = 28,
        Square = 29,
        Seven = 30,
        Eight = 31,
        Nine = 32,
        LeftParenthesis = 33,
        RightParenthesis = 34,
        Four = 36,
        Five = 37,
        Six = 38,
        Multiplication = 39,
        Division = 40,
        One = 42,
        Two = 43,
        Three = 44,
        Plus = 45,
        Minus = 46,
        Zero = 48,
        Dot = 49,
        Ee = 50,
        Ans = 51,
        Exe = 52      
    }

    /// Holds a snapshot of which keys were pressed.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct KeyboardScan(u64);

    impl KeyboardScan {
        /// Determines whether the given [Key] was pressed during this scan.
        pub fn is_pressed(&self, key: Key) -> bool {
            self.0 & (1 << (key as u64)) > 0
        }
    }

    /// Scans the keyboard and returns its state.
    pub fn keyboard_scan() -> KeyboardScan {
        unsafe { KeyboardScan(super::eadk_bridge__keyboard_scan()) }
    }
}

pub mod display {
    use alloc::borrow::ToOwned;

    /// A rectangle on the display.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct Rect {
        pub x: u16,
        pub y: u16,
        pub width: u16,
        pub height: u16,
    }
    impl Rect {
        pub const SCREEN: Rect = Rect { x: 0, y: 0, width: 320, height: 240 };
    }
    
    /// A collection of sized pixel data which can be drawn to the screen as an image.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct Bitmap<'d> {
        pub width: u16,
        pub height: u16,

        // Expected to have size (width * height)
        pub data: &'d [Color],
    }

    impl<'d> Bitmap<'d> {
        /// Checks that the size of `data` is equal to `width * height`, panicking if this is not
        /// the case.
        pub fn validate(&self) {
            if self.data.len() != (self.width * self.height) as usize {
                panic!(
                    "bitmap data size ({}) does not match dimensions ({} x {})",
                    self.data.len(), self.width, self.height
                )
            }
        }
    }

    /// A point on the display.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct Point {
        pub x: u16,
        pub y: u16,
    }

    /// An RGB565 color.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    #[repr(C)]
    pub struct Color(pub u16);
    impl Color {
        pub const WHITE: Color = Color(0xFFFF);
        pub const BLACK: Color = Color(0);
        pub const RED:   Color = Color(0xF800);
        pub const GREEN: Color = Color(0x07E0);
        pub const BLUE:  Color = Color(0x001F);
    }

    /// The fonts available for drawing text.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub enum Font {
        Small,
        Large,
    }

    impl Font {
        fn to_large_font_arg(self) -> u8 {
            if self == Font::Large { 1 } else { 0 }
        }
    }

    /// Fills a [Rect] with a [Color].
    pub fn fill(rect: Rect, color: Color) {
        unsafe {
            super::eadk_bridge__display_push_rect_uniform(rect.x, rect.y, rect.width, rect.height, color.0)
        }
    }

    /// Draws a [Bitmap] to a given [Point] on the display.
    pub fn draw_bitmap(pt: Point, bitmap: Bitmap) {
        bitmap.validate();
        unsafe {
            super::eadk_bridge__display_push_rect(pt.x, pt.y, bitmap.width, bitmap.height, bitmap.data.as_ptr() as *const u16)
        }
    }

    /// Writes a string to the display.
    pub fn write_string(str: &str, pt: Point, font: Font, text_color: Color, bg_color: Color) {
        // Allocate a buffer for the string, and push a null-terminator
        let mut str_owned = str.to_owned();
        str_owned.push('\0');

        // Write
        unsafe {
            write_string_null_terminated(str_owned.as_bytes(), pt, font, text_color, bg_color)
        }
    }

    /// Writes a null-terminated sequence of ASCII bytes to the display.
    /// 
    /// It is the caller's responsibility to ensure that the given slice contains a NUL (0).
    /// Undefined Behaviour will occur if it doesn't.
    pub unsafe fn write_string_null_terminated(str: &[u8], pt: Point, font: Font, text_color: Color, bg_color: Color) {
        super::eadk_bridge__display_draw_string(str.as_ptr(), pt.x, pt.y, font.to_large_font_arg(), text_color.0, bg_color.0)
    }

    /// Waits for display VBLANK.
    /// 
    /// Honestly, I don't know what this does, but I'll bridge it anyway in case it's important for
    /// something.
    pub fn wait_for_vblank() -> bool {
        unsafe {
            super::eadk_bridge__display_wait_for_vblank() > 0
        }
    }
}

pub mod timing {
    /// Sleep for the given number of microseconds.
    pub fn usleep(us: u32) {
        unsafe {
            super::eadk_bridge__timing_usleep(us);
        }
    }

    // Sleep for the given number of milliseconds.
    pub fn msleep(ms: u32) {
        unsafe {
            super::eadk_bridge__timing_msleep(ms);
        }
    }

    /// Gets a millisecond counter, which can be used to implement custom timing-related operations.
    pub fn millis() -> u64 {
        unsafe {
            super::eadk_bridge__timing_millis()
        }
    }
}
