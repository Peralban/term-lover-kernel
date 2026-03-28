
use crate::cursor::Cursor;
use crate::screen::Screen;

pub struct Render {
    vga_buffer: *mut u8,
}

impl Render {
    pub fn new(vga_buffer: *mut u8) -> Self {
        Render {
            vga_buffer: vga_buffer,
        }
    }

    fn clear_screen(&mut self) {
        for y in 0..25 {
            for x in 0..80 {
                unsafe {
                    *self.vga_buffer.add((y * 80 + x) * 2) = b' ';
                    *self.vga_buffer.add((y * 80 + x) * 2 + 1) = 0x0f;
                }
            }
        }
    }

    
    fn write_screen(&mut self, screen: &mut Screen, cursor: &Cursor) {
        for y in 0..25 {
            for x in 0..80 {
                let offset = (y * 80 + x) * 2;
                let (character, color) = if x == cursor.x && y == cursor.y {
                    (cursor.simbol, cursor.color)
                } else {
                    (screen.get_current_screen()[y][x], 0x0f)
                };
                
                if character != 0 {
                    self.write_character((offset, offset + 1), character, color);
                }
            }
        }
    }

    fn write_character(&mut self, position: (usize, usize), character: u8, color: u8) {
        let (char_pos, color_pos) = position;
        unsafe {
            *self.vga_buffer.add(char_pos) = character;
            *self.vga_buffer.add(color_pos) = color;
        }
    }

    pub fn render(&mut self, screen: &mut Screen, cursor: &Cursor) {
        self.clear_screen();
        self.write_screen(screen, cursor);
    }
}
