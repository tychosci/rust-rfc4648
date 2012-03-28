// util.rs

#[inline(always)]
fn range(start: uint, end: uint, f: fn(uint) -> bool) {
    let mut i = start;
    while i < end {
        if !f(i) { break; }
        i += 1u;
    }
}
