//
// base64.rs - base64 implementation
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

import std::io;
import std::map;

export base64, mk;

const padd: u8 = 61u8;

iface base64 {
    fn encode(src: [u8]) -> [u8];
    fn decode(src: [u8]) -> [u8];
    fn decode2(src: [u8]) -> [u8];
    fn urlsafe_encode(src: [u8]) -> [u8];
    fn urlsafe_decode(src: [u8]) -> [u8];
    fn urlsafe_decode2(src: [u8]) -> [u8];
}

fn mk() -> base64 {
    type _base64 = {table: [u8]};

    impl of base64 for _base64 {
        fn encode(src: [u8]) -> [u8] {
            b64encode(self.table + [43u8, 47u8], src)
        }
        fn decode(src: [u8]) -> [u8] {
            b64decode(self.table + [43u8, 47u8], src)
        }
        fn urlsafe_encode(src: [u8]) -> [u8] {
            b64encode(self.table + [45u8, 95u8], src)
        }
        fn urlsafe_decode(src: [u8]) -> [u8] {
            b64decode(self.table + [45u8, 95u8], src)
        }
        fn decode2(src: [u8]) -> [u8] {
            b64decode2(self.table + [43u8, 47u8], src)
        }
        fn urlsafe_decode2(src: [u8]) -> [u8] {
            b64decode2(self.table + [45u8, 95u8], src)
        }
    }

    let table = vec::init_elt_mut(62u, 0u8), i = 0u8;
    u8::range(65u8, 91u8)  { |j| table[i] = j; i += 1u8; }
    u8::range(97u8, 123u8) { |j| table[i] = j; i += 1u8; }
    u8::range(48u8, 58u8)  { |j| table[i] = j; i += 1u8; }

    {table: vec::from_mut(table)} as base64
}

fn b64idx(x: u8, y: u8, z: u8) -> u8 {
    if 65u8 <= x && x <= 90u8 { x - 65u8 }
    else if 97u8 <= x && x <= 122u8 { x - 97u8 + 26u8 }
    else if 48u8 <= x && x <= 57u8  { x - 48u8 + 52u8 }
    else if x == y { 62u8 }
    else if x == z { 63u8 }
    else { fail "malformed base64 string"; }
}

fn b64encode(table: [u8], src: [u8]) -> [u8] {
    let srclen = vec::len(src);

    let targ: [mutable u8] = [mutable];
    let input = vec::init_elt_mut(3u, 0u8);
    let output = vec::init_elt_mut(4u, 0u8);
    let curr = 0u, src_curr = 0u;
    let targlen = if srclen % 3u == 0u {
        (srclen / 3u) * 4u
    } else {
        (srclen / 3u + 1u) * 4u
    };
    vec::reserve(targ, targlen);
    unsafe { vec::unsafe::set_len(targ, targlen); }

    while srclen > 2u {
        input[0] = src[src_curr];
        input[1] = src[src_curr + 1u];
        input[2] = src[src_curr + 2u];
        srclen -= 3u; src_curr += 3u;

        output[0] = input[0] >> 2u8;
        output[1] = ((input[0] &  3u8) << 4u8) | (input[1] >> 4u8);
        output[2] = ((input[1] & 15u8) << 2u8) | (input[2] >> 6u8);
        output[3] = input[2] & 63u8;

        targ[curr + 0u] = table[output[0]];
        targ[curr + 1u] = table[output[1]];
        targ[curr + 2u] = table[output[2]];
        targ[curr + 3u] = table[output[3]];
        curr += 4u;
    }

    if srclen != 0u {
        input[0] = 0u8; input[1] = 0u8; input[2] = 0u8;

        alt srclen {
          1u {input[0] = src[src_curr];}
          2u {input[0] = src[src_curr];
              input[1] = src[src_curr + 1u];}
          _  { }
        }

        output[0] = input[0] >> 2u8;
        output[1] = ((input[0] &  3u8) << 4u8) | (input[1] >> 4u8);
        output[2] = ((input[1] & 15u8) << 2u8) | (input[2] >> 6u8);

        targ[curr + 0u] = table[output[0]];
        targ[curr + 1u] = table[output[1]];
        targ[curr + 2u] = if srclen == 1u { padd } else { table[output[2]] };
        targ[curr + 3u] = padd;
    }

    vec::from_mut(targ)
}

