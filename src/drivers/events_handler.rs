
use crate::push_event;
use crate::drivers::keyboard::KeyEvent;
use crate::drivers::keyboard::change_state;
use crate::events::events::Event;
use crate::events::events::InputEvent;
use crate::events::events::Event_Return;
use crate::events::events::UiEvent;
use crate::session::desktop::content::app_events::Direction;
use crate::session::desktop::content::app_events::MoveCursorEvent;
use crate::session::desktop::content::app_manager::AppEvent;
use crate::session::desktop::content::app_events::WriteEvent;

fn determine_case(shift: bool, key: u8) -> u8 {
    if shift {
        match key {
            b'a'..=b'z' => key - 32,
            b'1' => b'!',
            b'2' => b'@',
            b'3' => b'#',
            b'4' => b'$',
            b'5' => b'%',
            b'6' => b'^',
            b'7' => b'&',
            b'8' => b'*',
            b'9' => b'(',
            b'0' => b')',
            b'-' => b'_',
            b'=' => b'+',
            b'[' => b'{',
            b']' => b'}',
            b'\\' => b'|',
            b';' => b':',
            b'\'' => b'"',
            b',' => b'<',
            b'.' => b'>',
            b'/' => b'?',
            b'`' => b'~',
            _ => key,
        }
    } else {
        key
    }
}

fn kp_event_builder(_event: KeyEvent) -> Event_Return {
    if _event.key == 0x2A || _event.key == 0x1D || _event.key == 0x38 || _event.key == 0x5B {
        change_state(_event.key, true);
        return Event_Return::NoVisualChange;
    }
    if !(_event.mods.ctrl && _event.mods.alt && _event.mods.super_key) && !_event.mods.extended && matches!(_event.key, 32..=126) {
        push_event(Event::UI(UiEvent::App(AppEvent::WriteAscii(WriteEvent {
            ascii: determine_case(_event.mods.shift, _event.key),
        }))));
    }
    if _event.mods.extended {
        match _event.key {
            b'U' => { push_event(Event::UI(UiEvent::App(AppEvent::MoveCursor(MoveCursorEvent {
                direction: Direction::Up,
            })))); return Event_Return::VisualChange },
            b'D' => { push_event(Event::UI(UiEvent::App(AppEvent::MoveCursor(MoveCursorEvent {
                direction: Direction::Down,
            })))); return Event_Return::VisualChange },
            b'L' => { push_event(Event::UI(UiEvent::App(AppEvent::MoveCursor(MoveCursorEvent {
                direction: Direction::Left,
            })))); return Event_Return::VisualChange },
            b'R' => { push_event(Event::UI(UiEvent::App(AppEvent::MoveCursor(MoveCursorEvent {
                direction: Direction::Right,
            })))); return Event_Return::VisualChange },
            _ => {}
        }
    }
    Event_Return::NoVisualChange
}

fn kr_event_builder(_event: KeyEvent) -> Event_Return {
    if _event.key == 0x2A || _event.key == 0x1D || _event.key == 0x38 || _event.key == 0x5B {
        change_state(_event.key, false);
    }
    Event_Return::NoVisualChange
}

pub fn events_handler(_event: InputEvent) -> Event_Return {
    match _event {
        InputEvent::KeyPress(kp) => { return kp_event_builder(kp) }
        InputEvent::KeyRelease(kr) => {return kr_event_builder(kr) }
    }
}
