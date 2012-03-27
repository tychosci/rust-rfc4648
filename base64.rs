//
// base64.rs - base64 module
//
// The Base64 Alphabet
//
//  Value Encoding  Value Encoding  Value Encoding  Value Encoding
//      0 A            17 R            34 i            51 z
//      1 B            18 S            35 j            52 0
//      2 C            19 T            36 k            53 1
//      3 D            20 U            37 l            54 2
//      4 E            21 V            38 m            55 3
//      5 F            22 W            39 n            56 4
//      6 G            23 X            40 o            57 5
//      7 H            24 Y            41 p            58 6
//      8 I            25 Z            42 q            59 7
//      9 J            26 a            43 r            60 8
//     10 K            27 b            44 s            61 9
//     11 L            28 c            45 t            62 +
//     12 M            29 d            46 u            63 /
//     13 N            30 e            47 v
//     14 O            31 f            48 w         (pad) =
//     15 P            32 g            49 x
//     16 Q            33 h            50 y
//
// The "URL and Filename safe" Base 64 Alphabet
//
// Value Encoding  Value Encoding  Value Encoding  Value Encoding
//     0 A            17 R            34 i            51 z
//     1 B            18 S            35 j            52 0
//     2 C            19 T            36 k            53 1
//     3 D            20 U            37 l            54 2
//     4 E            21 V            38 m            55 3
//     5 F            22 W            39 n            56 4
//     6 G            23 X            40 o            57 5
//     7 H            24 Y            41 p            58 6
//     8 I            25 Z            42 q            59 7
//     9 J            26 a            43 r            60 8
//    10 K            27 b            44 s            61 9
//    11 L            28 c            45 t            62 - (minus)
//    12 M            29 d            46 u            63 _ (underline)
//    13 N            30 e            47 v
//    14 O            31 f            48 w
//    15 P            32 g            49 x
//    16 Q            33 h            50 y         (pad) =
//

#[doc = "

    Base64 module

    See <http://tools.ietf.org/html/rfc4648#section-4> for details.

"];

use std;

export mk, enc, encode, encode_u, decode, decode_u;

import vec::len;

const PAD: u8 = 61u8;

type enc_t = {
    table: [u8],
    table_u: [u8],
    decode_map: [u8],
    decode_map_u: [u8]
};

iface enc {
    fn encode(dst: [mut u8], src: [u8]);
    fn encode_u(dst: [mut u8], src: [u8]);
    fn decode(dst: [mut u8], src: [u8]) -> uint;
    fn decode_u(dst: [mut u8], src: [u8]) -> uint;
    #[doc = "
    Encode input bytes to base64-encoded bytes.

    # Arguments

    * src - bytes for encoding

    # Return

    base64-encoded bytes
    "]
    fn encode_bytes(src: [u8]) -> [u8];
    #[doc = "
    Encode input bytes to base64-encoded bytes.

    Note that this method is for url and filename safe base64 encoding.
    See <http://tools.ietf.org/html/rfc4648#section-5> for details.

    # Arguments

    * src - bytes for encoding

    # Return

    base64-encoded bytes
    "]
    fn encode_bytes_u(src: [u8]) -> [u8];
    #[doc = "
    Encode given string to base64-encoded string

    # Arguments

    * src - string for encoding

    # Return

    base64-encoded string
    "]
    fn encode_str(src: str) -> str;
    #[doc = "
    Encode given string to base64-encoded string

    Note that this method is for url and filename safe base64 encoding.
    See <http://tools.ietf.org/html/rfc4648#section-5> for details.

    # Arguments

    * src - string for encoding

    # Return

    base64-encoded string
    "]
    fn encode_str_u(src: str) -> str;
    #[doc = "
    Decode base64-encoded bytes to its original bytes.

    # Arguments

    * src - base64-encoded bytes

    # Return

    decoded bytes
    "]
    fn decode_bytes(src: [u8]) -> [u8];
    #[doc = "
    Decode base64-encoded bytes to its original bytes.

    Note that this method is for url and filename safe base64 encoding.
    See <http://tools.ietf.org/html/rfc4648#section-5> for details.

    # Arguments

    * src - base64-encoded bytes

    # Return

    decoded bytes
    "]
    fn decode_bytes_u(src: [u8]) -> [u8];
}

