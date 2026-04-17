
use crate::events::events::Event_Return;
use crate::session::desktop::desktop::Desktop;
use crate::session::desktop::desktop::DesktopEvent;

impl Desktop {
    pub fn events_handler(&mut self, _event: DesktopEvent) -> Event_Return {
        Event_Return::NoVisualChange
    }
}
