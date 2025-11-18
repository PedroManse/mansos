use core::cell::LazyCell;
use core::fmt;
use spin::Mutex;
use volatile::Volatile;

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum ForegroundColor {
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

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum BackgroundColor {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
}

#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Color(u8);

impl From<(ForegroundColor, BackgroundColor)> for Color {
    fn from((fg, bg): (ForegroundColor, BackgroundColor)) -> Self {
        Color((bg as u8) << 3 | fg as u8)
    }
}

impl Color {
    const fn new(fg: ForegroundColor, bg: BackgroundColor) -> Self {
        Color((bg as u8) << 3 | fg as u8)
    }
    fn blink(self) -> Color {
        Color(self.0 | 0b1000_0000)
    }
    fn no_blink(self) -> Color {
        Color(self.0 & (!0b1000_0000))
    }
    fn set_foreground(&mut self, fg: ForegroundColor) {
        self.0 = (self.0 & 0b1111_0000) | fg as u8;
    }
    fn set_background(&mut self, bg: BackgroundColor) {
        self.0 = (self.0 & 0b1000_1111) | (bg as u8) << 4;
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct ScreenChar {
    ascii: u8,
    color: Color,
}

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

#[repr(transparent)]
#[derive(Clone)]
pub struct Buffer {
    chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

pub struct ScreenWriter {
    pub column_position: usize,
    pub current_color: Color,
    pub buffer: LazyCell<&'static mut Buffer>,
}

impl ScreenWriter {
    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.column_position >= BUFFER_WIDTH {
                    self.new_line();
                }

                let row = BUFFER_HEIGHT - 1;
                let col = self.column_position;

                self.buffer.chars[row][col].write(ScreenChar {
                    ascii: byte,
                    color: self.current_color,
                });
                self.column_position += 1;
            }
        }
    }

    pub fn set_color(&mut self, color: Color) {
        self.current_color = color;
    }

    pub fn set_foreground(&mut self, fg: ForegroundColor) {
        self.current_color.set_foreground(fg);
    }

    pub fn set_background(&mut self, bg: BackgroundColor) {
        self.current_color.set_background(bg);
    }

    pub fn set_blink(&mut self) {
        self.current_color = self.current_color.blink();
    }

    pub fn set_no_blink(&mut self) {
        self.current_color = self.current_color.no_blink();
    }

    fn new_line(&mut self) {
        for row in 1..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                let character = self.buffer.chars[row][col].read();
                self.buffer.chars[row - 1][col].write(character);
            }
        }
        self.clear_row(BUFFER_HEIGHT - 1);
        self.column_position = 0;
    }

    fn clear_row(&mut self, row: usize) {
        for col in 0..BUFFER_WIDTH {
            self.buffer.chars[row][col].write(ScreenChar {
                ascii: b' ',
                color: self.current_color,
            });
        }
    }

    pub fn write_str(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                // printable ASCII byte or newline
                0x20..0x7f | b'\n' => self.write_byte(byte),
                // not part of printable ASCII range
                _ => self.write_byte(0xfe),
            }
        }
    }
}

impl fmt::Write for ScreenWriter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_str(s);
        Ok(())
    }
}

pub static VGA_WRITER: Mutex<ScreenWriter> = Mutex::new(ScreenWriter {
    column_position: 0,
    current_color: Color::new(ForegroundColor::White, BackgroundColor::Black),
    buffer: LazyCell::new(|| unsafe { &mut *(0xb8000 as *mut Buffer) }),
});
