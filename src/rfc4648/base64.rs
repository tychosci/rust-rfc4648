// base64.rs

use std::vec;

// TODO: doc
pub enum Base64Type {
    Standard,
    UrlSafe,
}

impl Base64Type {
    // TODO: doc
    pub fn encode(self, src: &[u8]) -> ~[u8] {
        encode(src, self)
    }
    // TODO: doc
    pub fn decode(self, src: &[u8]) -> ~[u8] {
        decode(src, self)
    }
    // TODO: doc
    pub fn decode_result(self, src: &[u8]) -> Result<~[u8], ~str> {
        decode_result(src, self)
    }
}

enum DecodeSize {
    Done(uint),       // on uncontinuable state
    Next(uint),       // on continuable state
    Fail(uint, ~str), // on decode failure
}

static PAD: u8 = 61; // '='

static BASE64_STANDARD_TABLE: &'static [u8] =
    bytes!("ABCDEFGHIJKLMNOPQRSTUVWXYZ",
           "abcdefghijklmnopqrstuvwxyz",
           "0123456789+/");

static BASE64_URLSAFE_TABLE: &'static [u8] =
    bytes!("ABCDEFGHIJKLMNOPQRSTUVWXYZ",
           "abcdefghijklmnopqrstuvwxyz",
           "0123456789-_");

static BASE64_STANDARD_DECODE_MAP: [u8, ..256] = [
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,  62, 255, 255, 255,  63,
     52,  53,  54,  55,  56,  57,  58,  59,  60,  61, 255, 255, 255, 255, 255, 255,
    255,   0,   1,   2,   3,   4,   5,   6,   7,   8,   9,  10,  11,  12,  13,  14,
     15,  16,  17,  18,  19,  20,  21,  22,  23,  24,  25, 255, 255, 255, 255, 255,
    255,  26,  27,  28,  29,  30,  31,  32,  33,  34,  35,  36,  37,  38,  39,  40,
     41,  42,  43,  44,  45,  46,  47,  48,  49,  50,  51, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
];

static BASE64_URLSAFE_DECODE_MAP: [u8, ..256] = [
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,  62, 255, 255,
     52,  53,  54,  55,  56,  57,  58,  59,  60,  61, 255, 255, 255, 255, 255, 255,
    255,   0,   1,   2,   3,   4,   5,   6,   7,   8,   9,  10,  11,  12,  13,  14,
     15,  16,  17,  18,  19,  20,  21,  22,  23,  24,  25, 255, 255, 255, 255,  63,
    255,  26,  27,  28,  29,  30,  31,  32,  33,  34,  35,  36,  37,  38,  39,  40,
     41,  42,  43,  44,  45,  46,  47,  48,  49,  50,  51, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
];

fn encode(src: &[u8], base64_type: Base64Type) -> ~[u8] {
    let dst_length = (src.len() + 2) / 3 * 4;
    let mut dst = vec::with_capacity(dst_length);

    unsafe {
        vec::raw::set_len(&mut dst, dst_length);
    }

    match base64_type {
        Standard => base64_encode(BASE64_STANDARD_TABLE, dst, src),
        UrlSafe => base64_encode(BASE64_URLSAFE_TABLE, dst, src)
    }

    dst
}

fn decode(src: &[u8], base64_type: Base64Type) -> ~[u8] {
    match decode_result(src, base64_type) {
        Ok(dst) => dst,
        Err(reason) => fail!(reason)
    }
}

fn decode_result(src: &[u8], base64_type: Base64Type) -> Result<~[u8], ~str> {
    let dst_length = src.len() / 4 * 3;
    let mut dst = vec::with_capacity(dst_length);

    unsafe {
        vec::raw::set_len(&mut dst, dst_length);
    }

    let size = match base64_type {
        Standard => base64_decode(BASE64_STANDARD_DECODE_MAP, dst, src),
        UrlSafe => base64_decode(BASE64_URLSAFE_DECODE_MAP, dst, src)
    };

    match size {
        Done(n) => unsafe { vec::raw::set_len(&mut dst, n) },
        Next(n) => unsafe { vec::raw::set_len(&mut dst, n) },
        Fail(_, reason) => return Err(reason)
    }

    Ok(dst)
}

fn base64_encode(table: &[u8], dst: &mut [u8], src: &[u8]) {
    let len = src.len();
    let pad = len % 3;
    let mut i = 0;
    let mut j = 0;

    while i < len - pad {
        let n = (src[i] as u32)<<16 | (src[i+1] as u32)<<8 | (src[i+2] as u32);

        dst[j] = table[n>>18 & 0x3f];
        dst[j+1] = table[n>>12 & 0x3f];
        dst[j+2] = table[n>>6  & 0x3f];
        dst[j+3] = table[n     & 0x3f];

        i += 3;
        j += 4;
    }

    let dst = dst.mut_slice_from(j);
    if pad == 1 {
        let n = (src[i] as u32)<<16;
        dst[0] = table[n>>18 & 0x3f];
        dst[1] = table[n>>12 & 0x3f];
        dst[2] = PAD;
        dst[3] = PAD;
    } else if pad == 2 {
        let n = (src[i] as u32)<<16 | (src[i+1] as u32)<<8;
        dst[0] = table[n>>18 & 0x3f];
        dst[1] = table[n>>12 & 0x3f];
        dst[2] = table[n>>6  & 0x3f];
        dst[3] = PAD;
    }
}

fn base64_decode(decode_map: &[u8], dst: &mut [u8], src: &[u8]) -> DecodeSize {
    let len = src.len();

    if len == 0 {
        return Done(0);
    }
    if len < 4 || (len % 4) != 0 {
        return Fail(0, ~"the input length should be divisible by 4");
    }

    let mut end = false;
    let mut leftover = len;
    let mut ndecoded = 0;
    let mut i = 0;

    while leftover > 0 && !end {
        let mut buf = [0xff, ..4];
        let mut buf_len = 4;

        let mut j = 0u;
        while j < 4 {
            if leftover == 0 {
                return Fail(ndecoded, fail_decode_at(len - leftover - j));
            }
            let c = src[len - leftover];
            leftover -= 1;
            if c == PAD && j >= 2 && leftover < 4 {
                if leftover + j < 4 - 1 {
                    return Fail(ndecoded, fail_decode_at(len));
                }
                if leftover > 0 && src[len - leftover] != PAD {
                    return Fail(ndecoded, fail_decode_at(len - leftover - 1));
                }
                buf_len = j;
                end = true;
                break;
            }
            buf[j] = decode_map[c];
            if buf[j] == 0xff {
                return Fail(ndecoded, fail_decode_at(len - leftover - 1));
            }
            j += 1;
        }

        dst[i] = buf[0]<<2 | buf[1]>>4;
        dst[i+1] = if buf_len > 2 { buf[1]<<4 | buf[2]>>2 } else { 0 };
        dst[i+2] = if buf_len > 3 { buf[2]<<6 | buf[3]    } else { 0 };

        i += 3;
        ndecoded += buf_len - 1;
    }

    if end { Done(ndecoded) } else { Next(ndecoded) }
}

fn fail_decode_at(n: uint) -> ~str {
    format!("illegal base64 data at input byte {}", n)
}
