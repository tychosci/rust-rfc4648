//
// base32.rs - base32 module
//
// The Base32 Alphabet
//
// Value Encoding  Value Encoding  Value Encoding  Value Encoding
//     0 A             9 J            18 S            27 3
//     1 B            10 K            19 T            28 4
//     2 C            11 L            20 U            29 5
//     3 D            12 M            21 V            30 6
//     4 E            13 N            22 W            31 7
//     5 F            14 O            23 X
//     6 G            15 P            24 Y         (pad) =
//     7 H            16 Q            25 Z
//     8 I            17 R            26 2
//
// The "Extended Hex" Base 32 Alphabet
//
// Value Encoding  Value Encoding  Value Encoding  Value Encoding
//     0 0             9 9            18 I            27 R
//     1 1            10 A            19 J            28 S
//     2 2            11 B            20 K            29 T
//     3 3            12 C            21 L            30 U
//     4 4            13 D            22 M            31 V
//     5 5            14 E            23 N
//     6 6            15 F            24 O         (pad) =
//     7 7            16 G            25 P
//     8 8            17 H            26 Q
//

use std;

export mk, enc, encode, encode_h, decode, decode_h;

import vec::len;

const PAD: u8 = 61u8;

iface enc {
    fn encode(dst: [mut u8], src: [u8]);
    fn encode_h(dst: [mut u8], src: [u8]);
    fn encode_bytes(src: [u8]) -> [u8];
    fn encode_bytes_h(src: [u8]) -> [u8];
    fn encode_str(src: str) -> str;
    fn encode_str_h(src: str) -> str;
    fn decode(dst: [mut u8], src: [u8]) -> uint;
    fn decode_h(dst: [mut u8], src: [u8]) -> uint;
    fn decode_bytes(src: [u8]) -> [u8];
    fn decode_bytes_h(src: [u8]) -> [u8];
}

fn mk() -> enc {
    type _enc = {table: [u8], table_h: [u8],
                 decode_map: [u8], decode_map_h: [u8]};

    impl of enc for _enc {
        fn encode(dst: [mut u8], src: [u8]) {
            b32encode(self.table, dst, src);
        }
        fn encode_h(dst: [mut u8], src: [u8]) {
            b32encode(self.table_h, dst, src);
        }
        fn encode_bytes(src: [u8]) -> [u8] {
            let dst_length = encoded_len(len(src));
            let dst = vec::to_mut(vec::from_elem(dst_length, 0u8));
            self.encode(dst, src);
            vec::from_mut(dst)
        }
        fn encode_bytes_h(src: [u8]) -> [u8] {
            let dst_length = encoded_len(len(src));
            let dst = vec::to_mut(vec::from_elem(dst_length, 0u8));
            self.encode_h(dst, src);
            vec::from_mut(dst)
        }
        fn encode_str(src: str) -> str {
            let src = str::bytes(src);
            str::from_bytes(self.encode_bytes(src))
        }
        fn encode_str_h(src: str) -> str {
            let src = str::bytes(src);
            str::from_bytes(self.encode_bytes_h(src))
        }
        fn decode(dst: [mut u8], src: [u8]) -> uint {
            b32decode(self.decode_map, dst, src)
        }
        fn decode_h(dst: [mut u8], src: [u8]) -> uint {
            b32decode(self.decode_map_h, dst, src)
        }
        fn decode_bytes(src: [u8]) -> [u8] {
            let dst_length = decoded_len(len(src));
            let dst = vec::to_mut(vec::from_elem(dst_length, 0u8));
            let res = self.decode(dst, src);
            vec::slice(vec::from_mut(dst), 0u, res)
        }
        fn decode_bytes_h(src: [u8]) -> [u8] {
            let dst_length = decoded_len(len(src));
            let dst = vec::to_mut(vec::from_elem(dst_length, 0u8));
            let res = self.decode_h(dst, src);
            vec::slice(vec::from_mut(dst), 0u, res)
        }
    }

    let mut i = 0u8;
    let table = vec::to_mut(vec::from_elem(32u, 0u8));
    u8::range(65u8, 91u8) { |j| table[i] = j; i += 1u8; }
    u8::range(50u8, 56u8) { |j| table[i] = j; i += 1u8; }

    i = 0u8;
    let table_h = vec::to_mut(vec::from_elem(32u, 0u8));
    u8::range(48u8, 58u8) { |j| table_h[i] = j; i += 1u8; }
    u8::range(65u8, 87u8) { |j| table_h[i] = j; i += 1u8; }

    let decode_map = vec::to_mut(vec::from_elem(256u, 0xff_u8));
    let decode_map_h = vec::to_mut(vec::from_elem(256u, 0xff_u8));

    i = 0u8;
    while i < 32u8 {
        decode_map[table[i]] = i;
        i += 1u8;
    }

    i = 0u8;
    while i < 32u8 {
        decode_map_h[table_h[i]] = i;
        i += 1u8;
    }

    {table: vec::from_mut(table),
     table_h: vec::from_mut(table_h),
     decode_map: vec::from_mut(decode_map),
     decode_map_h: vec::from_mut(decode_map_h)} as enc
}

