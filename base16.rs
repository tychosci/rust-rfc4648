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
 * let src = \"base16\";
 * let res = src.encode(encoding::base16);
 * let res = str::from_bytes(res);
 *
 * io::println(#fmt[\"%s\", res]);
 * ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
 */

export base16, encode, decode;

class base16 {
    let table: ~[u8];
    let decode_map: ~[u8];

    new() {
        let table = str::bytes("0123456789ABCDEF");
        let decode_map = vec::to_mut(vec::from_elem(256u, 0xff_u8));

        for u8::range(0u8, 16u8) |i| {
            decode_map[table[i]] = i;
        }
        for u8::range(10u8, 16u8) |i| {
            decode_map[table[i] + 32u8] = i;
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
 * Shortcut for enc#encode_bytes
 *
 * Table of hex alphabet and decode map are allocated
 * every time when this function is called, so it's
 * recommended to use `mk` and then `encode_bytes` instead
 * if it's necessary to use this function many times.
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
 * Shortcut for enc#decode_bytes
 *
 * Table of hex alphabet and decode map are allocated
 * every time when this function is called, so it's
 * recommended to use `mk` and then `decode_bytes` instead
 * if it's necessary to use this function many times.
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
pure fn encoded_len(src_len: uint) -> uint { src_len * 2u }
#[inline(always)]
pure fn decoded_len(src_len: uint) -> uint { src_len / 2u }

fn b16encode(table: &[u8], dst: &[mut u8], src: &[u8]) {
    for uint::range(0u, src.len()) |j| {
        dst[j + 1u * j] = table[src[j] >> 4u8];
        dst[j + 1u + 1u * j] = table[src[j] & 0x0f_u8];
    }
}

fn b16decode(decode_map: &[u8], dst: &[mut u8], src: &[u8]) -> uint {
    let mut src_length = src.len();
    let mut i = 0u;
    let mut j = 0u;

    while src_length > 0u {
        if char::is_whitespace(src[i] as char) {
            src_length -= 1u;
            i += 1u;
            again;
        }

        let chr1 = decode_map[src[i]];
        let chr2 = decode_map[src[i + 1u]];
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
