extern crate ctgrind;

fn check16_bad(a: &[u8], b: &[u8]) -> bool {
    for i in range(0, a.len()) {
        if a[i] != b[i] {
            return false;
        }
    }
    true
}

fn check16_good(a: &[u8], b: &[u8]) -> bool {
    let mut r = 0u8;
    for i in range(0, a.len()) {
        r |= a[i] ^ b[i];
    }
    return r == 0;
}

fn bad_memory_access(a: &[u8]) -> u8 {
    let r = [1u8, 0];
    r[(a[0] & 1) as usize]
}

pub fn main() {
    let a : [u8, ..16] = [0, ..16];
    let b : [u8, ..16] = [0, ..16];

    ctgrind::poison(a.as_slice().as_ptr() as *const (), a.len());
    println!("check16_bad");
    check16_bad(a.as_slice(), b.as_slice());
    println!("check16_good");
    check16_good(a.as_slice(), b.as_slice());
    println!("bad_memory_access");
    bad_memory_access(a.as_slice());
}
