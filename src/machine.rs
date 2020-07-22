fn unit(n: i32) -> i32 {
    if n < 0 {
        -1
    } else {
        1
    }
}

/// Attempts to capture the essence of constructing an i32 from a series of digits.
fn i32(n: i32, ch: u8) -> i32 {
    n * 10 + (ch as i32) * unit(n)
}
