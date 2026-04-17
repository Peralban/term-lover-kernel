
#[derive(Copy, Clone)]
pub struct WriteEvent {
    pub ascii: u8,
}

#[derive(Copy, Clone)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Copy, Clone)]
pub struct MoveCursorEvent {
    pub direction: Direction,
}