fn encode(src: [u8]) -> [u8] {
    let enc = mk();
    enc.encode_bytes(src)
}

fn encode_h(src: [u8]) -> [u8] {
    let enc = mk();
    enc.encode_bytes_h(src)
}

fn decode(src: [u8]) -> [u8] {
    let enc = mk();
    enc.decode_bytes(src)
}

fn decode_h(src: [u8]) -> [u8] {
    let enc = mk();
    enc.decode_bytes_h(src)
}

fn encoded_len(src_length: uint) -> uint {
    (src_length + 4u) / 5u * 8u
}

fn decoded_len(src_length: uint) -> uint {
    src_length / 8u * 5u
}

fn b32encode(table: [u8], dst: [mut u8], src: [u8]) {
    if len(src) == 0u {
        ret;
    }

    let mut src_length = len(src);
    let mut dst_length = len(dst);
    let mut dst_curr = 0u;
    let mut src_curr = 0u;

    if dst_length % 8u != 0u {
        fail "dst's length should be divisible by 8";
    }

    while src_length > 0u {
        dst[dst_curr + 0u] = 0u8;
        dst[dst_curr + 1u] = 0u8;
        dst[dst_curr + 2u] = 0u8;
        dst[dst_curr + 3u] = 0u8;
        dst[dst_curr + 4u] = 0u8;
        dst[dst_curr + 5u] = 0u8;
        dst[dst_curr + 6u] = 0u8;
        dst[dst_curr + 7u] = 0u8;

        if src_length == 1u {
            dst[dst_curr + 0u] |= src[src_curr + 0u] >> 3u8;
            dst[dst_curr + 1u] |= (src[src_curr + 0u] << 2u8) & 0x1f_u8;
        } else if src_length == 2u {
            dst[dst_curr + 0u] |= src[src_curr + 0u] >> 3u8;
            dst[dst_curr + 1u] |= (src[src_curr + 0u] << 2u8) & 0x1f_u8;
            dst[dst_curr + 1u] |= (src[src_curr + 1u] >> 6u8) & 0x1f_u8;
            dst[dst_curr + 2u] |= (src[src_curr + 1u] >> 1u8) & 0x1f_u8;
            dst[dst_curr + 3u] |= (src[src_curr + 1u] << 4u8) & 0x1f_u8;
        } else if src_length == 3u {
            dst[dst_curr + 0u] |= src[src_curr + 0u] >> 3u8;
            dst[dst_curr + 1u] |= (src[src_curr + 0u] << 2u8) & 0x1f_u8;
            dst[dst_curr + 1u] |= (src[src_curr + 1u] >> 6u8) & 0x1f_u8;
            dst[dst_curr + 2u] |= (src[src_curr + 1u] >> 1u8) & 0x1f_u8;
            dst[dst_curr + 3u] |= (src[src_curr + 1u] << 4u8) & 0x1f_u8;
            dst[dst_curr + 3u] |= (src[src_curr + 2u] >> 4u8) & 0x1f_u8;
            dst[dst_curr + 4u] |= (src[src_curr + 2u] << 1u8) & 0x1f_u8;
        } else if src_length == 4u {
            dst[dst_curr + 0u] |= src[src_curr + 0u] >> 3u8;
            dst[dst_curr + 1u] |= (src[src_curr + 0u] << 2u8) & 0x1f_u8;
            dst[dst_curr + 1u] |= (src[src_curr + 1u] >> 6u8) & 0x1f_u8;
            dst[dst_curr + 2u] |= (src[src_curr + 1u] >> 1u8) & 0x1f_u8;
            dst[dst_curr + 3u] |= (src[src_curr + 1u] << 4u8) & 0x1f_u8;
            dst[dst_curr + 3u] |= (src[src_curr + 2u] >> 4u8) & 0x1f_u8;
            dst[dst_curr + 4u] |= (src[src_curr + 2u] << 1u8) & 0x1f_u8;
            dst[dst_curr + 4u] |= (src[src_curr + 3u] >> 7u8);
            dst[dst_curr + 5u] |= (src[src_curr + 3u] >> 2u8) & 0x1f_u8;
            dst[dst_curr + 6u] |= (src[src_curr + 3u] << 3u8) & 0x1f_u8;
        } else {
            dst[dst_curr + 0u] |= src[src_curr + 0u] >> 3u8;
            dst[dst_curr + 1u] |= (src[src_curr + 0u] << 2u8) & 0x1f_u8;
            dst[dst_curr + 1u] |= (src[src_curr + 1u] >> 6u8) & 0x1f_u8;
            dst[dst_curr + 2u] |= (src[src_curr + 1u] >> 1u8) & 0x1f_u8;
            dst[dst_curr + 3u] |= (src[src_curr + 1u] << 4u8) & 0x1f_u8;
            dst[dst_curr + 3u] |= (src[src_curr + 2u] >> 4u8) & 0x1f_u8;
            dst[dst_curr + 4u] |= (src[src_curr + 2u] << 1u8) & 0x1f_u8;
            dst[dst_curr + 4u] |= (src[src_curr + 3u] >> 7u8);
            dst[dst_curr + 5u] |= (src[src_curr + 3u] >> 2u8) & 0x1f_u8;
            dst[dst_curr + 6u] |= (src[src_curr + 3u] << 3u8) & 0x1f_u8;
            dst[dst_curr + 6u] |= (src[src_curr + 4u] >> 5u8);
            dst[dst_curr + 7u] |= src[src_curr + 4u] & 0x1f_u8;
        }

        dst[dst_curr + 0u] = table[dst[dst_curr + 0u]];
        dst[dst_curr + 1u] = table[dst[dst_curr + 1u]];
        dst[dst_curr + 2u] = table[dst[dst_curr + 2u]];
        dst[dst_curr + 3u] = table[dst[dst_curr + 3u]];
        dst[dst_curr + 4u] = table[dst[dst_curr + 4u]];
        dst[dst_curr + 5u] = table[dst[dst_curr + 5u]];
        dst[dst_curr + 6u] = table[dst[dst_curr + 6u]];
        dst[dst_curr + 7u] = table[dst[dst_curr + 7u]];

        if src_length < 5u {
            dst[dst_curr + 7u] = PAD;
            if src_length < 4u {
                dst[dst_curr + 6u] = PAD;
                dst[dst_curr + 5u] = PAD;
                if src_length < 3u {
                    dst[dst_curr + 4u] = PAD;
                    if src_length < 2u {
                        dst[dst_curr + 3u] = PAD;
                        dst[dst_curr + 2u] = PAD;
                    }
                }
            }
            break;
        }

        src_length -= 5u;
        src_curr += 5u;
        dst_curr += 8u;
    }
}

