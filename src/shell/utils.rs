
pub fn copy_into(dst: &mut [u8], src: &[u8]) {
    for i in 0..dst.len() {
        dst[i] = 0;
    }

    let len = core::cmp::min(dst.len(), src.len());

    for i in 0..len {
        dst[i] = src[i];
    }
}

pub fn bytes_cmp(a: &[u8], b: &[u8]) -> bool {
    let mut i = 0;

    while i < a.len() && i < b.len() {
        if a[i] == 0 && b[i] == 0 {
            return true;
        }
        if a[i] != b[i] {
            return false;
        }
        i += 1;
    }

    true
}

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
