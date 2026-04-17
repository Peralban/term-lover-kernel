
use crate::session::desktop::desktop::Destop;

pub struct Session {
    desktop: [Option<Destop>; 2],
    focus: usize,
}

impl Session {
    pub fn new() -> Self {
        Session { desktop: [Some(Destop::new()), None], focus: 0, }
    }

    pub fn get_current_desktop(&mut self) -> &mut Destop {
        self.desktop[self.focus].as_mut().expect("focused desktop must exist")
    }
}
