
use x86_64::instructions::port::Port;
use x86_64::structures::idt::InterruptStackFrame;
use crate::events::events::Event;
use crate::events::events::InputEvent;
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
    extended: false,
    super_key: false,
};

pub struct KeyboardState {
    shift: bool,
    ctrl: bool,
    alt: bool,
    extended: bool,
    super_key: bool,
}

#[derive(Copy, Clone)]
pub struct Modifiers {
    pub shift: bool,
    pub ctrl: bool,
    pub alt: bool,
    pub super_key: bool,
    pub extended: bool,
}

#[derive(Copy, Clone)]
pub struct KeyEvent {
    pub key: u8,
    pub mods: Modifiers,
}

fn from_scancode(scancode: u8) -> Option<Event> {
    unsafe {
        match scancode { // todo Delete ca et handle release
            0x2A => { KEYBOARD_STATE.shift = true; return None; } // shift press
            0xAA => { KEYBOARD_STATE.shift = false; return None; } // shift release
            0x1D => { KEYBOARD_STATE.ctrl = true; return None; } // ctrl press
            0x9D => { KEYBOARD_STATE.ctrl = false; return None; } // ctrl release
            0x38 => { KEYBOARD_STATE.alt = true; return None; } // alt press
            0xB8 => { KEYBOARD_STATE.alt = false; return None; } // alt release
            0xE0 => { KEYBOARD_STATE.extended = true; return None; } // extended

            _ => {}
        }

        if KEYBOARD_STATE.extended {
            KEYBOARD_STATE.extended = false;
            match scancode {
                0x5B => { KEYBOARD_STATE.super_key = true; return None; } // super key press
                0xDB => { KEYBOARD_STATE.super_key = false; return None; } // super key release
                _ => {}
            }
            let extended_key = decode_pressed_extended_key(scancode);
            if extended_key != 0 {
                return Some(Event::Input(InputEvent::KeyPress(KeyEvent {
                    key: extended_key,
                    mods: Modifiers {
                        shift: KEYBOARD_STATE.shift,
                        ctrl: KEYBOARD_STATE.ctrl,
                        alt: KEYBOARD_STATE.alt,
                        super_key: KEYBOARD_STATE.super_key,
                        extended: true,
                    }
                })));
            }
            return None;    
        }

        let key = decode_pressed_key(scancode);

        Some(Event::Input(InputEvent::KeyPress(KeyEvent {
            key,
            mods: Modifiers {
                shift: KEYBOARD_STATE.shift,
                ctrl: KEYBOARD_STATE.ctrl,
                alt: KEYBOARD_STATE.alt,
                super_key: KEYBOARD_STATE.super_key,
                extended: false,
            }
        })))
    }
}

fn decode_pressed_key(scancode: u8) -> u8 {
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
        // Space and enter
        0x39 => b' ',
        0x1c => b'\n',
        _ => 0,
    }
}

fn decode_pressed_extended_key(scancode: u8) -> u8 {
    match scancode {
        // numpad keys
        0x35 => b'/',  // Numpad slash
        0x37 => b'*',  // Numpad asterisk
        0x38 => b'-',  // Numpad minus
        0x39 => b'+',  // Numpad plus
        // arrow keys
        0x48 => b'U',  // Up arrow
        0x50 => b'D',  // Down arrow
        0x4b => b'L',  // Left arrow
        0x4d => b'R',  // Right arrow
        // other
        0x1c => b'\n', // Enter
        0x1d => b'\t', // Tab
        0x47 => b'H',  // Home
        0x4f => b'E',  // End
        0x49 => b'P',  // Page Up
        0x51 => b'N',  // Page Down
        _ => 0,
    }
}
