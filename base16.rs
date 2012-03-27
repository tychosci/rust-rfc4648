//
// base16.rs - base16 module
//
// The Base 16 Alphabet
//
//  Value Encoding  Value Encoding  Value Encoding  Value Encoding
//      0 0             4 4             8 8            12 C
//      1 1             5 5             9 9            13 D
//      2 2             6 6            10 A            14 E
//      3 3             7 7            11 B            15 F
//

#[doc = "

    Base16 Module

    See <http://tools.ietf.org/html/rfc4648#section-8> for details.

"];

use std;

export mk, enc, encode, decode;

import vec::len;

type enc_t = {table: [u8], decode_map: [u8]};

iface enc {
    fn encode(dst: [mut u8], src: [u8]);
    fn decode(dst: [mut u8], src: [u8]) -> uint;
    #[doc = "
    Encode input bytes to hex-encoded bytes.

    # Arguments

    * src - bytes for encoding

    # Return

    hex-encoded bytes
    "]
    fn encode_bytes(src: [u8]) -> [u8];
    #[doc = "
    Decode hex-encoded bytes to its original bytes.

    # Arguments

    * src - hex-encoded bytes

    # Return

    decoded bytes
    "]
    fn decode_bytes(src: [u8]) -> [u8];
}

impl of enc for enc_t {
    fn encode(dst: [mut u8], src: [u8]) {
        b16encode(self.table, dst, src);
    }
    fn decode(dst: [mut u8], src: [u8]) -> uint {
        b16decode(self.decode_map, dst, src)
    }
    fn encode_bytes(src: [u8]) -> [u8] {
        let dst_len = encoded_len(len(src));
        let dst = vec::to_mut(vec::from_elem(dst_len, 0u8));
        self.encode(dst, src);
        vec::from_mut(dst)
    }
    fn decode_bytes(src: [u8]) -> [u8] {
        let dst_len = decoded_len(len(src));
        let dst = vec::to_mut(vec::from_elem(dst_len, 0u8));
        let end = self.decode(dst, src);
        vec::slice(vec::from_mut(dst), 0u, end)
    }
}

fn mk() -> enc {
    #[doc = "
    Make instance of interface `enc`

    # Return

    instance of interface `enc`
    "];

    let table = str::bytes("0123456789ABCDEF");
    let decode_map = vec::to_mut(vec::from_elem(256u, 0xff_u8));

    let mut i = 0u8;
    while i < 16u8 {
        decode_map[table[i]] = i;
        i += 1u8;
    }

    i -= 6u8;
    while i < 16u8 {
        decode_map[table[i] + 32u8] = i;
        i += 1u8;
    }

    {table: table,
     decode_map: vec::from_mut(decode_map)} as enc
}

fn encode(src: [u8]) -> [u8] {
    #[doc = "
    Shortcut for enc#encode_bytes

    Table of hex alphabet and decode map is allocated
    every time when this function is called, so it's
    recommended to use `mk` and then `encode_bytes` instead
    if it's necessary to use this function many times.

    # Arguments

    * src - bytes for encoding

    # Return

    hex-encoded bytes
    "];

    let enc = mk();
    enc.encode_bytes(src)
}

fn decode(src: [u8]) -> [u8] {
    #[doc = "
    Shortcut for enc#decode_bytes

    Table of hex alphabet and decode map is allocated
    every time when this function is called, so it's
    recommended to use `mk` and then `decode_bytes` instead
    if it's necessary to use this function many times.

    # Arguments

    * src - hex-encoded bytes

    # Return

    decoded bytes
    "];

    let enc = mk();
    enc.decode_bytes(src)
}

fn encoded_len(src_len: uint) -> uint { src_len * 2u }
fn decoded_len(src_len: uint) -> uint { src_len / 2u }

fn b16encode(table: [u8], dst: [mut u8], src: [u8]) {
    let mut src_length = len(src);
    let mut i = 0u;
    let mut j = 0u;

    while src_length > 0u {
        dst[i] = table[src[j] >> 4u8];
        dst[i + 1u] = table[src[j] & 0x0f_u8];

        src_length -= 1u;
        i += 2u;
        j += 1u;
    }
}

fn b16decode(decode_map: [u8], dst: [mut u8], src: [u8]) -> uint {
    let mut src_length = len(src);
    let mut i = 0u;
    let mut j = 0u;
    let mut chr1 = 0u8;
    let mut chr2 = 0u8;

    while src_length > 0u {
        if src[i] == 10u8 || src[j] == 13u8 || src[i] == 32u8 {
            src_length -= 1u;
            i += 1u;
            cont;
        }

        chr1 = decode_map[src[i]];
        chr2 = decode_map[src[i + 1u]];
        if chr1 == 0xff_u8 || chr2 == 0xff_u8 {
            fail "malformed base16 string";
        }
        dst[j] = chr1 << 4u8 | chr2;

        src_length -= 2u;
        i += 2u;
        j += 1u;
    }

    j
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_encode() {
        let src = str::bytes("foo");
        let exp = str::bytes("666F6F");
        let res = encode(src);
        assert res == exp;
    }
    #[test]
    fn test_decode() {
        let src = str::bytes("66 6f 6f");
        let exp = str::bytes("foo");
        let res = decode(src);
        assert res == exp;
    }
}
