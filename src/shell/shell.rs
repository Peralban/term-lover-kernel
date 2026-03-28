
use crate::shell::terminal::Terminal;
use crate::shell::utils;
use crate::shell::command::Command;
use crate::shell::command_register::CommandEntry;

#[warn(improper_ctypes)]
unsafe extern "C" {
    static __start_commands: CommandEntry;
    static __stop_commands: CommandEntry;
}

pub struct Shell {
    buffer: [u8; 256],
    len: usize,

    cmd: Command,
    term: Terminal,
}

impl Shell {
    pub fn new() -> Self {
        Shell {
            buffer: [0; 256],
            len: 0,
            cmd: Command::new([[0; 32]; 8],0),
            term: Terminal::new(),
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

    // fn cmd_create_terminal(_cmd: &Command) {}

    // fn cmd_exit(_cmd: &Command) {}

    fn execute_cmd(&mut self) {
        let (cmd_array, array_len) = utils::my_split(&self.buffer, self.len, b' ');
        self.cmd = Command::new(cmd_array, array_len);
        let cmd_name = &self.cmd.get_arg()[0];

        unsafe {
            let mut ptr = &__start_commands as *const CommandEntry;
            let end = &__stop_commands as *const CommandEntry;

            while ptr < end {
                let entry = &*ptr;

                if utils::bytes_cmp(cmd_name, entry.get_name()) {
                    entry.get_func()(&self.cmd);
                    return;
                }

                ptr = ptr.add(1);
            }
        }

    }
}