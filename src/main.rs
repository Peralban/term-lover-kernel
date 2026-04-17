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

use x86_config::pic;
use x86_config::interrupt;
use events::events::EventQueue;
use events::events::Event;
use session::session::Session;
use session::events_handler::Event_Return;
use render::Render;

pub static EVENT_QUEUE: Mutex<EventQueue> = Mutex::new(EventQueue::new());

fn pop_event() -> Option<Event> {
    interrupts::without_interrupts(|| {
        EVENT_QUEUE.lock().pop()
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
    let mut session = Session::new();
    loop {
        let mut visual_changes = Event_Return::NoVisualChange;
        while let Some(_event) = pop_event() {
            visual_changes = session.events_handler(_event); // next TODO
        }
        if visual_changes.as_bool() {
            session.get_current_desktop().update_screen();
        }
        render.render_screen(session.get_current_desktop().get_screen());
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
