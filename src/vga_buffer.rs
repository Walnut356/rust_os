

use core::{fmt, prelude::rust_2024::*};
use volatile::Volatile;
use lazy_static::lazy_static;
use spin::Mutex;

pub const BUFFER_HEIGHT: usize = 25;
pub const BUFFER_WIDTH: usize = 80;

lazy_static!{
    pub static ref VGA_WRITER: Mutex<Writer> = Mutex::new(Writer::default());
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga_buffer::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    VGA_WRITER.lock().write_fmt(args).unwrap();
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Color {
    Black,
    Blue,
    Green,
    Cyan,
    Red,
    Magenta,
    Brown,
    LtGray,
    DkGray,
    LtBlue,
    LtGreen,
    LtCyan,
    LtRed,
    Pink,
    Yellow,
    White,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
struct ColorCode(u8);

impl ColorCode {
    fn new(fg: Color, bg: Color) -> Self {
        Self(((bg as u8) << 4) | (fg as u8))
    }
}

impl Default for ColorCode {
    fn default() -> Self {
        Self::new(Color::White, Color::Black)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ScreenChar {
    ascii: u8,
    color: ColorCode,
}

impl Default for ScreenChar {
    fn default() -> Self {
        Self { ascii: b' ', color: Default::default() }
    }
}

#[repr(transparent)]
struct Buffer {
    inner: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT]
}

impl Default for &mut Buffer {
    fn default() -> Self {
        unsafe { &mut *(0xb8000 as *mut Buffer) }
    }
}

#[derive(Default)]
pub struct Writer {
    col: usize,
    color: ColorCode,
    buffer: &'static mut Buffer,
}

impl Writer {
    pub fn write_byte(&mut self, b: u8) {
        match b {
            b'\n' => self.new_line(),
            byte => {
                if self.col >= BUFFER_WIDTH {
                    self.new_line();
                }

                self.buffer.inner[BUFFER_HEIGHT - 1][self.col].write(ScreenChar {
                    ascii: byte,
                    color: self.color,
                });

                self.col += 1;
            }
        }
    }

    pub fn write_string(&mut self, s: &str) {
        for b in s.bytes() {
            match b {
                0x20..=0x7E | b'\n' => self.write_byte(b),
                _ => self.write_byte(0xfe)
            }
        }
    }

    fn new_line(&mut self) {
        for row in 1..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                let c = self.buffer.inner[row][col].read();
                self.buffer.inner[row - 1][col].write(c);
            }
        }

        self.clear_row(BUFFER_HEIGHT - 1);
        self.col = 0;
    }

    fn clear_row(&mut self, row: usize) {
        for col in 0..BUFFER_WIDTH {
            self.buffer.inner[row][col].write(ScreenChar::default());
        }
    }
}

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

#[test_case]
fn test_println_simple() {
    println!("test_println_simple output");
}

#[test_case]
fn test_println_many() {
    for _ in 0..200 {
        println!("test_println_many output");
    }
}

#[test_case]
fn test_println_output() {
    let s = "Some test string that fits on a single line";
    println!("{}", s);
    for (i, c) in s.chars().enumerate() {
        let screen_char = VGA_WRITER.lock().buffer.inner[BUFFER_HEIGHT - 2][i].read();
        assert_eq!(char::from(screen_char.ascii), c);
    }
}