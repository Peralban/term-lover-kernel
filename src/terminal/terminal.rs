
mod cursor;

use cursor::Cursor;

pub struct Terminal { // Root terminal (it can have child terminals inside it)
    in_term: bool,
    cursor: Cursor,

    start_x: i8,
    start_y: i8,
    widht: i8,
    height: i8,
    wall_color: u8,
    children: [Option<Terminal>; 4], // Max 4 child terminals (2 rows x 2 columns)
}

impl Terminal {
    pub fn new() -> Self {
        Terminal { 
            in_term: false,
            start_x: 0,
            start_y: 0,
            widht: 0,
            height: 0,
            wall_color: 0,

            cursor: Cursor::new(0xdb, 0x0f),  // █ character with white color
        }
    }
}
