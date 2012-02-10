//
// base64.rs - base64 implementation
//
// The Base64 Alphabet
//
//  Value Encoding  Value Encoding  Value Encoding  Value Encoding
//   0 A            17 R            34 i            51 z
//   1 B            18 S            35 j            52 0
//   2 C            19 T            36 k            53 1
//   3 D            20 U            37 l            54 2
//   4 E            21 V            38 m            55 3
//   5 F            22 W            39 n            56 4
//   6 G            23 X            40 o            57 5
//   7 H            24 Y            41 p            58 6
//   8 I            25 Z            42 q            59 7
//   9 J            26 a            43 r            60 8
//  10 K            27 b            44 s            61 9
//  11 L            28 c            45 t            62 +
//  12 M            29 d            46 u            63 /
//  13 N            30 e            47 v
//  14 O            31 f            48 w         (pad) =
//  15 P            32 g            49 x
//  16 Q            33 h            50 y
//

use std;

import std::map;

export base64, mk_base64;

iface base64 {
    fn encode(src: [u8]) -> [u8];
    fn decode(src: [u8]) -> [u8];
    fn urlsafe_encode(src: [u8]) -> [u8];
    fn urlsafe_decode(src: [u8]) -> [u8];
}

fn mk_base64() -> base64 {
    type _base64 = {padd: u8, mutable table: [u8]};
    impl of base64 for _base64 {
        fn encode(src: [u8]) -> [u8] {
            let srclen = vec::len(src);
            let targ = if srclen % 3u == 0u {
                vec::init_elt_mut((srclen / 3u) * 4u, 0u8)
            } else {
                vec::init_elt_mut((srclen / 3u + 1u) * 4u, 0u8)
            };
            let input = vec::init_elt_mut(3u, 0u8);
            let output = vec::init_elt_mut(4u, 0u8);
            let curr = 0u, src_curr = 0u;
            let table = self.table;

            while srclen > 2u {
                input[0] = src[src_curr];
                input[1] = src[src_curr + 1u];
                input[2] = src[src_curr + 2u];
                srclen -= 3u; src_curr += 3u;

                output[0] = input[0] >> 2u8;
                output[1] = ((input[0] &  3u8) << 4u8) | (input[1] >> 4u8);
                output[2] = ((input[1] & 15u8) << 2u8) | (input[2] >> 6u8);
                output[3] = input[2] & 63u8;

                targ[curr] = table[output[0]]; curr += 1u;
                targ[curr] = table[output[1]]; curr += 1u;
                targ[curr] = table[output[2]]; curr += 1u;
                targ[curr] = table[output[3]]; curr += 1u;
            }

            if srclen != 0u {
                input[0] = 0u8; input[1] = 0u8; input[2] = 0u8;

                alt srclen {
                  1u { input[0] = src[src_curr]; }
                  2u { input[0] = src[src_curr];
                       input[1] = src[src_curr + 1u]; }
                  _  { }
                }

                output[0] = input[0] >> 2u8;
                output[1] = ((input[0] &  3u8) << 4u8) | (input[1] >> 4u8);
                output[2] = ((input[1] & 15u8) << 2u8) | (input[2] >> 6u8);

                targ[curr] = table[output[0]]; curr += 1u;
                targ[curr] = table[output[1]]; curr += 1u;
                targ[curr] = if srclen == 1u {
                    self.padd
                } else {
                    table[output[2]]
                };
                curr += 1u; targ[curr] = self.padd;
            }

            vec::from_mut(targ)
        }
        fn decode(src: [u8]) -> [u8] {
            let srclen = vec::len(src);

            if srclen % 4u != 0u { fail "malformed base64 string"; }
            if srclen == 0u { ret []; }

            let input  = vec::init_elt_mut(4u, 0u8);
            let output = vec::init_elt_mut(4u, 0u8);
            let targ = if src[srclen - 2u] == self.padd {
                vec::init_elt_mut(srclen / 4u * 3u - 2u, 0u8)
            } else if src[srclen - 1u] == self.padd {
                vec::init_elt_mut(srclen / 4u * 3u - 1u, 0u8)
            } else {
                vec::init_elt_mut(srclen / 4u * 3u, 0u8)
            };
            let curr = 0u, src_curr = 0u;
            let table = self.table;

            while srclen > 4u {
                input[0] = src[src_curr];
                input[1] = src[src_curr + 1u];
                input[2] = src[src_curr + 2u];
                input[3] = src[src_curr + 3u];
                srclen -= 4u; src_curr += 4u;

                output[0] = idx(table, input[0]);
                if output[0] == 64u8 { fail "malformed base64 string"; }

                output[1] = idx(table, input[1]);
                if output[1] == 64u8 { fail "malformed base64 string"; }

                output[2] = idx(table, input[2]);
                if output[2] == 64u8 { fail "malformed base64 string"; }

                output[3] = idx(table, input[3]);
                if output[3] == 64u8 { fail "malformed base64 string"; }

                targ[curr] = (output[0] << 2u8) | (output[1] >> 4u8);
                curr += 1u;

                targ[curr] =
                    ((output[1] & 15u8) << 4u8) | (output[2] >> 2u8);
                curr += 1u;

                targ[curr] = ((output[2] &  3u8) << 6u8) | output[3];
                curr += 1u;
            }

            if srclen == 4u {
                input[0] = src[src_curr];
                input[1] = src[src_curr + 1u];
                input[2] = src[src_curr + 2u];
                input[3] = src[src_curr + 3u];

                output[0] = idx(table, input[0]);
                if output[0] == 64u8 { fail "malformed base64 string"; }

                output[1] = idx(table, input[1]);
                if output[1] == 64u8 { fail "malformed base64 string"; }

                targ[curr] = (output[0] << 2u8) | (output[1] >> 4u8);
                curr += 1u;

                if input[2] == self.padd { ret vec::from_mut(targ); }

                output[2] = idx(table, input[2]);
                if output[2] == 64u8 { fail "malformed base64 string"; }

                targ[curr] = ((output[1] & 15u8) << 4u8) | (output[2] >> 2u8);
                curr += 1u;

                if input[3] == self.padd { ret vec::from_mut(targ); }

                output[3] = idx(table, input[3]);
                if output[3] == 64u8 { fail "malformed base64 string"; }

                targ[curr] = ((output[2] & 3u8) << 6u8) | output[3];
            }

            vec::from_mut(targ)
        }
        fn urlsafe_encode(src: [u8]) -> [u8] {
            let tmp = self.table;
            self.table = vec::map(tmp) { |i|
                if i == 43u8 { 45u8 }
                else if i == 47u8 { 95u8 }
                else { i }
            };
            let res = self.encode(src);
            self.table = tmp;
            res
        }
        fn urlsafe_decode(src: [u8]) -> [u8] {
            let tmp = self.table;
            self.table = vec::map(tmp) { |i|
                if i == 43u8 { 45u8 }
                else if i == 47u8 { 95u8 }
                else { i }
            };
            let res = self.decode(src);
            self.table = tmp;
            res
        }
    }

    let table = vec::init_elt_mut(64u, 0u8), i = 0u8;
    u8::range(65u8, 91u8)  { |j| table[i] = j; i += 1u8; }
    u8::range(97u8, 123u8) { |j| table[i] = j; i += 1u8; }
    u8::range(48u8, 58u8)  { |j| table[i] = j; i += 1u8; }
    table[i] = 43u8; i += 1u8;
    table[i] = 47u8; i += 1u8;

    {padd: 61u8, mutable table: vec::from_mut(table)} as base64
}

fn idx(elems: [const u8], x: u8) -> u8 {
    let i = 0u8;
    while i < 64u8 {
        if elems[i] == x { break; }
        i += 1u8;
    }
    ret i;
}

#[cfg(test)]
mod tests {
    import core::str::{from_bytes, bytes};
    enum mode {
        t_decode,
        t_encode,
        t_urlsafe_encode,
        t_urlsafe_decode
    }
    fn setup(t: mode) -> map::hashmap<str, str> {
        let m = map::new_str_hash::<str>();
        m.insert("", "");
        alt t {
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
        let b64 = mk_base64();
        let m = setup(t);
        m.keys { |k|
            let expected = m.get(k);
            let actual = alt t {
              t_decode { from_bytes(b64.decode(bytes(k))) }
              t_encode { from_bytes(b64.encode(bytes(k))) }
              t_urlsafe_decode { from_bytes(b64.urlsafe_decode(bytes(k))) }
              t_urlsafe_encode { from_bytes(b64.urlsafe_encode(bytes(k))) }
            };
            #debug("expected: %s", expected);
            #debug("actual:   %s", actual);
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
}
