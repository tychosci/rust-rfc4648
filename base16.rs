/*!
 * Base16 Module
 *
 * See <http://tools.ietf.org/html/rfc4648#section-8> for details.
 *
 * # Example
 *
 * ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
 * use encoding;
 * import encoding::extensions;
 *
 * let src = "base16";
 * let res = src.encode(encoding::base16);
 * let res = str::from_bytes(res);
 *
 * io::println(#fmt["%s", res]);
 * ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
 */

export base16, encode, decode;

class base16 {
    let table: ~[u8];
    let decode_map: ~[u8];

    new() {
        let table = str::bytes("0123456789ABCDEF");
        let decode_map = vec::to_mut(vec::from_elem(256u, 0xff_u8));

        for u8::range(0, 16) |i| {
            decode_map[table[i]] = i;
        }
        for u8::range(10, 16) |i| {
            decode_map[table[i]+32] = i;
        }

        self.table = table;
        self.decode_map = vec::from_mut(decode_map);
    }

    fn encode(dst: &[mut u8], src: &[u8]) {
        b16encode(self.table, dst, src);
    }
    fn decode(dst: &[mut u8], src: &[u8]) -> uint {
        b16decode(self.decode_map, dst, src)
    }

    /**
     * Encode input bytes to hex-encoded bytes.
     *
     * # Arguments
     *
     * * src - bytes for encoding
     *
     * # Return
     *
     * hex-encoded bytes
     */
    fn encode_bytes(src: &[u8]) -> ~[u8] {
        let dst_len = encoded_len(src.len());
        let dst = vec::to_mut(vec::from_elem(dst_len, 0u8));
        self.encode(dst, src);
        vec::from_mut(dst)
    }

    /**
     * Decode hex-encoded bytes to its original bytes.
     *
     * # Arguments
     *
     * * src - hex-encoded bytes
     *
     * # Return
     *
     * decoded bytes
     */
    fn decode_bytes(src: &[u8]) -> ~[u8] {
        let dst_len = decoded_len(src.len());
        let dst = vec::to_mut(vec::from_elem(dst_len, 0u8));
        let end = self.decode(dst, src);
        vec::slice(vec::from_mut(dst), 0u, end)
    }
}

/**
 * Shortcut for base16#encode_bytes
 *
 * # Arguments
 *
 * * src - bytes for encoding
 *
 * # Return
 *
 * hex-encoded bytes
 */
fn encode(src: &[u8]) -> ~[u8] {
    let base16 = base16();
    base16.encode_bytes(src)
}

/**
 * Shortcut for base16#decode_bytes
 *
 * # Arguments
 *
 * * src - hex-encoded bytes
 *
 * # Return
 *
 * decoded bytes
 */
fn decode(src: &[u8]) -> ~[u8] {
    let base16 = base16();
    base16.decode_bytes(src)
}

#[inline(always)]
pure fn encoded_len(src_len: uint) -> uint { src_len * 2 }
#[inline(always)]
pure fn decoded_len(src_len: uint) -> uint { src_len / 2 }

fn b16encode(table: &[u8], dst: &[mut u8], src: &[u8]) {
    for uint::range(0, src.len()) |j| {
        dst[j+1*j]     = table[src[j]>>4];
        dst[j+1*j + 1] = table[src[j] & 0x0f];
    }
}

fn b16decode(decode_map: &[u8], dst: &[mut u8], src: &[u8]) -> uint {
    let mut src_length = src.len();
    let mut i = 0u;
    let mut j = 0u;

    while src_length > 0 {
        if char::is_whitespace(src[i] as char) {
            src_length -= 1;
            i += 1;
            again;
        }

        let chr1 = decode_map[src[i]];
        let chr2 = decode_map[src[i+1]];
        if chr1 == 0xff_u8 || chr2 == 0xff_u8 {
            fail "malformed base16 string";
        }
        dst[j] = chr1<<4 | chr2;

        src_length -= 2;
        i += 2;
        j += 1;
    }

    j
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_encode() {
        let source = str::bytes("foo");
        let expect = str::bytes("666F6F");
        let actual = encode(source);
        assert expect == actual;
    }
    #[test]
    fn test_decode() {
        let source = str::bytes("\t66 6f\r\n 6f");
        let expect = str::bytes("foo");
        let actual = decode(source);
        assert expect == actual;
    }
}
