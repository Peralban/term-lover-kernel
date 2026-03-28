
use x86_64::structures::idt::InterruptDescriptorTable;
use crate::events::keyboard::keyboard_handler;

static mut IDT: InterruptDescriptorTable = InterruptDescriptorTable::new();

pub fn init_idt() {
    unsafe {
        IDT[33].set_handler_fn(keyboard_handler); // IRQ1 = 33 après remap
        IDT.load();
    }
}
