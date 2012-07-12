/*!
 * Base64 module
 *
 * See <http://tools.ietf.org/html/rfc4648#section-4> for details.
 *
 * # Example
 *
 * ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
 * use encoding;
 * import encoding::extensions;
 *
 * let src = \"base64\";
 * let res = src.encode(encoding::base64);
 * let res = str::from_bytes(res);
 *
 * io::println(#fmt[\"%s\", res]);
 * ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
 */

export base64, encode, urlsafe_encode, decode, urlsafe_decode;

const PAD: u8 = 61u8;

class base64 {
    let table: ~[u8];
    let table_u: ~[u8];
    let decode_map: ~[u8];
    let decode_map_u: ~[u8];

    new() {
        let table =
            str::bytes("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/");
        let table_u =
            str::bytes("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_");

        let decode_map = vec::to_mut(vec::from_elem(256u, 0xff_u8));
        let decode_map_u = vec::to_mut(vec::from_elem(256u, 0xff_u8));

        for u8::range(0u8, 64u8) |i| {
            decode_map[table[i]] = i;
        }
        for u8::range(0u8, 64u8) |i| {
            decode_map_u[table_u[i]] = i;
        }

        self.table = table;
        self.table_u = table_u;
        self.decode_map = vec::from_mut(decode_map);
        self.decode_map_u = vec::from_mut(decode_map_u);
    }

    fn encode(dst: &[mut u8], src: &[u8]) {
        b64encode(self.table, dst, src);
    }
    fn encode_u(dst: &[mut u8], src: &[u8]) {
        b64encode(self.table_u, dst, src);
    }
    fn decode(dst: &[mut u8], src: &[u8]) -> uint {
        b64decode(self.decode_map, dst, src)
    }
    fn decode_u(dst: &[mut u8], src: &[u8]) -> uint {
        b64decode(self.decode_map_u, dst, src)
    }

    /**
     * Encode input bytes to base64-encoded bytes.
     *
     * # Arguments
     *
     * * src - bytes for encoding
     *
     * # Return
     *
     * base64-encoded bytes
     */
    fn encode_bytes(src: &[u8]) -> ~[u8] {
        let dst_length = encoded_len(src.len());
        let dst = vec::to_mut(vec::from_elem(dst_length, 0u8));
        self.encode(dst, src);
        vec::from_mut(dst)
    }

    /**
     * Encode input bytes to base64-encoded bytes.
     *
     * Note that this method is for url and filename safe base64 encoding.
     * See <http://tools.ietf.org/html/rfc4648#section-5> for details.
     *
     * # Arguments
     *
     * * src - bytes for encoding
     *
     * # Return
     *
     * base64-encoded bytes
     */
    fn encode_bytes_u(src: &[u8]) -> ~[u8] {
        let dst_length = encoded_len(src.len());
        let dst = vec::to_mut(vec::from_elem(dst_length, 0u8));
        self.encode_u(dst, src);
        vec::from_mut(dst)
    }

    /**
     * Decode base64-encoded bytes to its original bytes.
     *
     * # Arguments
     *
     * * src - base64-encoded bytes
     *
     * # Return
     *
     * decoded bytes
     */
    fn decode_bytes(src: &[u8]) -> ~[u8] {
        let dst_length = decoded_len(src.len());
        let dst = vec::to_mut(vec::from_elem(dst_length, 0u8));
        let end = self.decode(dst, src);
        vec::slice(vec::from_mut(dst), 0u, end)
    }

    /**
     * Decode base64-encoded bytes to its original bytes.
     *
     * Note that this method is for url and filename safe base64 encoding.
     * See <http://tools.ietf.org/html/rfc4648#section-5> for details.
     *
     * # Arguments
     *
     * * src - base64-encoded bytes
     *
     * # Return
     *
     * decoded bytes
     */
    fn decode_bytes_u(src: &[u8]) -> ~[u8] {
        let dst_length = decoded_len(src.len());
        let dst = vec::to_mut(vec::from_elem(dst_length, 0u8));
        let end = self.decode_u(dst, src);
        vec::slice(vec::from_mut(dst), 0u, end)
    }
}

/**
 * Shortcut for enc#encode_bytes
 *
 * Table of base64 alphabet and decode map are allocated
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
 * base64-encoded bytes
 */
fn encode(src: &[u8]) -> ~[u8] {
    let base64 = base64();
    base64.encode_bytes(src)
}

