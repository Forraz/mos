use core::fmt;

use bootloader_api::info::FrameBuffer;
use noto_sans_mono_bitmap::{get_raster, FontWeight, RasterHeight, RasterizedChar};

use super::utils::racy_cell::RacyCell;

const BORDER_PADDING: usize = 2;
const FONT_WEIGHT: FontWeight = FontWeight::Bold;
const FONT_SIZE: RasterHeight = RasterHeight::Size20;
const FONT_HOR_SPACING: usize = 0;
const FONT_VER_SPACING: usize = 0;
const BACKUP_CHAR: char = '�';

struct Cursor {
    x_pos: usize,
    y_pos: usize
}


pub struct Writer {
    framebuffer: FrameBuffer,
    cursor: Cursor
}

impl Writer {
    
    pub fn new(framebuffer: FrameBuffer) -> Self {
        Self {
            framebuffer,
            cursor: Cursor {
                x_pos: BORDER_PADDING,
                y_pos: BORDER_PADDING
            }
        }
    }

    fn get_index(&self, x_pos: usize, y_pos: usize) -> usize {
        let info = self.framebuffer.info();
        (x_pos + info.width * y_pos) * 4

    }

    fn new_line(&mut self) {
        self.cursor.y_pos += FONT_SIZE.val() + FONT_VER_SPACING;
        self.cursor.x_pos = BORDER_PADDING;
    }

    pub fn clear(&mut self) {
        for i in self.framebuffer.buffer_mut() {
            *i = 0;
        }
    }
    
    fn move_cursor(&mut self, width: usize) {
        self.cursor.x_pos += width + FONT_HOR_SPACING;
        if self.cursor.x_pos >= self.framebuffer.info().width {
            self.new_line();
        }
    }

    fn write_pixel(&mut self, pixel: u32, x_pos: usize, y_pos: usize) {
        let index = self.get_index(x_pos, y_pos);
        for i in 0..3 {
            self.framebuffer.buffer_mut()[index+i] = (pixel >> (8*i) & (u8::max_value() as u32)) as u8;
        }
    }
    
    pub fn get_rasterized_char(&self, c: char) -> RasterizedChar {
        fn get(c: char) -> Option<RasterizedChar> {
            get_raster(c, FONT_WEIGHT, FONT_SIZE)
        }
        
        get(c).unwrap_or_else(|| get(BACKUP_CHAR).expect(":("))
    }

    fn write_char(&mut self, c: char) {
        let rasterized_char = self.get_rasterized_char(c);
        let raster = rasterized_char.raster();
        let width = rasterized_char.width();

        for (y, row) in raster.iter().enumerate() {
            let y_pos = self.cursor.y_pos + y;

            for (x, intensity) in row.iter().enumerate() {
                let pixel = (*intensity) as u32 | ((*intensity as u32) << 8) | ((*intensity as u32) << 16);
                let x_pos = self.cursor.x_pos + x;
                self.write_pixel(pixel, x_pos, y_pos);
            }
        }
        self.move_cursor(width);
    }

    pub fn write_string(&mut self, s: &str) {
        for c in s.chars() {
            match c {
                '\n' => self.new_line(),
                _ => self.write_char(c)
            }
        }
    }
    
}

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

unsafe impl Send for Writer {}
unsafe impl Sync for Writer {} 

pub static WRITER: RacyCell<Option<Writer>> = RacyCell::new(None);

pub fn set_writer(framebuffer: FrameBuffer) {
   unsafe { *WRITER.get_mut() = Some(Writer::new(framebuffer)) }
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::framebuffer::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    unsafe { WRITER.get_mut() }.as_mut().unwrap().write_fmt(args).unwrap();
}
