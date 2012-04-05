//
// url.rs - module for url-encoding
//

import vec::len;

enum enc_mode {
    query,
    path,
    fragment,
    userinfo
}

fn query_escape(s: str) -> str { url_escape(s, query) }

fn should_escape(c: u8, mode: enc_mode) -> bool {
    alt c {
      65u8 to 90u8  { ret false; } // A .. Z
      97u8 to 122u8 { ret false; } // a .. z
      48u8 to 57u8  { ret false; } // 0 .. 9
      _ { }
    }

    alt c {
      45u8  | 95u8 | 46u8 | 33u8 | // '-', '_', '.', '!'
      126u8 | 42u8 | 39u8 | 40u8 | // '~', '*', '\'', '('
      41u8 { false }               // ')'

      36u8 | 38u8 | 43u8 | 44u8 |  // '$', '&', '+', ','
      47u8 | 58u8 | 59u8 | 61u8 |  // '/', ':', ';', '='
      63u8 | 64u8 {                // '?', '@'
        alt mode {
          query    { true }
          fragment { false }
          path     { (c == 63u8) }
          userinfo { (c == 47u8 || c == 58u8 || c == 64u8) }
        }
      }

      _ { true }
    }
}

fn url_escape(s: str, mode: enc_mode) -> str {
    let bs = str::bytes(s);
    let src_length = len(bs);
    let mut space_count = 0u;
    let mut hex_count = 0u;
    let table =
        [48u8, 49u8, 50u8, 51u8, 52u8, 53u8, 54u8, 55u8,
         56u8, 57u8, 65u8, 66u8, 67u8, 68u8, 69u8, 70u8];

    uint::range(0u, src_length) {|i|
        let c = bs[i];
        if should_escape(bs[i], mode) {
            if c == 32u8 && mode == query {
                space_count += 1u;
            } else {
                hex_count += 1u;
            }
        }
    }

    // Nothing to do if there's no space and no escapable chars in `s`
    if space_count == 0u && hex_count == 0u {
        ret s;
    }

    let ts = vec::to_mut(vec::from_elem(src_length + 2u * hex_count, 0u8));
    let mut i = 0u;
    let mut j = 0u;
    let mut c = 0u8;

    while i < src_length {
        c = bs[i];
        if c == 32u8 && mode == query {
            ts[j] = '+' as u8;
            j += 1u;
        } else if should_escape(c, mode) {
            ts[j] = '%' as u8;
            ts[j+1u] = table[c >> 4u8];
            ts[j+2u] = table[c & 0x0f_u8];
            j += 3u;
        } else {
            ts[j] = c;
            j += 1u;
        }
        i += 1u;
    }

    str::from_bytes(vec::from_mut(ts))
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_should_escape() {
        assert should_escape('A' as u8, path) == false;
        assert should_escape('0' as u8, path) == false;
        assert should_escape('?' as u8, path) == true;
        assert should_escape('A' as u8, query) == false;
        assert should_escape('0' as u8, query) == false;
        assert should_escape('@' as u8, query) == true;
        assert should_escape('/' as u8, userinfo) == true;
        assert should_escape(':' as u8, userinfo) == true;
        assert should_escape('$' as u8, userinfo) == false;
        assert should_escape('A' as u8, fragment) == false;
        assert should_escape('0' as u8, fragment) == false;
        assert should_escape(';' as u8, fragment) == false;
    }
    #[test]
    fn test_query_escape() {
        assert query_escape("a") == "a";
        assert query_escape("a z") == "a+z";
        assert query_escape("å") == "%C3%A5";
        assert query_escape("?") == "%3F";
        assert query_escape("あ") == "%E3%81%82";
    }
}
