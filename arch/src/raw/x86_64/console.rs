use core::fmt;

type VGABuffer = [[Char; Console::WIDTH]; Console::HEIGHT];

pub struct Console {
    row: usize,
    col: usize,
    buf: &'static mut VGABuffer,
}

impl Console {
    const VGA_BUFFER: *mut u8 = 0xb8000 as *mut u8;
    pub const WIDTH: usize = 80;
    pub const HEIGHT: usize = 25;

    pub fn new() -> Self {
        Self {
            row: 0,
            col: 0,
            buf: unsafe { &mut *(Self::VGA_BUFFER as *mut VGABuffer) }
        }
    }

    pub fn shift_rows_up(&mut self) {
        self.buf[0].fill(Char::NONE);
        self.buf.rotate_left(1);
    }

    pub fn next_row(&mut self) {
        if self.row == Self::HEIGHT - 1 {
            self.shift_rows_up();
        } else {
            self.row += 1;
        }
        self.col = 0;
    }

    pub fn next_col(&mut self) {
        if self.col == Self::WIDTH - 1 {
            self.next_row();
        } else {
            self.col += 1;
        }
    }

    pub fn write_char(&mut self, char: Char) {
        match char.char {
            b'\n' => self.next_row(),
            b'\r' => self.col = 0,
            // visible asci
            0x20..=0x7e => {
                self.buf[self.row][self.col].write(char);
                self.next_col();
            }
            _ => {
                self.buf[self.row][self.col].write(Char::UNPRINTABLE);
                self.next_col();
            }
        }
    }
}

impl fmt::Write for Console {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for byte in s.bytes() {
            self.write_char(Char::new(byte, ColorCode::WHITE_ON_BLACK));
        }

        Ok(())
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Char {
    char: u8,
    color: ColorCode,
}

impl Char {
    pub const NONE: Self = Self::new(0, ColorCode::BLACK);
    pub const UNPRINTABLE: Self = Self::new(0xfe, ColorCode::WHITE_ON_BLACK);

    pub const fn new(char: u8, color: ColorCode) -> Self {
        Self { char, color }
    }

    pub fn write(&mut self, val: Self) {
        unsafe { core::ptr::write_volatile(self, val); }
    }
}

#[derive(Clone, Copy)]
pub struct ColorCode(u8);

impl ColorCode {
    pub const BLACK: Self = Self::new(Color::Black, Color::Black);
    pub const WHITE_ON_BLACK: Self = Self::new(Color::White, Color::Black);

    pub const fn new(fg: Color, bg: Color) -> Self {
        Self((bg as u8) << 4 | (fg as u8))
    }
}

#[repr(u8)]
#[allow(dead_code)]
#[derive(Clone, Copy)]
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