/**
 * Shortcut for enc#encode_bytes_u
 *
 * Table of base64 alphabet and decode map are allocated
 * every time when this function is called, so it's
 * recommended to use `mk` and then `encode_bytes_u` instead
 * if it's necessary to use this function many times.
 *
 * # Arguments
 *
 * * src - bytes for encoding
 *
 * # Return
 *
 * base64-encoded bytes (url and filename safe)
 */
fn urlsafe_encode(src: &[u8]) -> ~[u8] {
    let base64 = base64();
    base64.encode_bytes_u(src)
}

/**
 * Shortcut for enc#decode_bytes
 *
 * Table of base64 alphabet and decode map are allocated
 * every time when this function is called, so it's
 * recommended to use `mk` and then `decode_bytes` instead
 * if it's necessary to use this function many times.
 *
 * # Arguments
 *
 * * src - base64-encoded bytes
 *
 * # Return
 *
 * decoded bytes
 */
fn decode(src: &[u8]) -> ~[u8] {
    let base64 = base64();
    base64.decode_bytes(src)
}

/**
 * Shortcut for enc#decode_bytes_u
 *
 * Table of base64 alphabet and decode map are allocated
 * every time when this function is called, so it's
 * recommended to use `mk` and then `decode_bytes_u` instead
 * if it's necessary to use this function many times.
 *
 * # Arguments
 *
 * * src - base64-encoded bytes
 *
 * # Return
 *
 * decoded bytes
 */
fn urlsafe_decode(src: &[u8]) -> ~[u8] {
    let base64 = base64();
    base64.decode_bytes_u(src)
}

#[inline(always)]
pure fn encoded_len(src_length: uint) -> uint {
    (src_length + 2u) / 3u * 4u
}

#[inline(always)]
pure fn decoded_len(src_length: uint) -> uint {
    src_length / 4u * 3u
}

fn b64encode(table: &[u8], dst: &[mut u8], src: &[u8]) {
    let src_length = src.len();
    let dst_length = dst.len();

    if src_length == 0u {
        ret;
    }

    if dst_length % 4u != 0u {
        fail "dst's length should be divisible by 4";
    }

    for uint::range(0u, (src_length + 2u) / 3u) |i| {
        let src_curr = 3u * i;
        let dst_curr = 4u * i;
        let remain = src_length - src_curr;

        dst[dst_curr + 0u] = 0u8;
        dst[dst_curr + 1u] = 0u8;
        dst[dst_curr + 2u] = 0u8;
        dst[dst_curr + 3u] = 0u8;

        if remain == 1u {
            dst[dst_curr + 0u] |= src[src_curr + 0u] >> 2u8;
            dst[dst_curr + 1u] |= src[src_curr + 0u] << 4u8 & 0x3f_u8;
        } else if remain == 2u {
            dst[dst_curr + 0u] |= src[src_curr + 0u] >> 2u8;
            dst[dst_curr + 1u] |= src[src_curr + 0u] << 4u8 & 0x3f_u8;
            dst[dst_curr + 1u] |= src[src_curr + 1u] >> 4u8;
            dst[dst_curr + 2u] |= src[src_curr + 1u] << 2u8 & 0x3f_u8;
        } else {
            dst[dst_curr + 0u] |= src[src_curr + 0u] >> 2u8;
            dst[dst_curr + 1u] |= src[src_curr + 0u] << 4u8 & 0x3f_u8;
            dst[dst_curr + 1u] |= src[src_curr + 1u] >> 4u8;
            dst[dst_curr + 2u] |= src[src_curr + 1u] << 2u8 & 0x3f_u8;
            dst[dst_curr + 2u] |= src[src_curr + 2u] >> 6u8;
            dst[dst_curr + 3u] |= src[src_curr + 2u] & 0x3f_u8;
        }

        dst[dst_curr + 0u] = table[dst[dst_curr + 0u]];
        dst[dst_curr + 1u] = table[dst[dst_curr + 1u]];
        dst[dst_curr + 2u] = table[dst[dst_curr + 2u]];
        dst[dst_curr + 3u] = table[dst[dst_curr + 3u]];

        if remain < 3u {
            dst[dst_curr + 3u] = PAD;
            if remain < 2u {
                dst[dst_curr + 2u] = PAD;
            }
            break;
        }
    }
}

