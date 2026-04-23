
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
