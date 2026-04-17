
// shell connection

use crate::events::events::Event;
use crate::session::session::Session;

pub enum Event_Return {
    NoVisualChange = 0,
    VisualChange = 1,
}

impl Event_Return {
    pub fn as_bool(self) -> bool {
        matches!(self, Self::VisualChange)
    }
}

impl Session {
    pub fn events_handler(&mut self, _event: Event) -> Event_Return {
        todo!("faire la function");
        Event_Return::NoVisualChange
    }
}
