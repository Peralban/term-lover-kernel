
use crate::session::desktop::content::app_manager::AppManager;
use crate::session::desktop::content::screen::Screen;
// use crate::session::desktop::content::cursor::Mouth;

#[derive(Copy, Clone)]
pub enum DesktopEvent {

}

pub enum ScreenUpdate<'a> {
    Windows(&'a AppManager),
    // Cursor(&'a Mouth),
}

pub struct Desktop {
    app_manager: AppManager,
    // mouth: Dirty<Mouth>,
    screen: Screen,
}

impl Desktop {
    pub fn new() -> Self {
        Desktop {
            app_manager: AppManager::new(),
            // mouth: Dirty::new(Mouth::new(0xdb, 0x0f), false),
            screen: Screen::new(),
        }
    }

    pub fn update_screen(&mut self) -> () {
            self.screen.update_screen(ScreenUpdate::Windows(&self.app_manager));

        // if *self.mouth.changed() {
        //     self.screen.update_screen(ScreenUpdate::Cursor(self.mouth.value()));
        // }
    }

    pub fn get_screen(&self) -> &Screen {
        &self.screen
    }

    pub fn get_app_manager(&mut self) -> &mut AppManager {
        &mut self.app_manager
    }
}
