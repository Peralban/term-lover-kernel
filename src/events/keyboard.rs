
use x86_64::instructions::port::Port;
use x86_64::structures::idt::InterruptStackFrame;
use crate::events::events::Event;
use crate::EVENT_QUEUE;

pub extern "x86-interrupt" fn keyboard_handler(
    _stack_frame: InterruptStackFrame
) {
    let scancode = unsafe {
        let mut data = Port::new(0x60);
        data.read()
    };

    EVENT_QUEUE.lock().push(from_scancode(scancode));

    unsafe {
        Port::new(0x20).write(0x20u8); // EOI
    }
}

static mut KEYBOARD_STATE: KeyboardState = KeyboardState {
    shift: false,
    ctrl: false,
    alt: false,
};

pub struct KeyboardState {
    shift: bool,
    ctrl: bool,
    alt: bool,
}

#[derive(Copy, Clone)]
pub struct Modifiers {
    pub shift: bool,
    pub ctrl: bool,
    pub alt: bool,
    pub super_key: bool,
}

#[derive(Copy, Clone)]
pub struct KeyEvent {
    pub key: u8,
    pub mods: Modifiers,
}

fn from_scancode(scancode: u8) -> Option<Event> {
    unsafe {
        match scancode {
            0x2A => { KEYBOARD_STATE.shift = true; return None; } // shift press
            0xAA => { KEYBOARD_STATE.shift = false; return None; } // shift release
            0x1D => { KEYBOARD_STATE.ctrl = true; return None; } // ctrl press
            0x9D => { KEYBOARD_STATE.ctrl = false; return None; } // ctrl release
            0x38 => { KEYBOARD_STATE.alt = true; return None; } // alt press
            0xB8 => { KEYBOARD_STATE.alt = false; return None; } // alt release

            _ => {}
        }

        let key = decode_key(scancode);

        Some(Event::KeyPress(KeyEvent {
            key,
            mods: Modifiers {
                shift: KEYBOARD_STATE.shift,
                ctrl: KEYBOARD_STATE.ctrl,
                alt: KEYBOARD_STATE.alt,
                super_key: false,
            }
        }))
    }
}

fn decode_key(scancode: u8) -> u8 {
    match scancode {
        // Number row
        0x02 => b'1',
        0x03 => b'2',
        0x04 => b'3',
        0x05 => b'4',
        0x06 => b'5',
        0x07 => b'6',
        0x08 => b'7',
        0x09 => b'8',
        0x0a => b'9',
        0x0b => b'0',
        0x0c => b'-',
        0x0d => b'=',
        // Top letter row
        0x10 => b'q',
        0x11 => b'w',
        0x12 => b'e',
        0x13 => b'r',
        0x14 => b't',
        0x15 => b'y',
        0x16 => b'u',
        0x17 => b'i',
        0x18 => b'o',
        0x19 => b'p',
        0x1a => b'[',
        0x1b => b']',
        0x2b => b'\\',
        // Middle letter row
        0x1e => b'a',
        0x1f => b's',
        0x20 => b'd',
        0x21 => b'f',
        0x22 => b'g',
        0x23 => b'h',
        0x24 => b'j',
        0x25 => b'k',
        0x26 => b'l',
        0x27 => b';',
        0x28 => b'\'',
        // Bottom letter row
        0x2c => b'z',
        0x2d => b'x',
        0x2e => b'c',
        0x2f => b'v',
        0x30 => b'b',
        0x31 => b'n',
        0x32 => b'm',
        0x33 => b',',
        0x34 => b'.',
        0x35 => b'/',
        // Special keys
        0x39 => b' ',
        0x0f => b'\t',  // Tab
        0x1c => b'\n', // Enter
        0x29 => b'`',  // Backtick
        _ => 0,
    }
}
