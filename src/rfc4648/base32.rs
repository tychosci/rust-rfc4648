// base32.rs

use std::vec;

// TODO: doc
pub enum Base32Type {
    Standard,
    Hex,
}

impl Base32Type {
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

static BASE32_STANDARD_TABLE: &'static [u8] =
    bytes!("ABCDEFGHIJKLMNOPQRSTUVWXYZ234567");

static BASE32_HEX_TABLE: &'static [u8] =
    bytes!("0123456789ABCDEFGHIJKLMNOPQRSTUV");

static BASE32_STANDARD_DECODE_MAP: [u8, ..256] = [
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255,  26,  27,  28,  29,  30,  31, 255, 255, 255, 255, 255, 255, 255, 255,
    255,   0,   1,   2,   3,   4,   5,   6,   7,   8,   9,  10,  11,  12,  13,  14,
     15,  16,  17,  18,  19,  20,  21,  22,  23,  24,  25, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
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

static BASE32_HEX_DECODE_MAP: [u8, ..256] = [
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
      0,   1,   2,   3,   4,   5,   6,   7,   8,   9, 255, 255, 255, 255, 255, 255,
    255,  10,  11,  12,  13,  14,  15,  16,  17,  18,  19,  20,  21,  22,  23,  24,
     25,  26,  27,  28,  29,  30,  31, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
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

fn encode(src: &[u8], base32_type: Base32Type) -> ~[u8] {
    let dst_length = (src.len() + 4) / 5 * 8;
    let mut dst = vec::with_capacity(dst_length);

    unsafe {
        vec::raw::set_len(&mut dst, dst_length);
    }

    match base32_type {
        Standard => base32_encode(BASE32_STANDARD_TABLE, dst, src),
        Hex => base32_encode(BASE32_HEX_TABLE, dst, src)
    }

    dst
}

fn decode(src: &[u8], base32_type: Base32Type) -> ~[u8] {
    match decode_result(src, base32_type) {
        Ok(dst) => dst,
        Err(reason) => fail!(reason)
    }
}

fn decode_result(src: &[u8], base32_type: Base32Type) -> Result<~[u8], ~str> {
    let dst_length = src.len() / 8 * 5;
    let mut dst = vec::with_capacity(dst_length);

    unsafe {
        vec::raw::set_len(&mut dst, dst_length);
    }

    let size = match base32_type {
        Standard => base32_decode(BASE32_STANDARD_DECODE_MAP, dst, src),
        Hex => base32_decode(BASE32_HEX_DECODE_MAP, dst, src)
    };

    match size {
        Done(n) => unsafe { vec::raw::set_len(&mut dst, n) },
        Next(n) => unsafe { vec::raw::set_len(&mut dst, n) },
        Fail(_, reason) => return Err(reason)
    }

    Ok(dst)
}

fn base32_encode(table: &[u8], dst: &mut [u8], src: &[u8]) {
    let len = src.len();

    for i in range(0u, (len + 4) / 5) {
        let src_index = 5 * i;
        let dst_index = 8 * i;
        let remain = len - src_index;

        let n = (src[src_index+0] as u64)<<32
            | if remain > 1 { (src[src_index+1] as u64)<<24 } else { 0 }
            | if remain > 2 { (src[src_index+2] as u64)<<16 } else { 0 }
            | if remain > 3 { (src[src_index+3] as u64)<< 8 } else { 0 }
            | if remain > 4 { (src[src_index+4] as u64)     } else { 0 };

        dst[dst_index+0] = table[n>>35 & 0x1f];
        dst[dst_index+1] = table[n>>30 & 0x1f];
        dst[dst_index+2] = if remain > 1 { table[n>>25 & 0x1f] } else { PAD };
        dst[dst_index+3] = if remain > 1 { table[n>>20 & 0x1f] } else { PAD };
        dst[dst_index+4] = if remain > 2 { table[n>>15 & 0x1f] } else { PAD };
        dst[dst_index+5] = if remain > 3 { table[n>>10 & 0x1f] } else { PAD };
        dst[dst_index+6] = if remain > 3 { table[n>> 5 & 0x1f] } else { PAD };
        dst[dst_index+7] = if remain > 4 { table[n     & 0x1f] } else { PAD };
    }
}

fn base32_decode(decode_map: &[u8], dst: &mut [u8], src: &[u8]) -> DecodeSize {
    let len = src.len();

    if len == 0 {
        return Done(0);
    }
    if len < 8 || (len % 8) != 0 {
        return Fail(0, ~"the input length should be divisible by 8");
    }

    let mut end = false;
    let mut leftover = len;
    let mut ndecoded = 0;
    let mut i = 0;

    while leftover > 0 && !end {
        let mut buf = [0xff_u8, ..8];
        let mut buf_len = 8u;

        let mut j = 0u;
        while j < 8 {
            if leftover == 0 {
                return Fail(ndecoded, fail_decode_at(len - leftover - j));
            }
            let c = src[len - leftover];
            leftover -= 1;
            if c == PAD && j >= 2 && leftover < 8 {
                if leftover + j < 8 - 1 {
                    return Fail(ndecoded, fail_decode_at(len));
                }
                for k in range(0u, 8 - 1 - j) {
                    if leftover > k && src[len-leftover+k] != PAD {
                        return Fail(ndecoded, fail_decode_at(len - leftover + k - 1));
                    }
                }
                buf_len = j;
                end = true;
                if buf_len == 1 || buf_len == 3 || buf_len == 6 {
                    return Fail(ndecoded, fail_decode_at(len - leftover - 1));
                }
                break;
            }
            buf[j] = decode_map[c];
            if buf[j] == 0xff {
                return Fail(ndecoded, fail_decode_at(len - leftover - 1));
            }
            j += 1;
        }

        dst[i] = 0;
        dst[i+1] = 0;
        dst[i+2] = 0;
        dst[i+3] = 0;
        dst[i+4] = 0;

        dst[i] |= buf[0]<<3 | buf[1]>>2;
        dst[i+1] |= if buf_len > 2 { buf[1]<<6 | buf[2]<<1 } else { 0 };
        dst[i+1] |= if buf_len > 3 { buf[3]>>4             } else { 0 };
        dst[i+2] |= if buf_len > 3 { buf[3]<<4             } else { 0 };
        dst[i+2] |= if buf_len > 4 { buf[4]>>1             } else { 0 };
        dst[i+3] |= if buf_len > 4 { buf[4]<<7 | buf[5]<<2 } else { 0 };
        dst[i+3] |= if buf_len > 6 { buf[6]>>3             } else { 0 };
        dst[i+4] |= if buf_len > 6 { buf[6]<<5 | buf[7]    } else { 0 };

        i += 5;
        match buf_len {
            2     => ndecoded += 1,
            3 | 4 => ndecoded += 2,
            5     => ndecoded += 3,
            6 | 7 => ndecoded += 4,
            8     => ndecoded += 5,
            _     => fail!(~"malformed base32 string")
        }
    }

    if end { Done(ndecoded) } else { Next(ndecoded) }
}

fn fail_decode_at(n: uint) -> ~str {
    format!("illegal base32 data at input byte {}", n)
}
