#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

use core::panic::PanicInfo;
use x86_64::instructions::interrupts;
use spin::Mutex;


mod events;
mod x86_config;
mod session;
mod render;
mod utils;
mod drivers;

use x86_config::pic;
use x86_config::interrupt;
use events::events::EventQueue;
use events::events::Event;
use session::session::Session;
use events::events::Event_Return;
use render::Render;
use utils::dirty::Dirty;

pub static EVENT_QUEUE: Mutex<EventQueue> = Mutex::new(EventQueue::new());

fn pop_event() -> Option<Event> {
    interrupts::without_interrupts(|| {
        EVENT_QUEUE.lock().pop()
    })
}

pub fn push_event(event: Event) {
    interrupts::without_interrupts(|| {
        EVENT_QUEUE.lock().push(event);
    })
}

fn init() {
    interrupt::init_idt();
    pic::init();
    
    interrupts::enable();
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    init();

    let mut render = Render::new();
    let mut session = Dirty::new(Session::new(), Event_Return::VisualChange);
    loop {
        while let Some(_event) = pop_event() {
            if session.value().dispatch_events(_event).as_bool() {
                *session.changed() = Event_Return::VisualChange;
            }
        }
        if session.changed().as_bool() {
            session.value().get_current_desktop().update_screen();
        }
        render.render_screen(session.value().get_current_desktop().get_screen());
        *session.changed() = Event_Return::NoVisualChange;
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
