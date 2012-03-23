//
// base64n.rs - new base64 module
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

use std;

import vec::len;

const PAD: u8 = 61u8;

iface enc {
    fn encode(dst: [mutable u8], src: [u8]);
    fn encode_bytes(src: [u8]) -> [u8];
    fn encode_str(src: str) -> str;
    // FIXME `decode` should return desired length of `dst`
    fn decode(dst: [mutable u8], src: [u8]);
    fn decode_byte(src: [u8]) -> [u8];
}

fn mk() -> enc {
    type _enc = {table: [u8]};

    impl of enc for _enc {
        fn encode(dst: [mutable u8], src: [u8]) {
            b64encode(self.table + [43u8, 47u8], dst, src);
        }
        fn decode(dst: [mutable u8], src: [u8]) {
            // FIXME need self.decode_map instead of self.table.
            b64decode(self.table, dst, src);
        }
        fn encode_bytes(src: [u8]) -> [u8] {
            let dst_length = encoded_len(len(src));
            let dst = vec::to_mut(vec::from_elem(dst_length, 0u8));
            self.encode(dst, src);
            vec::from_mut(dst)
        }
        fn encode_str(src: str) -> str {
            let src = str::bytes(src);
            str::from_bytes(self.encode_bytes(src))
        }
        fn decode_byte(src: [u8]) -> [u8] { [] }
    }

    let table = vec::to_mut(vec::from_elem(62u, 0u8)), i = 0u8;
    u8::range(65u8, 91u8)  { |j| table[i] = j; i += 1u8; }
    u8::range(97u8, 123u8) { |j| table[i] = j; i += 1u8; }
    u8::range(48u8, 58u8)  { |j| table[i] = j; i += 1u8; }

    {table: vec::from_mut(table)} as enc
}

fn encoded_len(src_length: uint) -> uint {
    (src_length + 2u) / 3u * 4u
}

fn decoded_len(src_length: uint) -> uint {
    src_length / 4u * 3u
}

fn b64encode(table: [u8], dst: [mutable u8], src: [u8]) {
    if len(src) == 0u {
        ret;
    }

    let src_length = len(src);
    let dst_length = len(dst);
    let dst_curr = 0u;
    let src_curr = 0u;

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

fn b64decode(table: [u8], dst: [mutable u8], src: [u8]) {
    // FIXME write
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_encode_bytes() {
        let src = [102u8, 111u8, 111u8, 98u8, 97u8, 114u8];
        let enc = mk();
        let res = enc.encode_bytes(src);
        assert res == [90u8, 109u8, 57u8, 118u8, 89u8, 109u8, 70u8, 121u8];
    }
    #[test]
    fn test_encode_str() {
        let src = "foobar";
        let enc = mk();
        let res = enc.encode_str(src);
        assert res == "Zm9vYmFy";
    }
}
