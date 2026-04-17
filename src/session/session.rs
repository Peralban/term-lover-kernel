
use crate::session::desktop::desktop::Desktop;

#[derive(Copy, Clone)]
pub enum SessionEvent {

}

pub struct Session {
    desktop: [Option<Desktop>; 2],
    focus: usize,
}

impl Session {
    pub fn new() -> Self {
        Session { desktop: [Some(Desktop::new()), None], focus: 0, }
    }

    pub fn get_current_desktop(&mut self) -> &mut Desktop {
        self.desktop[self.focus].as_mut().expect("focused desktop must exist")
    }
}
