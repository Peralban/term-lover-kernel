#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

use core::panic::PanicInfo;
use x86_64::instructions::interrupts;
use spin::Mutex;


mod events;
mod x86_config;

use x86_config::pic;
use x86_config::interrupt;
use events::events::EventQueue;
// use events::events_handler;

pub static EVENT_QUEUE: Mutex<EventQueue> = Mutex::new(EventQueue::new());

fn init() {
    interrupt::init_idt();
    pic::init();
    
    interrupts::enable();
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    init();
    loop {
        while let Some(event) = EVENT_QUEUE.lock().pop() {
            // events_handler::events_handler(event);
        }
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
