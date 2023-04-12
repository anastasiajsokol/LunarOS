use lazy_static::lazy_static;
use volatile::Volatile;
use spin::Mutex;
use core::fmt;

#[allow(dead_code)]
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
    pub fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ScreenChar {
    ascii_character: u8,
    color_code: ColorCode,
}

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

#[repr(transparent)]
struct Buffer {
    chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

pub struct Writer {
    column_position: usize,
    row_position: usize,
    color_code: ColorCode,
    buffer: &'static mut Buffer,
}

impl Writer {
    pub fn cwrite(&mut self, character: u8){
        match character {
            b'\n' => self.newline(),
            b'\t' => {
                self.column_position += 4;
                if self.column_position >= BUFFER_WIDTH {
                    self.newline();
                }
            },
            character => {
                if self.column_position >= BUFFER_WIDTH {
                    self.newline();
                }

                let row = self.row_position;
                let col = self.column_position;

                let color_code = self.color_code;
                self.buffer.chars[row][col].write(ScreenChar {
                    ascii_character: character,
                    color_code,
                });
                self.column_position += 1;
            }
        }
    }

    pub fn swrite(&mut self, string: &str){
        for character in string.bytes() {
            match character {
                // printable ASCII byte or newline
                0x20..=0x7e | b'\n' | b'\t' => self.cwrite(character),
                
                // not part of printable ASCII range
                _ => self.cwrite(0xfe),
            }
        }
    }

    pub fn newline(&mut self){
        if self.row_position != BUFFER_HEIGHT - 1 {
            self.row_position += 1;
        } else {
            
            for row in 1..BUFFER_HEIGHT {
                for col in 0..BUFFER_WIDTH {
                    let character = self.buffer.chars[row][col].read();
                    self.buffer.chars[row - 1][col].write(character);
                }
            }
            
            let blank = ScreenChar {
                ascii_character: b' ',
                color_code: self.color_code,
            };

            for col in 0..BUFFER_WIDTH {
                self.buffer.chars[BUFFER_HEIGHT - 1][col].write(blank);
            }
        }
        self.column_position = 0;
    }

    pub fn set_color(&mut self, color: ColorCode){
        self.color_code = color;
    }
}

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.swrite(s);
        Ok(())
    }
}

lazy_static! {
    pub static ref DEFAULT_COLOR: ColorCode = ColorCode::new(Color::White, Color::Black);

    pub static ref WRITER: Mutex<Writer> = Mutex::new( Writer {
        column_position: 0,
        row_position: 0,
        color_code: *DEFAULT_COLOR,
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    });
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
macro_rules! cprint {
    ($color:expr, $($arg:tt)*) => ($crate::vga::_cprint($color, format_args!($($arg)*)));
}

#[macro_export]
macro_rules! cprintln {
    () => ($crate::print!("\n"));
    ($color:expr, $($arg:tt)*) => ($crate::cprint!($color, "{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    WRITER.lock().write_fmt(args).unwrap();
}

#[doc(hidden)]
pub fn _cprint(color: ColorCode, args: fmt::Arguments) {
    use core::fmt::Write;
    WRITER.lock().set_color(color);
    WRITER.lock().write_fmt(args).unwrap();
    WRITER.lock().set_color(*DEFAULT_COLOR);
}