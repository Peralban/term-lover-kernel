
#[derive(Clone)]
#[derive(Copy)]
pub struct Cell {
    ascii: u8,
    color: u8,
}

impl Cell {
    pub fn new(ascii: u8, color: u8) -> Self {
        Cell {
            ascii,
            color,
        }
    }

    pub fn get_ascii(&self) -> u8 {
        self.ascii
    }

    pub fn get_color(&self) -> u8 {
        self.color
    }

    pub fn get_ascii_mut(&mut self) -> &mut u8 {
        &mut self.ascii
    }

    pub fn get_color_mut(&mut self) -> &mut u8 {
        &mut self.color
    }

    pub fn set_cell(&mut self, ascii: u8, color: u8) -> () {
        self.ascii = ascii;
        self.color = color;
    }
}
