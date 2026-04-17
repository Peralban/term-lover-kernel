
use crate::events::events::Event_Return;
use crate::session::desktop::content::app_manager::AppEvent;
use crate::session::desktop::content::app_manager::AppManager;

impl AppManager {
    pub fn events_handler(&mut self, _event: AppEvent) -> Event_Return {
        match _event {
            AppEvent::WriteAscii(we) => self.get_focused_app().write_ascii(we),
            AppEvent::MoveCursor(mce) => self.get_focused_app().move_cursor(mce),
        }
    }
}
