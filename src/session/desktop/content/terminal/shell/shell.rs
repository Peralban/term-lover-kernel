
use crate::utils::lib::{my_split::my_split, bytes_cmp::bytes_cmp};
use crate::session::desktop::content::terminal::shell::command::Command;
use crate::session::desktop::content::terminal::shell::command_register::CommandEntry;

#[warn(improper_ctypes)]
unsafe extern "C" {
    static __start_commands: CommandEntry;
    static __stop_commands: CommandEntry;
}

pub struct Shell {
    buffer: [u8; 256],
    len: usize,
}

impl Shell {
    pub fn new() -> Self {
        Shell {
            buffer: [0; 256],
            len: 0,
        }
    }

    pub fn add_char(&mut self, c: u8) {
        if c == b'\n' {
            self.execute_cmd();
            self.len = 0;
            self.buffer = [0; 256];
            return;
        }

        if self.len < self.buffer.len() {
            self.buffer[self.len] = c;
            self.len += 1;
        }
    }

    fn execute_cmd(&mut self) {
        let (cmd_array, array_len) = my_split(&self.buffer, self.len, b' ');
        let cmd = Command::new(cmd_array, array_len);
        let cmd_name = cmd.get_arg()[0];

        unsafe {
            let mut ptr = &__start_commands as *const CommandEntry;
            let end = &__stop_commands as *const CommandEntry;

            while ptr < end {
                let entry = &*ptr;

                if bytes_cmp(&cmd_name, entry.get_name()) {
                    entry.get_func()(&cmd);
                    return;
                }

                ptr = ptr.add(1);
            }
        }
    }
}
