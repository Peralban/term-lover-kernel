
use crate::utils::lib::copy_into::copy_into;

pub fn my_split(buffer: &[u8], buffer_len: usize, delimiter: u8) -> ([[u8; 32]; 8], usize) {
    let mut start = 0;
    let mut arg_index = 0;

    let mut args: [[u8; 32]; 8] = [[0; 32]; 8];

    for i in 0..buffer_len {
        if buffer[i] == delimiter {
            let slice = &buffer[start..i];
            copy_into(&mut args[arg_index], slice);
            arg_index += 1;
            start = i + 1;
        }
    }

    let slice = &buffer[start..buffer_len];
    copy_into(&mut args[arg_index], slice);
    arg_index += 1;
    return (args, arg_index)
}