fn b64decode(decode_map: &[u8], dst: &[mut u8], src: &[u8]) -> uint {
    let buf = vec::to_mut(vec::from_elem(4u, 0u8));
    let mut src_length = src.len();
    let mut src_curr = 0u;
    let mut dst_curr = 0u;
    let mut buf_len = 4u;
    let mut end = false;

    while src_length > 0u && !end {
        buf[0] = 0xff_u8;
        buf[1] = 0xff_u8;
        buf[2] = 0xff_u8;
        buf[3] = 0xff_u8;

        let mut i = 0u;
        while i < 4u {
            if src_length == 0u {
                fail "malformed base64 string";
            }
            let chr = src[src_curr];
            src_curr += 1u;
            src_length -= 1u;
            if char::is_whitespace(chr as char) {
                again;
            }
            if chr == PAD && i >= 2u && src_length < 4u {
                if src_length > 0u && src[src_curr] != PAD {
                    fail "malformed base64 string";
                }
                buf_len = i;
                end = true;
                break;
            }
            buf[i] = decode_map[chr];
            if buf[i] == 0xff_u8 {
                fail "malformed base64 string";
            }
            i += 1u;
        }

        if buf_len == 2u {
            dst[dst_curr + 0u] = buf[0] << 2u8 | buf[1] >> 4u8;
        } else if buf_len == 3u {
            dst[dst_curr + 0u] = buf[0] << 2u8 | buf[1] >> 4u8;
            dst[dst_curr + 1u] = (buf[1] & 0x0f_u8) << 4u8 | buf[2] >> 2u8;
        } else {
            dst[dst_curr + 0u] = buf[0] << 2u8 | buf[1] >> 4u8;
            dst[dst_curr + 1u] = (buf[1] & 0x0f_u8) << 4u8 | buf[2] >> 2u8;
            dst[dst_curr + 2u] = (buf[2] & 0x03_u8) << 6u8 | buf[3];
        }

        dst_curr += buf_len - 1u;
    }

    dst_curr
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_encode_bytes() {
        let src = ["", "f", "fo", "foo", "foob", "fooba", "foobar"]/_;
        let exp = ["", "Zg==", "Zm8=", "Zm9v",
                   "Zm9vYg==", "Zm9vYmE=", "Zm9vYmFy"]/_;
        let src = src.map(|e| str::bytes(e));
        let exp = exp.map(|e| str::bytes(e));
        let base64 = base64();

        for uint::range(0u, src.len()) |i| {
            let res = base64.encode_bytes(src[i]);
            assert res == exp[i];
        }
    }
    fn test_encode_bytes_u() {
        let src = ["", "f", "fo", "fo>", "foob", "fooba", "fo?ba?"]/_;
        let exp = ["", "Zg==", "Zm8=", "Zm8-",
                   "Zm9vYg==", "Zm9vYmE=", "Zm8_YmE_"]/_;
        let src = src.map(|e| str::bytes(e));
        let exp = exp.map(|e| str::bytes(e));
        let base64 = base64();

        for uint::range(0u, src.len()) |i| {
            let res = base64.encode_bytes_u(src[i]);
            assert res == exp[i];
        }
    }
    #[test]
    fn test_decode_bytes() {
        let src = ["", "Zg==", "Zm8=", "Zm8+",
                   "Zm9v\r\nYg==", "\tZm9vYmE=", "Zm8/YmE/"]/_;
        let exp = ["", "f", "fo", "fo>", "foob", "fooba", "fo?ba?"]/_;
        let src = src.map(|e| str::bytes(e));
        let exp = exp.map(|e| str::bytes(e));
        let base64 = base64();

        for uint::range(0u, src.len()) |i| {
            let res = base64.decode_bytes(src[i]);
            assert res == exp[i];
        }
    }
    #[test]
    fn test_decode_bytes_u() {
        let src = ["", "Zg==", "Zm8=", "Zm8-",
                   "Zm9v\r\nYg==", "\tZm9vYmE=", "Zm8_YmE_"]/_;
        let exp = ["", "f", "fo", "fo>", "foob", "fooba", "fo?ba?"]/_;
        let src = src.map(|e| str::bytes(e));
        let exp = exp.map(|e| str::bytes(e));
        let base64 = base64();

        for uint::range(0u, src.len()) |i| {
            let res = base64.decode_bytes_u(src[i]);
            assert res == exp[i];
        }
    }
}
