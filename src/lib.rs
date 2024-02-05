#![no_std]
#![feature(prelude_2024)]

pub mod vga_buffer;



pub mod prelude {
    pub use core::fmt::Write;
    pub use crate::vga_buffer::{VGA_WRITER, BUFFER_HEIGHT, BUFFER_WIDTH};
    pub use crate::print;
    pub use crate::println;
}