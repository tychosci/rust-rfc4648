/*!
 * Base64 module
 *
 * See <http://tools.ietf.org/html/rfc4648#section-4> for details.
 *
 * # Example
 *
 * ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
 * use encoding;
 * import encoding::codec;
 *
 * let src = "base64";
 * let res = src.encode(encoding::base64);
 * let res = str::from_bytes(res);
 *
 * io::println(#fmt["%s", res]);
 * ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
 */

export base64, encode, urlsafe_encode, decode, urlsafe_decode;

const PAD: u8 = 61u8;

struct Base64 {
    table_std: ~[u8];
    table_url: ~[u8];
    decode_map_std: ~[u8];
    decode_map_url: ~[u8];
}

fn base64() -> @Base64 {
    let table_std = str::bytes(~"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/");
    let table_url = str::bytes(~"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_");

    let decode_map_std = vec::to_mut(vec::from_elem(256, 0xFF_u8));
    let decode_map_url = vec::to_mut(vec::from_elem(256, 0xFF_u8));

    for u8::range(0, 64) |i| {
        decode_map_std[table_std[i]] = i;
        decode_map_url[table_url[i]] = i;
    }

    @Base64 {
        table_std: table_std,
        table_url: table_url,
        decode_map_std: vec::from_mut(decode_map_std),
        decode_map_url: vec::from_mut(decode_map_url)
    }
}

#[inline(always)]
pure fn encoded_len(src_length: uint) -> uint {
    (src_length + 2) / 3 * 4
}

#[inline(always)]
pure fn decoded_len(src_length: uint) -> uint {
    src_length / 4 * 3
}

impl Base64 : encode {
    fn encode(dst: &[mut u8], src: &[u8]) {
        b64encode(self.table_std, dst, src);
    }
    fn encode_u(dst: &[mut u8], src: &[u8]) {
        b64encode(self.table_url, dst, src);
    }
    fn encoded_len(src_length: uint) -> uint {
        encoded_len(src_length)
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
        let dst_length = self.encoded_len(src.len());
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
        let dst_length = self.encoded_len(src.len());
        let dst = vec::to_mut(vec::from_elem(dst_length, 0u8));
        self.encode_u(dst, src);
        vec::from_mut(dst)
    }
}

impl Base64 : decode {
    fn decode(dst: &[mut u8], src: &[u8]) -> uint {
        b64decode(self.decode_map_std, dst, src)
    }
    fn decode_u(dst: &[mut u8], src: &[u8]) -> uint {
        b64decode(self.decode_map_url, dst, src)
    }
    fn decoded_len(src_length: uint) -> uint {
        decoded_len(src_length)
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
        let dst_length = self.decoded_len(src.len());
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
        let dst_length = self.decoded_len(src.len());
        let dst = vec::to_mut(vec::from_elem(dst_length, 0u8));
        let end = self.decode_u(dst, src);
        vec::slice(vec::from_mut(dst), 0u, end)
    }
}

