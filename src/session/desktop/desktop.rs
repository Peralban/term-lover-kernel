
use crate::session::desktop::content::windows::WindowManager;
use crate::session::desktop::content::screen::Screen;

pub struct Destop {
    windowsManager: WindowManager,
    screen: Screen,
}

impl Destop {
    pub fn new() -> Self {
        Destop {
            windowsManager: WindowManager::new(),
            screen: Screen::new(),
        }
    }

    pub fn update_screen(&mut self) -> () {
        
    }

    pub fn get_screen(&self) -> &Screen {
        &self.screen
    }
}