impl of enc for enc_t {
    fn encode(dst: [mut u8], src: [u8]) {
        b64encode(self.table, dst, src);
    }
    fn encode_u(dst: [mut u8], src: [u8]) {
        b64encode(self.table_u, dst, src);
    }
    fn decode(dst: [mut u8], src: [u8]) -> uint {
        b64decode(self.decode_map, dst, src)
    }
    fn decode_u(dst: [mut u8], src: [u8]) -> uint {
        b64decode(self.decode_map_u, dst, src)
    }
    fn encode_bytes(src: [u8]) -> [u8] {
        let dst_length = encoded_len(len(src));
        let dst = vec::to_mut(vec::from_elem(dst_length, 0u8));
        self.encode(dst, src);
        vec::from_mut(dst)
    }
    fn encode_bytes_u(src: [u8]) -> [u8] {
        let dst_length = encoded_len(len(src));
        let dst = vec::to_mut(vec::from_elem(dst_length, 0u8));
        self.encode_u(dst, src);
        vec::from_mut(dst)
    }
    fn encode_str(src: str) -> str {
        let src = str::bytes(src);
        str::from_bytes(self.encode_bytes(src))
    }
    fn encode_str_u(src: str) -> str {
        let src = str::bytes(src);
        str::from_bytes(self.encode_bytes_u(src))
    }
    fn decode_bytes(src: [u8]) -> [u8] {
        let dst_length = decoded_len(len(src));
        let dst = vec::to_mut(vec::from_elem(dst_length, 0u8));
        let res = self.decode(dst, src);
        vec::slice(vec::from_mut(dst), 0u, res)
    }
    fn decode_bytes_u(src: [u8]) -> [u8] {
        let dst_length = decoded_len(len(src));
        let dst = vec::to_mut(vec::from_elem(dst_length, 0u8));
        let res = self.decode_u(dst, src);
        vec::slice(vec::from_mut(dst), 0u, res)
    }
}

fn mk() -> enc {
    #[doc = "
    Make instance of interface `enc`

    # Return

    instance of interface `enc`
    "];

    let mut i = 0u8;
    let table = vec::to_mut(vec::from_elem(64u, 0u8));
    u8::range(65u8, 91u8)  { |j| table[i] = j; i += 1u8; }
    u8::range(97u8, 123u8) { |j| table[i] = j; i += 1u8; }
    u8::range(48u8, 58u8)  { |j| table[i] = j; i += 1u8; }
    table[i] = 43u8; table[i + 1u8] = 47u8;

    let table_u = table;
    table_u[i] = 45u8; table_u[i + 1u8] = 95u8;

    let decode_map = vec::to_mut(vec::from_elem(256u, 0xff_u8));
    let decode_map_u = vec::to_mut(vec::from_elem(256u, 0xff_u8));

    i = 0u8;
    while i < 64u8 {
        decode_map[table[i]] = i;
        i += 1u8;
    }

    i = 0u8;
    while i < 64u8 {
        decode_map_u[table_u[i]] = i;
        i += 1u8;
    }

    {table: vec::from_mut(table),
     table_u: vec::from_mut(table_u),
     decode_map: vec::from_mut(decode_map),
     decode_map_u: vec::from_mut(decode_map_u)} as enc
}

fn encode(src: [u8]) -> [u8] {
    #[doc = "
    Shortcut for enc#encode_bytes

    Table of base64 alphabet and decode map is allocated
    every time when this function is called, so it's
    recommended to use `mk` and then `encode_bytes` instead
    if it's necessary to use this function many times.

    # Arguments

    * src - bytes for encoding

    # Return

    base64-encoded bytes
    "];

    let enc = mk();
    enc.encode_bytes(src)
}

fn encode_u(src: [u8]) -> [u8] {
    #[doc = "
    Shortcut for enc#encode_bytes_u

    Table of base64 alphabet and decode map is allocated
    every time when this function is called, so it's
    recommended to use `mk` and then `encode_bytes_u` instead
    if it's necessary to use this function many times.

    # Arguments

    * src - bytes for encoding

    # Return

    base64-encoded bytes (url and filename safe)
    "];

    let enc = mk();
    enc.encode_bytes_u(src)
}

fn decode(src: [u8]) -> [u8] {
    #[doc = "
    Shortcut for enc#decode_bytes

    Table of base64 alphabet and decode map is allocated
    every time when this function is called, so it's
    recommended to use `mk` and then `decode_bytes` instead
    if it's necessary to use this function many times.

    # Arguments

    * src - base64-encoded bytes

    # Return

    decoded bytes
    "];

    let enc = mk();
    enc.decode_bytes(src)
}