/**
 * Shortcut for base64#encode_bytes
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
 * Shortcut for base64#encode_bytes_u
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
 * Shortcut for base64#decode_bytes
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
 * Shortcut for base64#decode_bytes_u
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

fn b64encode(table: &[u8], dst: &[mut u8], src: &[u8]) {
    let src_length = src.len();
    let dst_length = dst.len();

    if src_length == 0 {
        return;
    }

    if dst_length % 4 != 0 {
        fail ~"dst's length should be divisible by 4";
    }

    for uint::range(0, (src_length + 2) / 3) |i| {
        let src_curr = 3 * i;
        let dst_curr = 4 * i;
        let remain = src_length - src_curr;

        dst[dst_curr+0] = 0;
        dst[dst_curr+1] = 0;
        dst[dst_curr+2] = 0;
        dst[dst_curr+3] = 0;

        if remain == 1 {
            dst[dst_curr+0] |= src[src_curr+0]>>2;
            dst[dst_curr+1] |= src[src_curr+0]<<4 & 0x3f;
        } else if remain == 2 {
            dst[dst_curr+0] |= src[src_curr+0]>>2;
            dst[dst_curr+1] |= src[src_curr+0]<<4 & 0x3f;
            dst[dst_curr+1] |= src[src_curr+1]>>4;
            dst[dst_curr+2] |= src[src_curr+1]<<2 & 0x3f;
        } else {
            dst[dst_curr+0] |= src[src_curr+0]>>2;
            dst[dst_curr+1] |= src[src_curr+0]<<4 & 0x3f;
            dst[dst_curr+1] |= src[src_curr+1]>>4;
            dst[dst_curr+2] |= src[src_curr+1]<<2 & 0x3f;
            dst[dst_curr+2] |= src[src_curr+2]>>6;
            dst[dst_curr+3] |= src[src_curr+2]    & 0x3f;
        }

        dst[dst_curr+0] = table[dst[dst_curr+0]];
        dst[dst_curr+1] = table[dst[dst_curr+1]];
        dst[dst_curr+2] = table[dst[dst_curr+2]];
        dst[dst_curr+3] = table[dst[dst_curr+3]];

        if remain < 3 {
            dst[dst_curr+3] = PAD;
            if remain < 2 {
                dst[dst_curr+2] = PAD;
            }
            break;
        }
    }
}

fn b64decode(decode_map: &[u8], dst: &[mut u8], src: &[u8]) -> uint {
    let buf = [mut 0u8, 0u8, 0u8, 0u8]/_;
    let mut src_length = src.len();
    let mut src_curr = 0u;
    let mut dst_curr = 0u;
    let mut buf_len = 4u;
    let mut end = false;

    while src_length > 0 && !end {
        buf[0] = 0xff;
        buf[1] = 0xff;
        buf[2] = 0xff;
        buf[3] = 0xff;

        let mut i = 0u;
        while i < 4 {
            if src_length == 0 {
                fail ~"malformed base64 string";
            }
            let chr = src[src_curr];
            src_curr += 1;
            src_length -= 1;
            if char::is_whitespace(chr as char) {
                again;
            }
            if chr == PAD && i >= 2 && src_length < 4 {
                if src_length > 0 && src[src_curr] != PAD {
                    fail ~"malformed base64 string";
                }
                buf_len = i;
                end = true;
                break;
            }
            buf[i] = decode_map[chr];
            if buf[i] == 0xff {
                fail ~"malformed base64 string";
            }
            i += 1;
        }

        if buf_len == 2 {
            dst[dst_curr+0] = buf[0]<<2 | buf[1]>>4;
        } else if buf_len == 3 {
            dst[dst_curr+0] = buf[0]<<2 | buf[1]>>4;
            dst[dst_curr+1] = buf[1]<<4 | buf[2]>>2;
        } else {
            dst[dst_curr+0] = buf[0]<<2 | buf[1]>>4;
            dst[dst_curr+1] = buf[1]<<4 | buf[2]>>2;
            dst[dst_curr+2] = buf[2]<<6 | buf[3];
        }

        dst_curr += buf_len - 1;
    }

    dst_curr
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_encode_bytes() {
        let base64 = base64();

        let source = [~"", ~"f", ~"fo", ~"foo", ~"foob", ~"fooba", ~"foobar"]/_;
        let expect = [~"", ~"Zg==", ~"Zm8=", ~"Zm9v", ~"Zm9vYg==", ~"Zm9vYmE=", ~"Zm9vYmFy"]/_;
        let source = source.map(|e| str::bytes(e));
        let expect = expect.map(|e| str::bytes(e));

        let actual = source.map(|e| base64.encode_bytes(e));

        assert expect == actual;
    }
    #[test]
    fn test_encode_bytes_u() {
        let base64 = base64();

        let source = [~"", ~"f", ~"fo", ~"fo>", ~"foob", ~"fooba", ~"fo?ba?"]/_;
        let expect = [~"", ~"Zg==", ~"Zm8=", ~"Zm8-", ~"Zm9vYg==", ~"Zm9vYmE=", ~"Zm8_YmE_"]/_;
        let source = source.map(|e| str::bytes(e));
        let expect = expect.map(|e| str::bytes(e));

        let actual = source.map(|e| base64.encode_bytes_u(e));

        assert expect == actual;
    }
    #[test]
    fn test_decode_bytes() {
        let base64 = base64();

        let source = [~"", ~"Zg==", ~"Zm8=", ~"Zm8+", ~"Zm9v\r\nYg==", ~"\tZm9vYmE=", ~"Zm8/YmE/"]/_;
        let expect = [~"", ~"f", ~"fo", ~"fo>", ~"foob", ~"fooba", ~"fo?ba?"]/_;
        let source = source.map(|e| str::bytes(e));
        let expect = expect.map(|e| str::bytes(e));

        let actual = source.map(|e| base64.decode_bytes(e));

        assert expect == actual;
    }
    #[test]
    fn test_decode_bytes_u() {
        let base64 = base64();

        let source = [~"", ~"Zg==", ~"Zm8=", ~"Zm8-", ~"Zm9v\r\nYg==", ~"\tZm9vYmE=", ~"Zm8_YmE_"]/_;
        let expect = [~"", ~"f", ~"fo", ~"fo>", ~"foob", ~"fooba", ~"fo?ba?"]/_;
        let source = source.map(|e| str::bytes(e));
        let expect = expect.map(|e| str::bytes(e));

        let actual = source.map(|e| base64.decode_bytes_u(e));

        assert expect == actual;
    }
}
