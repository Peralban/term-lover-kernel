
use crate::events::keyboard::KeyEvent;
use x86_64::instructions::interrupts;

#[derive(Copy, Clone)]
pub enum Event {
    KeyPress(KeyEvent),
    KeyRelease(KeyEvent),
    //MouseMove { x: i32, y: i32 },
    //MouseClick { button: u8 },
    //Interrupt(u8),
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

    pub fn push(&mut self, event: Option<Event>) {
        let next = (self.tail + 1) % 256;

        if next == self.head {
            return;
        }

        self.events[self.tail] = event;
        self.tail = next;
    }
}
