
pub struct Command {
    args: [[u8; 32]; 8],
    arg_count: usize,
}

impl Command {
    pub fn new(args: [[u8; 32]; 8], arg_count: usize,) -> Self {
        Command {
            args: args,
            arg_count: arg_count,
        }
    }

    pub fn get_arg(&self) -> &[[u8; 32]; 8] {
        &self.args
    }

    pub fn get_len(&self) -> usize {
        self.arg_count
    }
}
