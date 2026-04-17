use crate::session::desktop::content::app_events::WriteEvent;
use crate::session::desktop::content::app_events::MoveCursorEvent;
use crate::session::desktop::content::terminal::cursor::Cursor;
use crate::session::desktop::content::terminal::terminal::Terminal;
use crate::events::events::Event_Return;
use crate::utils::cell::Cell;

pub enum App {
    Terminal(Terminal)
}

impl App {
    pub fn write_ascii(&mut self, we: WriteEvent) -> Event_Return {
        match self {
            App::Terminal(term) => term.write_ascii(we),
        }
    }

    pub fn move_cursor(&mut self, ce: MoveCursorEvent) -> Event_Return {
        match self {
            App::Terminal(term) => term.move_cursor(ce),
        }
    }

    pub fn get_buffer(&self) -> &[[Cell; 80]; 25] {
        match self {
            App::Terminal(term) => term.get_buffer(),
        }
    }

    pub fn get_cursor(&self) -> &Cursor {
        match self {
            App::Terminal(term) => term.get_cursor(),
        }
    }
}
