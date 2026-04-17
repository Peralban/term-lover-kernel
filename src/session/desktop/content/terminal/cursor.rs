
pub struct Cursor {
    pub x: usize,
    pub y: usize,


    pub simbol: u8,
    pub color: u8,
}

impl Cursor {
    pub fn new(x: usize, y: usize, simbol: u8, color: u8) -> Self {
        Cursor { x, y, simbol, color }
    }

    pub fn move_left(&mut self) {
        self.x -= 1;
    }

    pub fn move_right(&mut self) {
        self.x += 1;
    }

    pub fn move_up(&mut self) {
        self.y -= 1;
    }

    pub fn move_down(&mut self) {
        self.y += 1;
    }
}