fn decode_u(src: [u8]) -> [u8] {
    #[doc = "
    Shortcut for enc#decode_bytes_u

    Table of base64 alphabet and decode map is allocated
    every time when this function is called, so it's
    recommended to use `mk` and then `decode_bytes_u` instead
    if it's necessary to use this function many times.

    # Arguments

    * src - base64-encoded bytes

    # Return

    decoded bytes
    "];

    let enc = mk();
    enc.decode_bytes_u(src)
}

fn encoded_len(src_length: uint) -> uint {
    (src_length + 2u) / 3u * 4u
}

fn decoded_len(src_length: uint) -> uint {
    src_length / 4u * 3u
}

fn b64encode(table: [u8], dst: [mut u8], src: [u8]) {
    if len(src) == 0u {
        ret;
    }

    let mut src_length = len(src);
    let mut dst_length = len(dst);
    let mut dst_curr = 0u;
    let mut src_curr = 0u;

    if dst_length % 4u != 0u {
        fail "dst's length should be divisible by 4";
    }

    while src_length > 0u {
        dst[dst_curr + 0u] = 0u8;
        dst[dst_curr + 1u] = 0u8;
        dst[dst_curr + 2u] = 0u8;
        dst[dst_curr + 3u] = 0u8;

        if src_length == 1u {
            dst[dst_curr + 0u] |= (src[src_curr + 0u]) >> 2u8;
            dst[dst_curr + 1u] |= (src[src_curr + 0u] << 4u8) & 0x3f_u8;
        } else if src_length == 2u {
            dst[dst_curr + 0u] |= (src[src_curr + 0u]) >> 2u8;
            dst[dst_curr + 1u] |= (src[src_curr + 0u] << 4u8) & 0x3f_u8;
            dst[dst_curr + 1u] |= (src[src_curr + 1u] >> 4u8);
            dst[dst_curr + 2u] |= (src[src_curr + 1u] << 2u8) & 0x3f_u8;
        } else {
            dst[dst_curr + 0u] |= (src[src_curr + 0u]) >> 2u8;
            dst[dst_curr + 1u] |= (src[src_curr + 0u] << 4u8) & 0x3f_u8;
            dst[dst_curr + 1u] |= (src[src_curr + 1u] >> 4u8);
            dst[dst_curr + 2u] |= (src[src_curr + 1u] << 2u8) & 0x3f_u8;
            dst[dst_curr + 2u] |= (src[src_curr + 2u] >> 6u8);
            dst[dst_curr + 3u] |= (src[src_curr + 2u]) & 0x3f_u8;
        }

        dst[dst_curr + 0u] = table[dst[dst_curr + 0u]];
        dst[dst_curr + 1u] = table[dst[dst_curr + 1u]];
        dst[dst_curr + 2u] = table[dst[dst_curr + 2u]];
        dst[dst_curr + 3u] = table[dst[dst_curr + 3u]];

        if src_length < 3u {
            dst[dst_curr + 3u] = PAD;
            if src_length < 2u {
                dst[dst_curr + 2u] = PAD;
            }
            break;
        }

        src_length -= 3u;
        src_curr += 3u;
        dst_curr += 4u;
    }
}

