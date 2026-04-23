
use crate::{drivers::keyboard::KeyEvent, session::{desktop::{content::app_manager::AppEvent, desktop::DesktopEvent}, session::SessionEvent}};
use x86_64::instructions::interrupts;

#[derive(Copy, Clone)]
pub enum InputEvent { // futur peut etre que ca a term ca va devenir Keyboard et Mouth et les key event enum seront decla et mise dans keyboard 
    KeyPress(KeyEvent),
    KeyRelease(KeyEvent),
}

#[derive(Copy, Clone)]
pub enum UiEvent {
    Session(SessionEvent),
    Desktop(DesktopEvent),
    App(AppEvent),
}

#[derive(Copy, Clone)]
pub enum Event {
    Input(InputEvent),
    UI(UiEvent),
}


#[derive(Copy, Clone)]
pub enum Event_Return {
    NoVisualChange = 0,
    VisualChange = 1,
}

impl Event_Return {
    pub fn as_bool(self) -> bool {
        matches!(self, Self::VisualChange)
    }
}


pub struct EventQueue {
    events: [Option<Event>; 256],
    head: usize,
    tail: usize,
}

impl EventQueue {
    pub const fn new() -> Self {
        EventQueue {
            events: [None; 256],
            head: 0,
            tail: 0,
        }
    }

    pub fn pop(&mut self) -> Option<Event> {
        interrupts::without_interrupts(|| {
            if self.head == self.tail {
                return None;
            }

            let event = self.events[self.head].take();
            self.head = (self.head + 1) % 256;
            event
        })
    }

    pub fn push(&mut self, event: Event) {
        let next = (self.tail + 1) % 256;

        if next == self.head {
            return;
        }

        self.events[self.tail] = Some(event);
        self.tail = next;
    }
}
