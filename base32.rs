//
// base32.rs - base32 implementation
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

export base32, mk;

const padd: u8 = 61u8;

iface base32 {
    fn encode(src: [u8]) -> [u8];
    fn decode(src: [u8]) -> [u8];
    fn hex_encode(src: [u8]) -> [u8];
    fn hex_decode(src: [u8]) -> [u8];
}

fn mk() -> base32 {
    type _base32 = {table: [u8], table_hex: [u8]};

    impl of base32 for _base32 {
        fn encode(src: [u8]) -> [u8] {
            b32encode(self.table, src)
        }
        fn decode(src: [u8]) -> [u8] {
            b32decode(self.table, src)
        }
        fn hex_encode(src: [u8]) -> [u8] {
            b32encode(self.table_hex, src)
        }
        fn hex_decode(src: [u8]) -> [u8] {
            b32decode(self.table_hex, src)
        }
    }

    let table = vec::init_elt_mut(32u, 0u8), i = 0u8;
    u8::range(65u8, 91u8)  { |j| table[i] = j; i += 1u8; }
    u8::range(50u8, 56u8)  { |j| table[i] = j; i += 1u8; }

    let table_hex = vec::init_elt_mut(32u, 0u8), i = 0u8;
    u8::range(48u8, 58u8)  { |j| table_hex[i] = j; i += 1u8; }
    u8::range(65u8, 87u8)  { |j| table_hex[i] = j; i += 1u8; }

    {table: vec::from_mut(table),
     table_hex: vec::from_mut(table_hex)} as base32
}

fn b32encode(table: [u8], src: [u8]) -> [u8] {
    let srclen = vec::len(src);
    let targ = if srclen % 5u == 0u {
        vec::init_elt_mut(srclen / 5u * 8u, 0u8)
    } else {
        vec::init_elt_mut((srclen / 5u + 1u) * 8u, 0u8)
    };
    let input = vec::init_elt_mut(5u, 0u8);
    let output = vec::init_elt_mut(8u, 0u8);
    let curr = 0u, src_curr = 0u;

    while srclen > 4u {
        input[0] = src[src_curr];
        input[1] = src[src_curr + 1u];
        input[2] = src[src_curr + 2u];
        input[3] = src[src_curr + 3u];
        input[4] = src[src_curr + 4u];
        srclen -= 5u; src_curr += 5u;

        output[0] = input[0] >> 3u8;
        output[1] = (input[0] & 0x07_u8) << 2u8 | input[1] >> 6u8;
        output[2] = (input[1] & 0x3f_u8) >> 1u8;
        output[3] = (input[1] & 0x01_u8) << 4u8 | input[2] >> 4u8;
        output[4] = (input[2] & 0x0f_u8) << 1u8 | input[3] >> 7u8;
        output[5] = (input[3] & 0x7f_u8) >> 2u8;
        output[6] = (input[3] & 0x03_u8) << 3u8 | input[4] >> 5u8;
        output[7] = input[4] & 0x1f_u8;

        targ[curr + 0u] = table[output[0]];
        targ[curr + 1u] = table[output[1]];
        targ[curr + 2u] = table[output[2]];
        targ[curr + 3u] = table[output[3]];
        targ[curr + 4u] = table[output[4]];
        targ[curr + 5u] = table[output[5]];
        targ[curr + 6u] = table[output[6]];
        targ[curr + 7u] = table[output[7]];
        curr += 8u;
    }

    if srclen != 0u {
        input[0] = 0u8; input[1] = 0u8; input[2] = 0u8;
        input[3] = 0u8; input[4] = 0u8;

        alt srclen {
          1u {input[0] = src[src_curr];}
          2u {input[0] = src[src_curr];
              input[1] = src[src_curr + 1u];}
          3u {input[0] = src[src_curr];
              input[1] = src[src_curr + 1u];
              input[2] = src[src_curr + 2u];}
          4u {input[0] = src[src_curr];
              input[1] = src[src_curr + 1u];
              input[2] = src[src_curr + 2u];
              input[3] = src[src_curr + 3u];}
          _ { }
        }

        output[0] = input[0] >> 3u8;
        output[1] = (input[0] & 0x07_u8) << 2u8 | input[1] >> 6u8;
        output[2] = (input[1] & 0x3f_u8) >> 1u8;
        output[3] = (input[1] & 0x01_u8) << 4u8 | input[2] >> 4u8;
        output[4] = (input[2] & 0x0f_u8) << 1u8 | input[3] >> 7u8;
        output[5] = (input[3] & 0x7f_u8) >> 2u8;
        output[6] = (input[3] & 0x03_u8) << 3u8 | input[4] >> 5u8;

        targ[curr + 0u] = table[output[0]];
        targ[curr + 1u] = table[output[1]];
        targ[curr + 2u] = if srclen > 1u { table[output[2]] } else { padd };
        targ[curr + 3u] = if srclen > 1u { table[output[3]] } else { padd };
        targ[curr + 4u] = if srclen > 2u { table[output[4]] } else { padd };
        targ[curr + 5u] = if srclen > 3u { table[output[5]] } else { padd };
        targ[curr + 6u] = if srclen > 3u { table[output[6]] } else { padd };
        targ[curr + 7u] = padd;
    }

    vec::from_mut(targ)
}