fn b32decode(decode_map: [u8], dst: [mut u8], src: [u8]) -> uint {
    let buf = vec::to_mut(vec::from_elem(8u, 0u8));
    let mut src_length = len(src);
    let mut src_curr = 0u;
    let mut dst_curr = 0u;
    let mut src_temp = 0u;
    let mut buf_len = 8u;
    let mut end = false;
    let mut chr = 0u8;
    let mut i = 0u;

    while src_length > 0u && !end {
        buf[0] = 0xff_u8; buf[1] = 0xff_u8;
        buf[2] = 0xff_u8; buf[3] = 0xff_u8;
        buf[4] = 0xff_u8; buf[5] = 0xff_u8;
        buf[6] = 0xff_u8; buf[7] = 0xff_u8;

        i = 0u;
        while i < 8u {
            if src_length == 0u {
                fail "malformed base32 string";
            }
            chr = src[src_temp]; src_temp += 1u;
            if chr == 10u8 || chr == 13u8 {
                cont;
            }
            if chr == PAD && i >= 2u && src_length <= 8u {
                let mut j = 0u;
                while j < (8u - i - 1u) {
                    if src_length > j && src[src_temp + j - 1u] != PAD {
                        fail "malformed base32 string";
                    }
                    j += 1u;
                }
                buf_len = i;
                end = true;
                break;
            }
            buf[i] = decode_map[chr];
            if buf[i] == 0xff_u8 {
                fail "malformed base32 string";
            }
            i += 1u;
        }

        alt buf_len {
          2u {
            dst[dst_curr + 0u] = buf[0u] << 3u8 | buf[1u] >> 2u8;
          }
          3u {
            dst[dst_curr + 0u] = buf[0u] << 3u8 | buf[1u] >> 2u8;
            dst[dst_curr + 1u] = (buf[1u] & 0x03_u8) << 6u8 | buf[2u] << 1u8;
          }
          4u {
            dst[dst_curr + 0u] = buf[0u] << 3u8 | buf[1u] >> 2u8;
            dst[dst_curr + 1u] = (buf[1u] & 0x03_u8) << 6u8 | buf[2u] << 1u8;
            dst[dst_curr + 1u] |= buf[3u] >> 4u8;
            dst[dst_curr + 2u] = (buf[3u] & 0x0f_u8) << 4u8;
          }
          5u | 6u {
            dst[dst_curr + 0u] = buf[0u] << 3u8 | buf[1u] >> 2u8;
            dst[dst_curr + 1u] = (buf[1u] & 0x03_u8) << 6u8 | buf[2u] << 1u8;
            dst[dst_curr + 1u] |= buf[3u] >> 4u8;
            dst[dst_curr + 2u] = (buf[3u] & 0x0f_u8) << 4u8;
            dst[dst_curr + 2u] |= buf[4u] >> 1u8;
            dst[dst_curr + 3u] = (buf[4u] & 0x01_u8) << 7u8 | buf[5u] << 2u8;
          }
          7u | 8u {
            dst[dst_curr + 0u] = buf[0u] << 3u8 | buf[1u] >> 2u8;
            dst[dst_curr + 1u] = (buf[1u] & 0x03_u8) << 6u8 | buf[2u] << 1u8;
            dst[dst_curr + 1u] |= buf[3u] >> 4u8;
            dst[dst_curr + 2u] = (buf[3u] & 0x0f_u8) << 4u8;
            dst[dst_curr + 2u] |= buf[4u] >> 1u8;
            dst[dst_curr + 3u] = (buf[4u] & 0x01_u8) << 7u8 | buf[5u] << 2u8;
            dst[dst_curr + 3u] |= buf[6u] >> 3u8;
            dst[dst_curr + 4u] = (buf[6u] & 0x07_u8) << 5u8 | buf[7u];
          }
          _ { fail "malformed base32 string"; }
        }

        src_length -= 8u;
        src_curr = src_temp;
        src_temp = src_curr;

        alt buf_len {
          2u      { dst_curr += 1u; }
          3u | 4u { dst_curr += 2u; }
          5u      { dst_curr += 3u; }
          6u | 7u { dst_curr += 4u; }
          8u      { dst_curr += 5u; }
          _       { fail "malformed base32 string"; }
        }
    }

    dst_curr
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_encode_bytes() {
        let src0 = ["", "f", "fo", "foo", "foob", "fooba", "foobar"];
        let exp0 = ["", "MY======", "MZXQ====", "MZXW6===",
                    "MZXW6YQ=", "MZXW6YTB", "MZXW6YTBOI======"];
        let src = vec::map(src0)  {|e| str::bytes(e) };
        let exp = vec::map(exp0) {|e| str::bytes(e) };
        let enc = mk();
        let last = len(src);
        let mut i = 0u;

        while i < last {
            let res = enc.encode_bytes(src[i]);
            #debug("res = %?", res);
            assert exp[i] == res;
            i += 1u;
        }
    }
    #[test]
    fn test_encode_bytes_h() {
        let src0 = ["", "f", "fo", "foo", "foob", "fooba", "foobar"];
        let exp0 = ["", "CO======", "CPNG====", "CPNMU===",
                    "CPNMUOG=", "CPNMUOJ1", "CPNMUOJ1E8======"];
        let src = vec::map(src0)  {|e| str::bytes(e) };
        let exp = vec::map(exp0) {|e| str::bytes(e) };
        let enc = mk();
        let last = len(src);
        let mut i = 0u;

        while i < last {
            let res = enc.encode_bytes_h(src[i]);
            #debug("res = %?", res);
            assert exp[i] == res;
            i += 1u;
        }
    }
    #[test]
    fn test_encode_str() {
        let src = ["", "f", "fo", "foo", "foob", "fooba", "foobar"];
        let exp = ["", "MY======", "MZXQ====", "MZXW6===",
                   "MZXW6YQ=", "MZXW6YTB", "MZXW6YTBOI======"];
        let enc = mk();
        let last = len(src);
        let mut i = 0u;

        while i < last {
            let res = enc.encode_str(src[i]);
            #debug("res = %?", res);
            assert exp[i] == res;
            i += 1u;
        }
    }
    #[test]
    fn test_encode_str_h() {
        let src = ["", "f", "fo", "foo", "foob", "fooba", "foobar"];
        let exp = ["", "CO======", "CPNG====", "CPNMU===",
                   "CPNMUOG=", "CPNMUOJ1", "CPNMUOJ1E8======"];
        let enc = mk();
        let last = len(src);
        let mut i = 0u;

        while i < last {
            let res = enc.encode_str_h(src[i]);
            #debug("res = %?", res);
            assert exp[i] == res;
            i += 1u;
        }
    }
    #[test]
    fn test_decode_bytes() {
        let src = ["", "MY======", "MZXQ====", "MZXW6===",
                   "MZXW6YQ=", "MZXW6YTB", "MZXW6YTBOI======"];
        let exp = ["", "f", "fo", "foo", "foob", "fooba", "foobar"];
        let src = vec::map(src) {|e| str::bytes(e) };
        let exp = vec::map(exp) {|e| str::bytes(e) };
        let enc = mk();
        let last = len(src);
        let mut i = 0u;

        while i < last {
            let res = enc.decode_bytes(src[i]);
            #debug("res = %?", res);
            assert exp[i] == res;
            i += 1u;
        }
    }
    #[test]
    fn test_decode_bytes_h() {
        let src = ["", "CO======", "CPNG====", "CPNMU===",
                   "CPNMUOG=", "CPNMUOJ1", "CPNMUOJ1E8======"];
        let exp = ["", "f", "fo", "foo", "foob", "fooba", "foobar"];
        let src = vec::map(src) {|e| str::bytes(e) };
        let exp = vec::map(exp) {|e| str::bytes(e) };
        let enc = mk();
        let last = len(src);
        let mut i = 0u;

        while i < last {
            let res = enc.decode_bytes_h(src[i]);
            #debug("res = %?", res);
            assert exp[i] == res;
            i += 1u;
        }
    }
}
