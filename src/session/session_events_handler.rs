
use crate::events::events::Event_Return;
use crate::session::session::Session;
use crate::session::session::SessionEvent;

impl Session {
    pub fn events_handler(&mut self, _event: SessionEvent) -> Event_Return {
        Event_Return::NoVisualChange
    }
}
