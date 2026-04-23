
use x86_64::instructions::port::Port;
use x86_64::structures::idt::InterruptStackFrame;
use crate::events::events::Event;
use crate::events::events::InputEvent;
use crate::push_event;

pub extern "x86-interrupt" fn keyboard_handler(
    _stack_frame: InterruptStackFrame
) {
    let scancode = unsafe {
        let mut data = Port::new(0x60);
        data.read()
    };

    if let Some(event) = from_scancode(scancode) {
        push_event(event);
    }

    unsafe {
        Port::new(0x20).write(0x20u8); // EOI
    }
}

pub static mut KEYBOARD_STATE: KeyboardState = KeyboardState {
    shift: false,
    ctrl: false,
    alt: false,
    extended: false,
    super_key: false,
};

pub struct KeyboardState {
    pub shift: bool,
    pub ctrl: bool,
    pub alt: bool,
    pub extended: bool,
    pub super_key: bool,
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

pub struct KeyData {
    pub key: u8,
    pub p_or_r: bool, // true for press, false for release
}

pub fn change_state(key: u8, pressed: bool) {
    unsafe {
        match key {
            0x2A => KEYBOARD_STATE.shift = pressed, // Shift
            0x1D => KEYBOARD_STATE.ctrl = pressed,  // Ctrl
            0x38 => KEYBOARD_STATE.alt = pressed,   // Alt
            0x5B => KEYBOARD_STATE.super_key = pressed, // Super key
            _ => {}
        }
    }
}

fn from_scancode(scancode: u8) -> Option<Event> {
    unsafe {
        match scancode {
            0xE0 => { KEYBOARD_STATE.extended = true; return None; } // extended
            _ => {}
        }
        let key_event: KeyEvent;
        let data: KeyData;
        let ie: InputEvent;
        let mods = Modifiers {
            shift: KEYBOARD_STATE.shift,
            ctrl: KEYBOARD_STATE.ctrl,
            alt: KEYBOARD_STATE.alt,
            super_key: KEYBOARD_STATE.super_key,
            extended: KEYBOARD_STATE.extended,
        };

        data = if KEYBOARD_STATE.extended {
            KEYBOARD_STATE.extended = false;
            decode_extended_key(scancode)
        } else {
            decode_key(scancode)
        };

        key_event = KeyEvent {
            key: data.key,
            mods: mods
        };

        ie = if data.p_or_r {
            InputEvent::KeyPress(key_event)
        } else {
            InputEvent::KeyRelease(key_event)
        };

        Some(Event::Input(ie))
    }
}

fn decode_key(scancode: u8) -> KeyData {
    match scancode {
        // Pressed keys
        // Number row
        0x02 => KeyData { key: b'1', p_or_r: true },
        0x03 => KeyData { key: b'2', p_or_r: true },
        0x04 => KeyData { key: b'3', p_or_r: true },
        0x05 => KeyData { key: b'4', p_or_r: true },
        0x06 => KeyData { key: b'5', p_or_r: true },
        0x07 => KeyData { key: b'6', p_or_r: true },
        0x08 => KeyData { key: b'7', p_or_r: true },
        0x09 => KeyData { key: b'8', p_or_r: true },
        0x0a => KeyData { key: b'9', p_or_r: true },
        0x0b => KeyData { key: b'0', p_or_r: true },
        0x0c => KeyData { key: b'-', p_or_r: true },
        0x0d => KeyData { key: b'=', p_or_r: true },
        // Top letter row
        0x10 => KeyData { key: b'q', p_or_r: true },
        0x11 => KeyData { key: b'w', p_or_r: true },
        0x12 => KeyData { key: b'e', p_or_r: true },
        0x13 => KeyData { key: b'r', p_or_r: true },
        0x14 => KeyData { key: b't', p_or_r: true },
        0x15 => KeyData { key: b'y', p_or_r: true },
        0x16 => KeyData { key: b'u', p_or_r: true },
        0x17 => KeyData { key: b'i', p_or_r: true },
        0x18 => KeyData { key: b'o', p_or_r: true },
        0x19 => KeyData { key: b'p', p_or_r: true },
        0x1a => KeyData { key: b'[', p_or_r: true },
        0x1b => KeyData { key: b']', p_or_r: true },
        0x2b => KeyData { key: b'\\', p_or_r: true },
        // Middle letter row
        0x1e => KeyData { key: b'a', p_or_r: true },
        0x1f => KeyData { key: b's', p_or_r: true },
        0x20 => KeyData { key: b'd', p_or_r: true },
        0x21 => KeyData { key: b'f', p_or_r: true },
        0x22 => KeyData { key: b'g', p_or_r: true },
        0x23 => KeyData { key: b'h', p_or_r: true },
        0x24 => KeyData { key: b'j', p_or_r: true },
        0x25 => KeyData { key: b'k', p_or_r: true },
        0x26 => KeyData { key: b'l', p_or_r: true },
        0x27 => KeyData { key: b';', p_or_r: true },
        0x28 => KeyData { key: b'\'', p_or_r: true },
        // Bottom letter row
        0x2c => KeyData { key: b'z', p_or_r: true },
        0x2d => KeyData { key: b'x', p_or_r: true },
        0x2e => KeyData { key: b'c', p_or_r: true },
        0x2f => KeyData { key: b'v', p_or_r: true },
        0x30 => KeyData { key: b'b', p_or_r: true },
        0x31 => KeyData { key: b'n', p_or_r: true },
        0x32 => KeyData { key: b'm', p_or_r: true },
        0x33 => KeyData { key: b',', p_or_r: true },
        0x34 => KeyData { key: b'.', p_or_r: true },
        0x35 => KeyData { key: b'/', p_or_r: true },
        // Space and enter
        0x39 => KeyData { key: b' ', p_or_r: true },
        0x1c => KeyData { key: b'\n', p_or_r: true },
        0x0e => KeyData { key: 0x08, p_or_r: true }, // Backspace character
        0x0f => KeyData { key: b'\t', p_or_r: true },
        0x29 => KeyData { key: b'`', p_or_r: true },
        // special keys
        0x2A => KeyData { key: 0x2A, p_or_r: true }, // Shift press
        0x1D => KeyData { key: 0x1D, p_or_r: true }, // Ctrl press
        0x38 => KeyData { key: 0x38, p_or_r: true }, // Alt press

        // Released keys
        0x82 => KeyData { key: b'1', p_or_r: false },
        0x83 => KeyData { key: b'2', p_or_r: false },
        0x84 => KeyData { key: b'3', p_or_r: false },
        0x85 => KeyData { key: b'4', p_or_r: false },
        0x86 => KeyData { key: b'5', p_or_r: false },
        0x87 => KeyData { key: b'6', p_or_r: false },
        0x88 => KeyData { key: b'7', p_or_r: false },
        0x89 => KeyData { key: b'8', p_or_r: false },
        0x8a => KeyData { key: b'9', p_or_r: false },
        0x8b => KeyData { key: b'0', p_or_r: false },
        0x8c => KeyData { key: b'-', p_or_r: false },
        0x8d => KeyData { key: b'=', p_or_r: false },
        0x90 => KeyData { key: b'q', p_or_r: false },
        0x91 => KeyData { key: b'w', p_or_r: false },
        0x92 => KeyData { key: b'e', p_or_r: false },
        0x93 => KeyData { key: b'r', p_or_r: false },
        0x94 => KeyData { key: b't', p_or_r: false },
        0x95 => KeyData { key: b'y', p_or_r: false },
        0x96 => KeyData { key: b'u', p_or_r: false },
        0x97 => KeyData { key: b'i', p_or_r: false },
        0x98 => KeyData { key: b'o', p_or_r: false },
        0x99 => KeyData { key: b'p', p_or_r: false },
        0x9a => KeyData { key: b'[', p_or_r: false },
        0x9b => KeyData { key: b']', p_or_r: false },
        0xab => KeyData { key: b'\\', p_or_r: false },
        0x9e => KeyData { key: b'a', p_or_r: false },
        0x9f => KeyData { key: b's', p_or_r: false },
        0xa0 => KeyData { key: b'd', p_or_r: false },
        0xa1 => KeyData { key: b'f', p_or_r: false },
        0xa2 => KeyData { key: b'g', p_or_r: false },
        0xa3 => KeyData { key: b'h', p_or_r: false },
        0xa4 => KeyData { key: b'j', p_or_r: false },
        0xa5 => KeyData { key: b'k', p_or_r: false },
        0xa6 => KeyData { key: b'l', p_or_r: false },
        0xa7 => KeyData { key: b';', p_or_r: false },
        0xa8 => KeyData { key: b'\'', p_or_r: false },
        0xac => KeyData { key: b'z', p_or_r: false },
        0xad => KeyData { key: b'x', p_or_r: false },
        0xae => KeyData { key: b'c', p_or_r: false },
        0xaf => KeyData { key: b'v', p_or_r: false },
        0xb0 => KeyData { key: b'b', p_or_r: false },
        0xb1 => KeyData { key: b'n', p_or_r: false },
        0xb2 => KeyData { key: b'm', p_or_r: false },
        0xb3 => KeyData { key: b',', p_or_r: false },
        0xb4 => KeyData { key: b'.', p_or_r: false },
        0xb5 => KeyData { key: b'/', p_or_r: false },
        0xb9 => KeyData { key: b' ', p_or_r: false },
        0x9c => KeyData { key: b'\n', p_or_r: false },
        0x8e => KeyData { key: 0x08, p_or_r: false }, // Backspace character
        0x8f => KeyData { key: b'\t', p_or_r: false },
        0xa9 => KeyData { key: b'`', p_or_r: false },
        0xAA => KeyData { key: 0x2A, p_or_r: false }, // Shift release
        0x9D => KeyData { key: 0x1D, p_or_r: false }, // Ctrl release
        0xB8 => KeyData { key: 0x38, p_or_r: false }, // Alt release

        _ => KeyData { key: 0, p_or_r: false }, // Unknown key
    }
}

fn decode_extended_key(scancode: u8) -> KeyData {
    match scancode {
        // pressed keys
        // numpad keys
        0x35 => KeyData { key: b'/', p_or_r: true },  // Numpad slash
        0x37 => KeyData { key: b'*', p_or_r: true },  // Numpad asterisk
        0x38 => KeyData { key: b'-', p_or_r: true },  // Numpad minus
        0x39 => KeyData { key: b'+', p_or_r: true },  // Numpad plus
        // arrow keys
        0x48 => KeyData { key: b'U', p_or_r: true },  // Up arrow
        0x50 => KeyData { key: b'D', p_or_r: true },  // Down arrow
        0x4b => KeyData { key: b'L', p_or_r: true },  // Left arrow
        0x4d => KeyData { key: b'R', p_or_r: true },  // Right arrow
        // other
        0x1c => KeyData { key: b'\n', p_or_r: true }, // Enter
        0x1d => KeyData { key: 0x1D, p_or_r: true }, // Ctrl press (right)
        0x47 => KeyData { key: b'H', p_or_r: true },  // Home
        0x4f => KeyData { key: b'F', p_or_r: true },  // End
        0x49 => KeyData { key: b'P', p_or_r: true },  // Page Up
        0x51 => KeyData { key: b'N', p_or_r: true },  // Page Down
        // super key
        0x5B => KeyData { key: 0x5B, p_or_r: true }, // Super key press

        // released keys
        0xB5 => KeyData { key: b'/', p_or_r: false },  // Numpad slash release
        0xB7 => KeyData { key: b'*', p_or_r: false },  // Numpad asterisk release
        0xB8 => KeyData { key: b'-', p_or_r: false },  // Numpad minus release
        0xB9 => KeyData { key: b'+', p_or_r: false },  // Numpad plus release
        0xC8 => KeyData { key: b'U', p_or_r: false },  // Up arrow release
        0xD0 => KeyData { key: b'D', p_or_r: false },  // Down arrow release
        0xCB => KeyData { key: b'L', p_or_r: false },  // Left arrow release
        0xCD => KeyData { key: b'R', p_or_r: false },  // Right arrow release
        0x9C => KeyData { key: b'\n', p_or_r: false }, // Enter release
        0x9D => KeyData { key: 0x1D, p_or_r: false }, // Ctrl release (right)
        0xC7 => KeyData { key: b'H', p_or_r: false },  // Home release
        0xCF => KeyData { key: b'F', p_or_r: false },  // End release
        0xC9 => KeyData { key: b'P', p_or_r: false },  // Page Up release
        0xD1 => KeyData { key: b'N', p_or_r: false },  // Page Down release
        0xDB => KeyData { key: 0x5B, p_or_r: false }, // Super key release
        _ => KeyData { key: 0, p_or_r: false }, // Unknown key
    }
}
