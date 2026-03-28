
use x86_64::instructions::port::Port;

pub fn init() {
    unsafe {
        // ICW1
        Port::new(0x20).write(0x11u8);
        Port::new(0xA0).write(0x11u8);

        // ICW2 (offset)
        Port::new(0x21).write(32u8);
        Port::new(0xA1).write(40u8);

        // ICW3
        Port::new(0x21).write(4u8);
        Port::new(0xA1).write(2u8);

        // ICW4
        Port::new(0x21).write(0x01u8);
        Port::new(0xA1).write(0x01u8);

        // unmask keyboard (IRQ1)
        Port::new(0x21).write(0xFDu8);
    }
}
