
pub struct Cursor {
    pub x: usize,
    pub y: usize,
    pub simbol: u8,
    pub color: u8,
}

impl Cursor {
    pub fn new(simbol: u8, color: u8) -> Self {
        Cursor { x: 0, y: 0, simbol, color }
    }

    pub fn move_left(&mut self) {
        if self.x > 0 {
            self.x -= 1;
        }
    }

    pub fn move_right(&mut self) {
        if self.x < 79 {
            self.x += 1;
        }
    }

    pub fn move_up(&mut self) {
        if self.y > 0 {
            self.y -= 1;
        }
    }

    pub fn move_down(&mut self) {
        if self.y < 24 {
            self.y += 1;
        }
    }
}
