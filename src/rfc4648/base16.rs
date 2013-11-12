// base16.rs

use std::vec;

enum DecodeSize {
    Done(uint),       // on success
    Fail(uint, ~str), // on failure
}

static BASE16_TABLE: &'static [u8] = bytes!("0123456789ABCDEF");

static BASE16_DECODE_MAP: [u8, ..256] = [
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
      0,   1,   2,   3,   4,   5,   6,   7,   8,   9, 255, 255, 255, 255, 255, 255,
    255,  10,  11,  12,  13,  14,  15, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255,  10,  11,  12,  13,  14,  15, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
];

// TODO: doc
pub fn encode(src: &[u8]) -> ~[u8] {
    let dst_length = src.len() * 2;
    let mut dst = vec::with_capacity(dst_length);
    unsafe { vec::raw::set_len(&mut dst, dst_length); }
    base16_encode(BASE16_TABLE, dst, src);
    dst
}

// TODO: doc
pub fn decode(src: &[u8]) -> ~[u8] {
    match decode_result(src) {
        Ok(dst) => dst,
        Err(reason) => fail!(reason)
    }
}

// TODO: doc
pub fn decode_result(src: &[u8]) -> Result<~[u8], ~str> {
    let dst_length = src.len() / 2;
    let mut dst = vec::with_capacity(dst_length);
    unsafe { vec::raw::set_len(&mut dst, dst_length); }
    match base16_decode(BASE16_DECODE_MAP, dst, src) {
        Done(_) => Ok(dst),
        Fail(_, reason) => Err(reason)
    }
}

fn base16_encode(table: &[u8], dst: &mut [u8], src: &[u8]) {
    for i in range(0u, src.len()) {
        dst[i+1*i] = table[src[i]>>4];
        dst[i+1*i + 1] = table[src[i] & 0x0f];
    }
}

fn base16_decode(decode_map: &[u8], dst: &mut [u8], src: &[u8]) -> DecodeSize {
    if src.len() % 2 == 1 {
        return Fail(0, ~"odd length base16 data");
    }

    for i in range(0, src.len()/2) {
        let a = decode_map[src[i*2]];
        if (a == 0xff) { return Fail(0, fail_decode_on(src[i*2])); }
        let b = decode_map[src[i*2+1]];
        if (b == 0xff) { return Fail(0, fail_decode_on(src[i*2+1])); }
        dst[i] = a<<4 | b;
    }

    Done(src.len()/2)
}

fn fail_decode_on(b: u8) -> ~str {
    format!("illegal base16 byte {}", b)
}
