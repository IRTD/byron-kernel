use crate::cursor::Cursor2D;
use crate::*;

use core::fmt;
use lazy_static::lazy_static;
use spin::Mutex;
use volatile::Volatile;

pub const BUF_HEIGHT: usize = 25;
pub const BUF_WIDTH: usize = 80;

lazy_static! {
    pub static ref VGAFRAME: Mutex<VGAFrame> = Mutex::new(VGAFrame::new());
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Color {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct ColorCode(u8);

impl ColorCode {
    fn new(fg: Color, bg: Color) -> Self {
        ColorCode((bg as u8) << 4 | (fg as u8))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct ScreenChar {
    ascii: u8,
    color_code: ColorCode,
}

#[repr(transparent)]
struct VGABuffer {
    chars: [[Volatile<ScreenChar>; BUF_WIDTH]; BUF_HEIGHT],
}

impl VGABuffer {
    pub fn new() -> &'static mut Self {
        unsafe { &mut *(0xb8000 as *mut VGABuffer) }
    }
}

pub struct VGAFrame {
    buffer: &'static mut VGABuffer,
    color_code: ColorCode,
    err_color_code: ColorCode,
    cursor_color_code: ColorCode,
    cursor: Cursor2D,
}

impl VGAFrame {
    pub fn new() -> Self {
        let cursor = Cursor2D::new(BUF_WIDTH, BUF_HEIGHT - 1);
        VGAFrame {
            buffer: VGABuffer::new(),
            color_code: ColorCode::new(Color::Green, Color::Black),
            err_color_code: ColorCode::new(Color::Red, Color::Black),
            cursor_color_code: ColorCode::new(Color::Black, Color::Green),
            cursor,
        }
    }

    pub fn backspace(&mut self) {
        match self.cursor.x_with(|x| if x > 1 { x - 1 } else { x }) {
            Ok(_) => self.write_byte(b' '),
            Err(_e) => {}
        }
    }

    pub fn new_line(&mut self) {
        if self.cursor.next_line().is_none() {
            self.cursor.x_with(|_| 0).unwrap();
            self.shove_buffer();
            self.clear_line(self.cursor.height());
        }
    }

    pub fn shove_buffer(&mut self) {
        for y in 1..self.cursor.height() + 1 {
            for x in 0..self.cursor.width() {
                let c = self.buffer.chars[y][x].read();
                self.buffer.chars[y - 1][x].write(c);
            }
        }
    }

    pub fn clear_line(&mut self, line: usize) {
        if line > self.cursor.height() {
            return;
        }
        for c in &mut self.buffer.chars[line] {
            c.write(ScreenChar {
                ascii: b' ',
                color_code: self.color_code,
            })
        }
    }

    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                let x = self.cursor.x();
                let y = self.cursor.y();
                self.buffer.chars[y][x].write(ScreenChar {
                    ascii: byte,
                    color_code: self.color_code,
                });
            }
        }
    }

    pub fn write_screenchar(&mut self, c: ScreenChar) {
        let x = self.cursor.x();
        let y = self.cursor.y();
        self.buffer.chars[y][x].write(c);
    }

    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                _ => self.write_byte(0xfe),
            }
            if self.cursor.next().is_none() {
                self.new_line();
            }
            /*
            match byte {
                0x20..=0x7e | b'\n' => self.write_screenchar(ScreenChar {
                    ascii: b' ',
                    color_code: self.cursor_color_code,
                }),
                _ => self.write_byte(0xfe),
            }
            */
        }
    }
}

impl fmt::Write for VGAFrame {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

pub fn backspace() {
    use core::fmt::Write;
    use x86_64::instructions::interrupts;
    interrupts::without_interrupts(|| {
        let mut writer = VGAFRAME.lock();
        writer.backspace();
    })
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    use x86_64::instructions::interrupts;
    interrupts::without_interrupts(|| {
        VGAFRAME.lock().write_fmt(args).unwrap();
    });
}

#[doc(hidden)]
pub fn _err_print(args: fmt::Arguments) {
    use core::fmt::Write;
    use x86_64::instructions::interrupts;
    interrupts::without_interrupts(|| {
        let mut writer = VGAFRAME.lock();
        let prev = writer.color_code;
        writer.color_code = writer.err_color_code;
        writer.write_fmt(args).unwrap();
        writer.color_code = prev;
    })
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[macro_export]
macro_rules! eprint {
    ($($arg:tt)*) => ($crate::vga::_err_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! eprintln {
    () => ($crate::eprint!("\n"));
    ($($arg:tt)*) => ($crate::eprint!("{}\n", format_args!($($arg)*)));
}
