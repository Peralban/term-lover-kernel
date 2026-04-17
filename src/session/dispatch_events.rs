
use crate::events::events::Event;
use crate::events::events::Event_Return;
use crate::events::events::UiEvent;
use crate::session::session::Session;
use crate::drivers::events_handler::events_handler;

impl Session {
    pub fn dispatch_events(&mut self, _event: Event) -> Event_Return {
        match _event {
            Event::Input(ie) => { events_handler(ie) }
            Event::UI(ui) => { self.dispatch_ui_events(ui) }
        }
    }

    pub fn dispatch_ui_events(&mut self, ui: UiEvent) -> Event_Return {
        match ui {
            UiEvent::Session(se) => { self.events_handler(se) }
            UiEvent::Desktop(de) => { self.get_current_desktop().events_handler(de) }
            UiEvent::App(ae) => { self.get_current_desktop().get_app_manager().events_handler(ae) }
        }
    }
}