fn b32decode(table: [u8], src: [u8]) -> [u8] {
    let srclen = vec::len(src);

    if srclen % 8u != 0u { fail "malformed base32 string"; }
    if srclen == 0u { ret []; }

    let input = vec::init_elt_mut(8u, 0u8);
    let output = vec::init_elt_mut(8u, 0u8);
    let targ = if src[srclen - 6u] == padd {
        vec::init_elt_mut(srclen / 8u * 5u - 4u, 0u8)
    } else if src[srclen - 4u] == padd {
        vec::init_elt_mut(srclen / 8u * 5u - 3u, 0u8)
    } else if src[srclen - 3u] == padd {
        vec::init_elt_mut(srclen / 8u * 5u - 2u, 0u8)
    } else if src[srclen - 1u] == padd {
        vec::init_elt_mut(srclen / 8u * 5u - 1u, 0u8)
    } else {
        vec::init_elt_mut(srclen / 8u * 5u, 0u8)
    };
    let curr = 0u, src_curr = 0u;

    vec::from_mut(targ)
}

mod tests {
    import std::map;
    import core::str::{bytes, from_bytes};
    enum mode {
        t_encode,
        t_decode,
        t_hex_encode,
        t_hex_decode,
    }
    fn setup(t: mode) -> map::hashmap<str, str> {
        let m = map::new_str_hash::<str>();
        alt t {
          t_decode { }
          t_encode {
            m.insert("fooba", "MZXW6YTB");
            m.insert("foob",  "MZXW6YQ=");
            m.insert("foo",   "MZXW6===");
            m.insert("fo",    "MZXQ====");
            m.insert("f",     "MY======");
          }
          t_hex_decode { }
          t_hex_encode {
            m.insert("fooba", "CPNMUOJ1");
            m.insert("foob",  "CPNMUOG=");
            m.insert("foo",   "CPNMU===");
            m.insert("fo",    "CPNG====");
            m.insert("f",     "CO======");
          }
        }
        m.insert("", "");
        m
    }
    fn do_test(t: mode) {
        let b32 = mk();
        let m = setup(t);
        m.keys { |k|
            let expected = m.get(k);
            let actual = alt t {
              t_decode { from_bytes(b32.decode(bytes(k))) }
              t_encode { from_bytes(b32.encode(bytes(k))) }
              t_hex_decode { from_bytes(b32.hex_decode(bytes(k))) }
              t_hex_encode { from_bytes(b32.hex_encode(bytes(k))) }
            };
            #debug("expected: %s", expected);
            #debug("actual:   %s", actual);
            assert expected == actual;
        }
    }
    #[test]
    fn encode() { do_test(t_encode); }
    #[test]
    fn hex_encode() { do_test(t_hex_encode); }
    #[test]
    fn decode() { do_test(t_decode); }
    #[test]
    fn hex_decode() { do_test(t_hex_decode); }
}
