
use crate::session::desktop::desktop::ScreenUpdate;
use crate::session::desktop::content::app_manager::AppManager;
use crate::utils::cell::Cell;

pub struct Screen {
    screen: [[Cell; 80]; 25]
}

impl Screen {
    pub fn new() -> Self {
        Screen { screen: [[Cell::new(0, 0); 80]; 25] }
    }

    pub fn get_screen(&self) -> &[[Cell; 80]; 25] {
        &self.screen
    }

    pub fn update_screen(&mut self, update: ScreenUpdate) -> () {
        match update {
            ScreenUpdate::Windows(w) => { self.update_screen_windows(w); }
        }
    }

    fn update_screen_windows(&mut self, manager: &AppManager) -> () {
        // TODO get screen order and applay in decroissant order
        for app in manager.get_apps().iter().filter(|app| app.is_some()).flatten() {
            self.screen = app.get_buffer().clone();
            let cursor = app.get_cursor();
            self.screen[cursor.y][cursor.x].set_cell(cursor.simbol, cursor.color);
        }
    }
}
