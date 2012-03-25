//
// base32n.rs - new base32 module
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

import vec::len;

const PAD: u8 = 61u8;

iface enc {
    fn encode(dst: [mutable u8], src: [u8]);
    fn encode_h(dst: [mutable u8], src: [u8]);
    fn encode_bytes(src: [u8]) -> [u8];
    fn encode_bytes_h(src: [u8]) -> [u8];
    fn decode(dst: [mutable u8], src: [u8]);
    fn decode_h(dst: [mutable u8], src: [u8]);
    fn decode_bytes(src: [u8]) -> [u8];
    fn decode_bytes_h(src: [u8]) -> [u8];
}

fn mk() -> enc {
    type _enc = {table: [u8], table_h: [u8],
                 decode_map: [u8], decode_map_h: [u8]};

    impl of enc for _enc {
        fn encode(dst: [mutable u8], src: [u8]) {
            b32encode(self.table, dst, src);
        }
        fn encode_h(dst: [mutable u8], src: [u8]) {
            b32encode(self.table_h, dst, src);
        }
        fn encode_bytes(src: [u8]) -> [u8] {
            []
        }
        fn encode_bytes_h(src: [u8]) -> [u8] {
            []
        }
        fn decode(dst: [mutable u8], src: [u8]) {
            b32decode(self.decode_map, dst, src);
        }
        fn decode_h(dst: [mutable u8], src: [u8]) {
            b32decode(self.decode_map_h, dst, src);
        }
        fn decode_bytes(src: [u8]) -> [u8] {
            []
        }
        fn decode_bytes_h(src: [u8]) -> [u8] {
            []
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

fn b32encode(table: [u8], dst: [mutable u8], src: [u8]) {
}

fn b32decode(decode_map: [u8], dst: [mutable u8], src: [u8]) {
}

#[cfg(test)]
mod tests {
}
