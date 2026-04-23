
use crate::session::desktop::content::terminal::shell::command::Command;

#[repr(C)]
pub struct CommandEntry {
    name: &'static [u8],
    func: fn(&Command),
}

impl CommandEntry {
    pub fn get_name(&self) -> &'static [u8] {
        self.name
    }

    pub fn get_func(&self) -> fn(&Command) {
        self.func
    }
}

macro_rules! register_command {
    ($name:expr, $func:expr, $id:ident) => {
        #[link_section = ".commands"]
        #[used]
        static $id: CommandEntry = CommandEntry {
            name: $name,
            func: $func,
        };
    };
}
