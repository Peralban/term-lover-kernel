
use crate::session::desktop::content::screen::Screen;

pub struct Render {
    vga_buffer: *mut u8,
}

impl Render {
    pub fn new() -> Self {
        Render {
            vga_buffer: 0xb8000 as *mut u8,
        }
    }

    pub fn render_screen(&mut self, screen: &Screen) {

        let cells = screen.get_screen();

        for y in 0..25 {
            for x in 0..80 {
                unsafe {
                    *self.vga_buffer.offset((y * 80 + x) as isize * 2) = cells[y][x].get_ascii();
                    *self.vga_buffer.offset((y * 80 + x) as isize * 2 + 1) = cells[y][x].get_color();
                }
            }
        }
    }
}
