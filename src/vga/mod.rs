
use core::fmt;
use lazy_static::lazy_static;
use spin::Mutex;

const BUFFER_WIDTH: usize = 80;
const BUFFER_HEIGHT: usize = 25;
const BUFFER_ADDR: u32 = 0xB8000;


#[repr(C)]
#[derive(Clone, Copy)]
struct Character {
    char: u8,
    color: u8
}

struct Cursor {
    x: usize,
    y: usize
}

impl Cursor {
    fn new_line(&mut self) {
        self.x = 0;
        self.y += 1;
    }
    
}

#[repr(transparent)]
struct Buffer {
    chars: [[Character; BUFFER_WIDTH]; BUFFER_HEIGHT]
}

struct VGA {
    buffer: &'static mut Buffer,
    cursor: Cursor 
}


pub struct Writer {
    vga: VGA
}

impl Writer {

    pub fn write_byte(&mut self, b: u8) {

        if self.vga.cursor.x >= BUFFER_WIDTH {
            self.new_line();
        }
        if b == b'\n' {
            self.new_line();
            return 
        } 

        self.vga.buffer.chars[self.vga.cursor.y][self.vga.cursor.x] = Character {char: b, color: 0x0f};
        self.vga.cursor.x += 1;


    }

    pub fn write_string(&mut self, s: &str) {
        for &b in s.as_bytes() {
            self.write_byte(b);
        }
    }

    fn scroll_buffer(&mut self) {
        for i in 1..BUFFER_HEIGHT {
            self.vga.buffer.chars[i-1] = self.vga.buffer.chars[i]

        }
        self.vga.cursor.y -= 1;
        self.vga.cursor.x = 0;
        self.clean_row(BUFFER_HEIGHT-1);
    }

    fn new_line(&mut self) {
        self.vga.cursor.new_line();
        if self.vga.cursor.y >= BUFFER_HEIGHT {
            self.scroll_buffer();
        }
    }

    fn clean_row(&mut self, index: usize) {
       self.vga.buffer.chars[index] = [Character {char: " ".as_bytes()[0], color: 0x00}; 80] 

    }

    fn clean_buffer(&mut self) {
        for i in 0..BUFFER_HEIGHT {
            self.clean_row(i);

        }
    }

}

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer {
        vga: VGA {
            buffer: unsafe { &mut *(BUFFER_ADDR as *mut Buffer) },
            cursor: Cursor { x: 0, y: 0}
        }
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

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    WRITER.lock().write_fmt(args).unwrap();
}
