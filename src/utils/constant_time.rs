pub fn constant_time_eq(a: &[u8], b: &[u8]) -> bool {
    let mut result = a.len() ^ b.len();
    for (&a, &b) in a.iter().zip(b.iter()) {
        result |= a ^ b;
    }
    result == 0
}

