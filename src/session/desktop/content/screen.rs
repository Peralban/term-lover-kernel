
#[derive(Clone)]
#[derive(Copy)]
pub struct Cell {
    ascii: u8,
    color: u8,
}

impl Cell {
    pub fn new() -> Self {
        Cell {
            ascii: 0,
            color: 0,
        }
    }

    pub fn get_ascii(&self) -> u8 {
        self.ascii
    }

    pub fn get_color(&self) -> u8 {
        self.color
    }
}

pub struct Screen {
    screen: [[Cell; 80]; 25]
}

impl Screen {
    pub fn new() -> Self {
        Screen { screen: [[Cell::new(); 80]; 25] }
    }

    pub fn get_screen(&self) -> &[[Cell; 80]; 25] {
        &self.screen
    }
}