// strict
fn b64decode(table: [u8], src: [u8]) -> [u8] {
    let srclen = vec::len(src);

    if srclen % 4u != 0u { fail "malformed base64 string"; }
    if srclen == 0u { ret []; }

    let targ: [mutable u8] = [mutable];
    let input  = vec::init_elt_mut(4u, 0u8);
    let output = vec::init_elt_mut(4u, 0u8);
    let curr = 0u, src_curr = 0u;
    let targlen =
        if src[srclen - 2u] == padd && src[srclen - 1u] == padd {
            srclen / 4u * 3u - 2u
        } else if src[srclen - 1u] == padd {
            srclen / 4u * 3u - 1u
        } else {
            srclen / 4u * 3u
        };
    vec::reserve(targ, targlen);
    unsafe { vec::unsafe::set_len(targ, targlen); }

    while srclen > 4u {
        input[0] = src[src_curr];
        input[1] = src[src_curr + 1u];
        input[2] = src[src_curr + 2u];
        input[3] = src[src_curr + 3u];
        srclen -= 4u; src_curr += 4u;

        output[0] = b64idx(input[0], table[62], table[63]);
        output[1] = b64idx(input[1], table[62], table[63]);
        output[2] = b64idx(input[2], table[62], table[63]);
        output[3] = b64idx(input[3], table[62], table[63]);

        targ[curr + 0u] = (output[0] << 2u8) | (output[1] >> 4u8);
        targ[curr + 1u] = ((output[1] & 15u8) << 4u8) | (output[2] >> 2u8);
        targ[curr + 2u] = ((output[2] &  3u8) << 6u8) | output[3];
        curr += 3u;
    }

    if srclen == 4u {
        input[0] = src[src_curr];
        input[1] = src[src_curr + 1u];
        input[2] = src[src_curr + 2u];
        input[3] = src[src_curr + 3u];

        output[0] = b64idx(input[0], table[62], table[63]);
        output[1] = b64idx(input[1], table[62], table[63]);

        targ[curr + 0u] = (output[0] << 2u8) | (output[1] >> 4u8);

        if input[2] == padd { ret vec::from_mut(targ); }

        output[2] = b64idx(input[2], table[62], table[63]);

        targ[curr + 1u] = ((output[1] & 15u8) << 4u8) | (output[2] >> 2u8);

        if input[3] == padd { ret vec::from_mut(targ); }

        output[3] = b64idx(input[3], table[62], table[63]);

        targ[curr + 2u] = ((output[2] & 3u8) << 6u8) | output[3];
    }

    vec::from_mut(targ)
}

// lenient
fn b64decode2(table: [u8], src: [u8]) -> [u8] {
    let srclen = vec::len(src);

    if srclen == 0u { ret []; }

    let targ: [mutable u8] = [mutable];
    let end = false, i = 0, in = 0u8;
    let output = vec::init_elt_mut(4u, 0u8), outlen = 4;
    let src_curr = 0u, curr = 0u;
    let targlen =
        if src[srclen - 2u] == padd && src[srclen - 1u] == padd {
            srclen / 4u * 3u - 2u
        } else if src[srclen - 1u] == padd {
            srclen / 4u * 3u - 1u
        } else {
            srclen / 4u * 3u
        };
    vec::reserve(targ, targlen);
    unsafe { vec::unsafe::set_len(targ, targlen); }

    while srclen > 0u && !end {
        i = 0;
        while i < 4 {
            if srclen == 0u {
                fail "malformed base64 string";
            }
            in = src[src_curr]; srclen -= 1u; src_curr += 1u;
            // ignore whitespace, tab, "\r", "\n"
            if in == 13u8 || in == 10u8 ||
               in == 32u8 || in == 9u8 {
                cont;
            }
            if in == padd && i >= 2 && srclen < 4u {
                if srclen > 0u && src[src_curr] != padd {
                    fail "malformed base64 string";
                }
                outlen = i; end = true;
                break;
            }
            output[i] = b64idx(in, table[62], table[63]);
            i += 1;
        }
        if outlen == 4 {
            targ[curr + 0u] = (output[0] << 2u8) | (output[1] >> 4u8);
            targ[curr + 1u] = ((output[1] & 15u8) << 4u8) | (output[2] >> 2u8);
            targ[curr + 2u] = ((output[2] &  3u8) << 6u8) | output[3];
            curr += 3u;
        } else if outlen == 3 {
            targ[curr + 0u] = (output[0] << 2u8) | (output[1] >> 4u8);
            targ[curr + 1u] = ((output[1] & 15u8) << 4u8) | (output[2] >> 2u8);
            curr += 2u;
        } else if outlen == 2 {
            targ[curr + 0u] = (output[0] << 2u8) | (output[1] >> 4u8);
            curr += 1u;
        }
    }

    let result: [mutable u8] = [mutable];
    vec::reserve(result, curr);
    vec::as_buf(result) { |dest|
        vec::as_buf(targ) { |src|
            if dest != ptr::null() && curr > 0u {
                unsafe { ptr::memcpy(dest, src, curr); }
            }
        }
    }
    unsafe { vec::unsafe::set_len(result, curr); }

    vec::from_mut(result)
}

