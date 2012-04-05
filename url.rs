//
// url.rs - module for url-encoding
//

import vec::len;

export query_escape, query_unescape;

enum enc_mode { query, path, fragment, userinfo }

fn query_escape(s: str) -> str { url_escape(s, query) }
fn query_unescape(s: str) -> str { url_unescape(s, query) }

#[inline(always)]
pure fn ishex(c: u8) -> bool {
    if 48u8 <= c && c <= 57u8 { true }
    else if 65u8 <= c && c <= 90u8  { true }
    else if 97u8 <= c && c <= 122u8 { true }
    else { false }
}

#[inline(always)]
pure fn unhex(c: u8) -> u8 {
    if 48u8 <= c && c <= 57u8 { c - 48u8 }
    else if 65u8 <= c && c <= 90u8  { c - 65u8 + 10u8 }
    else if 97u8 <= c && c <= 122u8 { c - 97u8 + 10u8 }
    else { fail "should be unreachable"; }
}

pure fn should_escape(c: u8, mode: enc_mode) -> bool {
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

fn url_unescape(s: str, mode: enc_mode) -> str {
    let bs = str::bytes(s);
    let src_length = len(bs);
    let mut n = 0u;
    let mut i = 0u;
    let mut hasplus = false;

    while i < src_length {
        let c = bs[i];
        if c == 37u8 {
            n += 1u;
            if i+2u >= src_length || !ishex(bs[i+1u]) || !ishex(bs[i+2u]) {
                fail #fmt["Invalid URL escape: '%s'", s];
            }
            i += 3u;
        } else if c == 43u8 {
            hasplus = mode == query;
            i += 1u;
        } else {
            i += 1u;
        }
    }

    if n == 0u && !hasplus {
        ret s;
    }

    let ts = vec::to_mut(vec::from_elem(src_length - 2u * n, 0u8));
    let mut i = 0u;
    let mut j = 0u;

    while i < src_length {
        let c = bs[i];
        if c == 37u8 {
            ts[j] = (unhex(bs[i+1u])<<4u) | unhex(bs[i+2u]);
            j += 1u;
            i += 3u;
        } else if c == 43u8 {
            if mode == query { ts[j] = 32u8; }
            else { ts[j] = 43u8; }
            j += 1u;
            i += 1u;
        } else {
            ts[j] = c;
            j += 1u;
            i += 1u;
        }
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
    #[test]
    fn test_query_unescape() {
        assert query_unescape("a") == "a";
        assert query_unescape("a+z") == "a z";
        assert query_unescape("%3f") == "?";
        assert query_unescape("%C3%A5") == "å";
        assert query_unescape("%E3%81%82") == "あ";
    }
}
