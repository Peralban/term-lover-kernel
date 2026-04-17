

use crate::session::desktop::content::terminal::cursor::Cursor;

pub struct Terminal {
    cursor: Cursor,

    start_x: i8,
    start_y: i8,
    widht: i8,
    height: i8,
    wall_color: u8,
}

impl Terminal {
    pub fn new() -> Self {
        Terminal {
            start_x: 0,
            start_y: 0,
            widht: 0,
            height: 0,
            wall_color: 0,

            cursor: Cursor::new(0xdb, 0x0f),  // █ character with white color
        }
    }
}
