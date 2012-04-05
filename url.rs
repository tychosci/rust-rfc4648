//
// url.rs - module for url-encoding
//

enum enc_mode {
    query,
    path,
    fragment,
    userinfo
}

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
}
