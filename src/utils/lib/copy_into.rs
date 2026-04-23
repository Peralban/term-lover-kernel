
pub fn copy_into(dst: &mut [u8], src: &[u8]) {
    for i in 0..dst.len() {
        dst[i] = 0;
    }

    let len = core::cmp::min(dst.len(), src.len());

    for i in 0..len {
        dst[i] = src[i];
    }
}
