
use crate::{EVENT_QUEUE, events::events::{Event, Event_Return, UiEvent}, session::desktop::content::{app_events::{Direction, MoveCursorEvent, WriteEvent}, app_manager::AppEvent, terminal::cursor::{self, Cursor}}};
use crate::utils::cell::Cell;

struct TerminalBorder {
    pub top_left_corner: u8,
    pub bot_left_corner: u8,
    pub top_right_corner: u8,
    pub bot_right_corner: u8,

    pub top_walls: u8,
    pub bot_walls: u8,
    pub left_walls: u8,
    pub right_walls: u8,

    pub top_color: u8,
    pub bot_color: u8,
    pub left_color: u8,
    pub right_color: u8,
}

impl TerminalBorder {
    pub fn new() -> Self {
        TerminalBorder {
            top_left_corner: 0xc9, // ╔
            bot_left_corner: 0xc8, // ╚
            top_right_corner: 0xbb, // ╗
            bot_right_corner: 0xbc, // ╝

            top_walls: 0xcd, // ═
            bot_walls: 0xcd, // ═
            left_walls: 0xba, // ║
            right_walls: 0xba, // ║

            top_color: 0x1e, // light yellow on blue
            bot_color: 0x1e, // light yellow on blue
            left_color: 0x1e, // light yellow on blue
            right_color: 0x1e, // light yellow on blue
        }
    }
}

struct TerminalBuffer {
    border: TerminalBorder,
    
    start_x: usize,
    start_y: usize,
    widht: usize,
    height: usize,

    bgd_ascii: u8,
    bgd_color: u8,

    buffer: [[Cell; 80]; 25],
}

impl TerminalBuffer {
    pub fn new(border: TerminalBorder, bgd_ascii: u8, bgd_color: u8, start_x: usize, start_y: usize, widht: usize, height: usize) -> Self {
        let mut tb = TerminalBuffer {
            buffer: [[Cell::new(bgd_ascii, bgd_color); 80]; 25],
            
            start_x,
            start_y,
            widht,
            height,

            bgd_ascii,
            bgd_color,

            border,
        }; 
        for y in start_y..start_y + height { // PRIO CHANGER CA
            for x in start_x..start_x + widht {
                if y == start_y && x == start_x {
                    tb.buffer[y][x].set_cell(tb.border.top_left_corner, tb.border.top_color);
                } else if y == start_y && x == start_x + widht - 1 {
                    tb.buffer[y][x].set_cell(tb.border.top_right_corner, tb.border.top_color);
                } else if y == start_y + height - 1 && x == start_x {
                    tb.buffer[y][x].set_cell(tb.border.bot_left_corner, tb.border.bot_color);
                } else if y == start_y + height - 1 && x == start_x + widht - 1 {
                    tb.buffer[y][x].set_cell(tb.border.bot_right_corner, tb.border.bot_color);
                } else if y == start_y {
                    tb.buffer[y][x].set_cell(tb.border.top_walls, tb.border.top_color);
                } else if y == start_y + height - 1 {
                    tb.buffer[y][x].set_cell(tb.border.bot_walls, tb.border.bot_color);
                } else if x == start_x {
                    tb.buffer[y][x].set_cell(tb.border.left_walls, tb.border.left_color);
                } else if x == start_x + widht - 1 {
                    tb.buffer[y][x].set_cell(tb.border.right_walls, tb.border.right_color);
                }
            }
        }
        tb
    }

    pub fn get_buffer_mut(&mut self) -> &mut [[Cell; 80]; 25] {
        &mut self.buffer
    }

    pub fn get_buffer(&self) -> &[[Cell; 80]; 25] {
        &self.buffer
    }

    pub fn get_border(&self) -> &TerminalBorder {
        &self.border
    }

    pub fn get_start_x(&self) -> usize {
        self.start_x
    }

    pub fn get_start_y(&self) -> usize {
        self.start_y
    }

    pub fn get_widht(&self) -> usize {
        self.widht
    }

    pub fn get_height(&self) -> usize {
        self.height
    }
}

pub struct Terminal {
    cursor: Cursor,


    buffer: TerminalBuffer,


    have_history: bool, // futur: SHELL 
}

impl Terminal {
    pub fn new(start_x: usize, start_y: usize, widht: usize, height: usize, bgd_ascii: u8, bgd_color: u8) -> Self {
        Terminal {
            cursor: Cursor::new(start_x + 1, start_y + 1, 0xdb, 0x0f),  // █ character with white color

            buffer: TerminalBuffer::new(TerminalBorder::new(), bgd_ascii, bgd_color, start_x, start_y, widht, height),


            have_history: false,
        }
    }

    pub fn write_ascii(&mut self, we: WriteEvent) -> Event_Return {
        *self.buffer.get_buffer_mut()[self.cursor.y][self.cursor.x].get_ascii_mut() = we.ascii;
        EVENT_QUEUE.lock().push(
            Some(Event::UI(UiEvent::App(AppEvent::MoveCursor(MoveCursorEvent { 
                direction: Direction::Right,
            }))))
        );
        Event_Return::VisualChange
    }

    pub fn move_cursor(&mut self, wce: MoveCursorEvent) -> Event_Return {
        let direction = wce.direction;
        if self.have_history {
            match direction {
                Direction::Up => return Event_Return::VisualChange, // TODO ofc lhistorique mais apres le shell
                Direction::Down => return Event_Return::VisualChange,
                _ => {}
            }
        }
        if self.cursor.x == self.buffer.get_start_x() + self.buffer.get_widht() - 2 && matches!(direction, Direction::Right) ||
            self.cursor.x == self.buffer.get_start_x() + 1 && matches!(direction, Direction::Left) {
            return Event_Return::NoVisualChange;
        }
        match direction {
            Direction::Left => self.cursor.move_left(),
            Direction::Right => self.cursor.move_right(),
            _ => {}
        }
        Event_Return::VisualChange
    }

    pub fn get_buffer(&self) -> &[[Cell; 80]; 25] {
        self.buffer.get_buffer()
    }

    pub fn get_cursor(&self) -> &Cursor {
        &self.cursor
    }
}
