
use x86_64::structures::idt::InterruptDescriptorTable;
use crate::drivers::keyboard::keyboard_handler;

static mut IDT: InterruptDescriptorTable = InterruptDescriptorTable::new();

pub fn init_idt() {
    unsafe {
        let idt_ptr = &raw mut IDT;
        let idt = &mut *idt_ptr;

        idt[33].set_handler_fn(keyboard_handler);
        idt.load();
    }
}
