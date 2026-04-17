
use crate::session::desktop::content::terminal::terminal::Terminal;

pub struct WindowManager {
    term: [Option<Terminal>; 4], // Max 4 terminals
    focus: usize,
}


impl WindowManager {
    pub fn new() -> Self {
        WindowManager { 
            term: [Some(Terminal::new()), None, None, None],
            focus: 0,
        }
    }
}
