
use crate::session::desktop::content::terminal::terminal::Terminal;
use crate::session::desktop::content::app_events::MoveCursorEvent;
use crate::session::desktop::content::app_events::WriteEvent;
use crate::session::desktop::content::app::App;

#[derive(Copy, Clone)]
pub enum AppEvent {
    MoveCursor(MoveCursorEvent),
    WriteAscii(WriteEvent),
}
pub struct AppManager {
    apps: [Option<App>; 4], // TODO: mettre l'orde de focus
    focus: usize,
}


impl AppManager {
    pub fn new() -> Self {
        AppManager { 
            apps: [Some(App::Terminal(Terminal::new(0, 0, 80, 25, b' ', 0x0f))), None, None, None],
            focus: 0,
        }
    }

    pub fn get_focused_app(&mut self) -> &mut App {
        self.apps[self.focus].as_mut().expect("focused app must exist")
    }

    pub fn get_apps(&self) -> &[Option<App>; 4] {
        &self.apps
    }
}