fn b64decode(decode_map: [u8], dst: [mut u8], src: [u8]) -> uint {
    let buf = vec::to_mut(vec::from_elem(4u, 0u8));
    let mut src_length = len(src);
    let mut src_curr = 0u;
    let mut dst_curr = 0u;
    let mut src_temp = 0u;
    let mut buf_len = 4u;
    let mut end = false;
    let mut chr = 0u8;
    let mut i = 0u;

    while src_length > 0u && !end {
        buf[0] = 0xff_u8; buf[1] = 0xff_u8;
        buf[2] = 0xff_u8; buf[3] = 0xff_u8;

        i = 0u;
        while i < 4u {
            if src_length == 0u {
                fail "malformed base64 string";
            }
            chr = src[src_temp]; src_temp += 1u;
            if chr == 10u8 || chr == 13u8 {
                cont;
            }
            if chr == PAD && i >= 2u && src_length <= 4u {
                if src_length > 0u && src[src_temp - 1u] != PAD {
                    fail "malformed base64 string";
                }
                buf_len = i;
                end = true;
                break;
            }
            buf[i] = decode_map[chr];
            if buf[i] == 0xff_u8 {
                fail "malformed base64 string";
            }
            i += 1u;
        }

        if buf_len == 2u {
            dst[dst_curr + 0u] = buf[0] << 2u8 | buf[1] >> 4u8;
        } else if buf_len == 3u {
            dst[dst_curr + 0u] = buf[0] << 2u8 | buf[1] >> 4u8;
            dst[dst_curr + 1u] = (buf[1] & 0x0f_u8) << 4u8 | buf[2] >> 2u8;
        } else {
            dst[dst_curr + 0u] = buf[0] << 2u8 | buf[1] >> 4u8;
            dst[dst_curr + 1u] = (buf[1] & 0x0f_u8) << 4u8 | buf[2] >> 2u8;
            dst[dst_curr + 2u] = (buf[2] & 0x03_u8) << 6u8 | buf[3];
        }

        src_length -= 4u;
        dst_curr += buf_len - 1u;
        src_curr = src_temp;
        src_temp = src_curr;
    }

    dst_curr
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_encode_bytes() {
        let src = ["", "f", "fo", "foo", "foob", "fooba", "foobar"];
        let exp = ["", "Zg==", "Zm8=", "Zm9v",
                   "Zm9vYg==", "Zm9vYmE=", "Zm9vYmFy"];
        let src = vec::map(src) {|e| str::bytes(e) };
        let exp = vec::map(exp) {|e| str::bytes(e) };
        let enc = mk();
        let last = len(src);
        let mut i = 0u;

        while i < last {
            let res = enc.encode_bytes(src[i]);
            assert res == exp[i];
            i += 1u;
        }
    }
    #[test]
    fn test_encode_str() {
        let src = ["", "f", "fo", "fo>", "foob", "fooba", "fo?ba?"];
        let exp = ["", "Zg==", "Zm8=", "Zm8+",
                   "Zm9vYg==", "Zm9vYmE=", "Zm8/YmE/"];
        let enc = mk();
        let last = len(src);
        let mut i = 0u;

        while i < last {
            let res = enc.encode_str(src[i]);
            assert res == exp[i];
            i += 1u;
        }
    }
    #[test]
    fn test_encode_bytes_u() {
        let src = ["", "f", "fo", "fo>", "foob", "fooba", "fo?ba?"];
        let exp = ["", "Zg==", "Zm8=", "Zm8-",
                   "Zm9vYg==", "Zm9vYmE=", "Zm8_YmE_"];
        let src = vec::map(src) {|e| str::bytes(e) };
        let exp = vec::map(exp) {|e| str::bytes(e) };
        let enc = mk();
        let last = len(src);
        let mut i = 0u;

        while i < last {
            let res = enc.encode_bytes_u(src[i]);
            assert res == exp[i];
            i += 1u;
        }
    }
    #[test]
    fn test_encode_str_u() {
        let src = ["", "f", "fo", "fo>", "foob", "fooba", "fo?ba?"];
        let exp = ["", "Zg==", "Zm8=", "Zm8-",
                   "Zm9vYg==", "Zm9vYmE=", "Zm8_YmE_"];
        let enc = mk();
        let last = len(src);
        let mut i = 0u;

        while i < last {
            let res = enc.encode_str_u(src[i]);
            assert res == exp[i];
            i += 1u;
        }
    }
    #[test]
    fn test_decode_bytes() {
        let src = ["", "Zg==", "Zm8=", "Zm8+",
                   "Zm9vYg==", "Zm9vYmE=", "Zm8/YmE/"];
        let exp = ["", "f", "fo", "fo>", "foob", "fooba", "fo?ba?"];
        let src = vec::map(src) {|e| str::bytes(e) };
        let exp = vec::map(exp) {|e| str::bytes(e) };
        let enc = mk();
        let last = len(src);
        let mut i = 0u;

        while i < last {
            let res = enc.decode_bytes(src[i]);
            assert res == exp[i];
            i += 1u;
        }
    }
    #[test]
    fn test_decode_bytes_u() {
        let src = ["", "Zg==", "Zm8=", "Zm8-",
                   "Zm9vYg==", "Zm9vYmE=", "Zm8_YmE_"];
        let exp = ["", "f", "fo", "fo>", "foob", "fooba", "fo?ba?"];
        let src = vec::map(src) {|e| str::bytes(e) };
        let exp = vec::map(exp) {|e| str::bytes(e) };
        let enc = mk();
        let last = len(src);
        let mut i = 0u;

        while i < last {
            let res = enc.decode_bytes_u(src[i]);
            assert res == exp[i];
            i += 1u;
        }
    }
}