// FIXME write
mod stream {
    type encoder = {
        buf: [mutable u8],
        out: [mutable u8],
    };
    type decoder = {
        buf: [mutable u8],
        out: [mutable u8],
    };
    // encode with io::reader (buffer, file, sockets)
    fn mk_encoder(ei: io::reader) {
    }
    // decode with io::reader (buffer, file, sockets)
    fn mk_decoder(ei: io::reader) {
    }
}

#[cfg(test)]
mod tests {
    import core::str::{from_bytes, bytes};
    enum mode {
        t_decode,
        t_encode,
        t_urlsafe_encode,
        t_urlsafe_decode,
        t_decode2,
        t_urlsafe_decode2,
    }
    fn setup(t: mode) -> map::hashmap<str, str> {
        let m = map::new_str_hash::<str>();
        m.insert("", "");
        alt t {
          t_decode2 | t_urlsafe_decode2 {
            m.insert("cGxl    YXN1cmUu", "pleasure.");
            m.insert("bGVhc 3VyZS4=",  "leasure.");
            m.insert("Z W F z d X J l L g==",   "easure.");
            m.insert("YXN1c\r\nmUu",        "asure.");
            m.insert("c3Vy\tZS4=",         "sure.");
          }
          t_decode | t_urlsafe_decode {
            m.insert("cGxlYXN1cmUu", "pleasure.");
            m.insert("bGVhc3VyZS4=",  "leasure.");
            m.insert("ZWFzdXJlLg==",   "easure.");
            m.insert("YXN1cmUu",        "asure.");
            m.insert("c3VyZS4=",         "sure.");
          }
           t_encode | t_urlsafe_encode {
            m.insert("pleasure.", "cGxlYXN1cmUu");
            m.insert("leasure.",  "bGVhc3VyZS4=");
            m.insert("easure.",   "ZWFzdXJlLg==");
            m.insert("asure.",        "YXN1cmUu");
            m.insert("sure.",         "c3VyZS4=");
          }
        }
        m
    }
    fn do_test(t: mode) {
        let b64 = mk();
        let m = setup(t);
        m.keys { |k|
            let expected = m.get(k);
            let actual = alt t {
              t_decode { from_bytes(b64.decode(bytes(k))) }
              t_encode { from_bytes(b64.encode(bytes(k))) }
              t_urlsafe_decode { from_bytes(b64.urlsafe_decode(bytes(k))) }
              t_urlsafe_encode { from_bytes(b64.urlsafe_encode(bytes(k))) }
              t_decode2 { from_bytes(b64.decode2(bytes(k))) }
              t_urlsafe_decode2 { from_bytes(b64.urlsafe_decode2(bytes(k))) }
            };
            #debug("  expected: %s", expected);
            #debug("  actual:   %s", actual);
            assert expected == actual;
        }
    }
    #[test]
    fn decode() { do_test(t_decode); }
    #[test]
    fn encode() { do_test(t_encode); }
    #[test]
    fn urlsafe_decode() { do_test(t_urlsafe_decode); }
    #[test]
    fn urlsafe_encode() { do_test(t_urlsafe_encode); }
    #[test]
    fn decode2() { do_test(t_decode2); }
    #[test]
    fn urlsafe_decode2() { do_test(t_urlsafe_decode2); }
}
