use volatile::Volatile;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color {
    Black = 0,
    Blue,
    Green,
    Cyan,
    Red,
    Magenta,
    Brown,
    LightGray,
    DarkGray,
    LightBlue,
    LightGreen,
    LightCyan,
    LightRed,
    Pink,
    Yellow,
    White,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct ColorCode(u8);

impl ColorCode {
    pub fn new(foreground: Color, background: Color) -> ColorCode {
        return ColorCode((background as u8) << 4 | (foreground as u8));
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct ScreenChar {
    ascii_character: u8,
    color_code: ColorCode,
}

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

#[repr(transparent)]
pub struct Buffer {
    chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

pub struct Writer {
    row_pos: usize,
    column_pos: usize,
    color_code: ColorCode,
    buffer: &'static mut Buffer
}

impl Writer {
    pub fn new() -> Writer {
        return Writer {
            row_pos: 0,
            column_pos: 0,
            color_code: ColorCode::new(Color::Yellow, Color::Black),
            buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
        };
    }

    pub fn set_color(&mut self, color_code: ColorCode) {
        self.color_code = color_code;
    }

    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.newline(),
            _ => {
                let mut byte = byte;
                if byte < 0x20 || byte > 0x7e {
                    byte = 0xfe; // ■
                }

                if self.column_pos >= BUFFER_WIDTH {
                    self.newline();
                }

                self.buffer.chars[self.row_pos][self.column_pos].write(ScreenChar {
                    ascii_character: byte,
                    color_code: self.color_code,
                });
                self.column_pos += 1;
            }
        }
    }

    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            self.write_byte(byte)
        }
    }

    fn newline(&mut self) {
        if self.column_pos < BUFFER_HEIGHT - 1 {
            self.row_pos += 1;
        } else {
            for row in 0..BUFFER_HEIGHT - 1 {
                for col in 0..BUFFER_WIDTH {
                    let character = self.buffer.chars[row + 1][col].read();
                    self.buffer.chars[row][col].write(character);
                }
            }

            for col in 0..BUFFER_WIDTH {
                self.buffer.chars[BUFFER_HEIGHT - 1][col].write(ScreenChar {
                    ascii_character: b' ',
                    color_code: self.color_code,
                });
            }
        }

        self.column_pos = 0;
    }
}