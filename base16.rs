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

use std;

export mk, enc, encode, decode;

import vec::len;

iface enc {
    fn encode(dst: [mutable u8], src: [u8]);
    fn encode_bytes(src: [u8]) -> [u8];
    fn decode(dst: [mutable u8], src: [u8]) -> uint;
    fn decode_bytes(src: [u8]) -> [u8];
}

fn mk() -> enc {
    type _enc = {table: [u8], decode_map: [u8]};

    impl of enc for _enc {
        fn encode(dst: [mutable u8], src: [u8]) {
            b16encode(self.table, dst, src);
        }
        fn encode_bytes(src: [u8]) -> [u8] {
            let dst_len = encoded_len(len(src));
            let dst = vec::to_mut(vec::from_elem(dst_len, 0u8));
            self.encode(dst, src);
            vec::from_mut(dst)
        }
        fn decode(dst: [mutable u8], src: [u8]) -> uint {
            b16decode(self.decode_map, dst, src)
        }
        fn decode_bytes(src: [u8]) -> [u8] {
            let dst_len = decoded_len(len(src));
            let dst = vec::to_mut(vec::from_elem(dst_len, 0u8));
            let end = self.decode(dst, src);
            vec::slice(vec::from_mut(dst), 0u, end)
        }
    }

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
    let enc = mk();
    enc.encode_bytes(src)
}

fn decode(src: [u8]) -> [u8] {
    let enc = mk();
    enc.decode_bytes(src)
}

fn encoded_len(src_len: uint) -> uint { src_len * 2u }
fn decoded_len(src_len: uint) -> uint { src_len / 2u }

fn b16encode(table: [u8], dst: [mutable u8], src: [u8]) {
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

fn b16decode(decode_map: [u8], dst: [mutable u8], src: [u8]) -> uint {
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
