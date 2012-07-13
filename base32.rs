/*!
 * Base32 Module
 *
 * See <http://tools.ietf.org/html/rfc4648#section-6> for details.
 *
 * # Example
 *
 * ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
 * use encoding;
 * import encoding::extensions;
 *
 * let src = "base32";
 * let res = src.encode(encoding::base32);
 * let res = str::from_bytes(res);
 *
 * io::println(#fmt["%s", res]);
 * ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
 */

export base32, encode, hex_encode, decode, hex_decode;

const PAD: u8 = 61u8;

class base32 {
    let table: ~[u8];
    let table_h: ~[u8];

    new() {
        self.table = str::bytes("ABCDEFGHIJKLMNOPQRSTUVWXYZ234567");
        self.table_h = str::bytes("0123456789ABCDEFGHIJKLMNOPQRSTUV");
    }

    fn encode(dst: &[mut u8], src: &[u8]) {
        b32encode(self.table, dst, src);
    }
    fn encode_h(dst: &[mut u8], src: &[u8]) {
        b32encode(self.table_h, dst, src);
    }
    fn decode(dst: &[mut u8], src: &[u8]) -> uint {
        b32decode(self.table, dst, src)
    }
    fn decode_h(dst: &[mut u8], src: &[u8]) -> uint {
        b32decode(self.table_h, dst, src)
    }

    /**
     * Encode input bytes to base32-encoded bytes.
     *
     * # Arguments
     *
     * * src - bytes for encoding
     *
     * # Return
     *
     * base32-encoded bytes
     */
    fn encode_bytes(src: &[u8]) -> ~[u8] {
        let dst_length = encoded_len(src.len());
        let dst = vec::to_mut(vec::from_elem(dst_length, 0u8));
        self.encode(dst, src);
        vec::from_mut(dst)
    }

    /**
     * Encode input bytes to base32-encoded bytes.
     *
     * Note that this method is for base32-hex encoding.
     * See <http://tools.ietf.org/html/rfc4648#section-7> for details.
     *
     * # Arguments
     *
     * * src - bytes for encoding
     *
     * # Return
     *
     * base32-encoded bytes
     */
    fn encode_bytes_h(src: &[u8]) -> ~[u8] {
        let dst_length = encoded_len(src.len());
        let dst = vec::to_mut(vec::from_elem(dst_length, 0u8));
        self.encode_h(dst, src);
        vec::from_mut(dst)
    }

    /**
     * Decode base32-encoded bytes to its original bytes.
     *
     * # Arguments
     *
     * * src - base32-encoded bytes
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
     * Decode base32-encoded bytes to its original bytes.
     *
     * Note that this method is for base32-hex encoding.
     * See <http://tools.ietf.org/html/rfc4648#section-7> for details.
     *
     * # Arguments
     *
     * * src - base32-encoded bytes
     *
     * # Return
     *
     * decoded bytes
     */
    fn decode_bytes_h(src: &[u8]) -> ~[u8] {
        let dst_length = decoded_len(src.len());
        let dst = vec::to_mut(vec::from_elem(dst_length, 0u8));
        let end = self.decode_h(dst, src);
        vec::slice(vec::from_mut(dst), 0u, end)
    }
}

/**
 * Shortcut for base32#encode_bytes
 *
 * # Arguments
 *
 * * src - bytes for encoding
 *
 * # Return
 *
 * base32-encoded bytes
 */
fn encode(src: &[u8]) -> ~[u8] {
    let base32 = base32();
    base32.encode_bytes(src)
}

/**
 * Shortcut for base32#encode_bytes_h
 *
 * # Arguments
 *
 * * src - bytes for encoding
 *
 * # Return
 *
 * base32-encoded bytes (extended hex alphabet)
 */
fn hex_encode(src: &[u8]) -> ~[u8] {
    let base32 = base32();
    base32.encode_bytes_h(src)
}

/**
 * Shortcut for base32#decode_bytes
 *
 * # Arguments
 *
 * * src - base32-encoded bytes
 *
 * # Return
 *
 * decoded bytes
 */
fn decode(src: &[u8]) -> ~[u8] {
    let base32 = base32();
    base32.decode_bytes(src)
}

/**
 * Shortcut for base32#decode_bytes_h
 *
 * # Arguments
 *
 * * src - base32-encoded bytes (extended hex alphabent)
 *
 * # Return
 *
 * decoded bytes
 */
fn hex_decode(src: &[u8]) -> ~[u8] {
    let base32 = base32();
    base32.decode_bytes_h(src)
}

#[inline(always)]
pure fn encoded_len(src_length: uint) -> uint {
    (src_length + 4u) / 5u * 8u
}

#[inline(always)]
pure fn decoded_len(src_length: uint) -> uint {
    src_length / 8u * 5u
}

fn b32encode(table: &[u8], dst: &[mut u8], src: &[u8]) {
    let src_length = src.len();
    let dst_length = dst.len();

    if src_length == 0u {
        ret;
    }

    if dst_length % 8u != 0u {
        fail "dst's length should be divisible by 8";
    }

    for uint::range(0u, (src_length + 4u) / 5u) |i| {
        let src_curr = 5u * i;
        let dst_curr = 8u * i;
        let remain = src_length - src_curr;

        dst[dst_curr + 0u] = 0u8;
        dst[dst_curr + 1u] = 0u8;
        dst[dst_curr + 2u] = 0u8;
        dst[dst_curr + 3u] = 0u8;
        dst[dst_curr + 4u] = 0u8;
        dst[dst_curr + 5u] = 0u8;
        dst[dst_curr + 6u] = 0u8;
        dst[dst_curr + 7u] = 0u8;

        if remain == 1u {
            dst[dst_curr + 0u] |= src[src_curr + 0u] >> 3u8;
            dst[dst_curr + 1u] |= src[src_curr + 0u] << 2u8 & 0x1f_u8;
        } else if remain == 2u {
            dst[dst_curr + 0u] |= src[src_curr + 0u] >> 3u8;
            dst[dst_curr + 1u] |= src[src_curr + 0u] << 2u8 & 0x1f_u8;
            dst[dst_curr + 1u] |= src[src_curr + 1u] >> 6u8 & 0x1f_u8;
            dst[dst_curr + 2u] |= src[src_curr + 1u] >> 1u8 & 0x1f_u8;
            dst[dst_curr + 3u] |= src[src_curr + 1u] << 4u8 & 0x1f_u8;
        } else if remain == 3u {
            dst[dst_curr + 0u] |= src[src_curr + 0u] >> 3u8;
            dst[dst_curr + 1u] |= src[src_curr + 0u] << 2u8 & 0x1f_u8;
            dst[dst_curr + 1u] |= src[src_curr + 1u] >> 6u8 & 0x1f_u8;
            dst[dst_curr + 2u] |= src[src_curr + 1u] >> 1u8 & 0x1f_u8;
            dst[dst_curr + 3u] |= src[src_curr + 1u] << 4u8 & 0x1f_u8;
            dst[dst_curr + 3u] |= src[src_curr + 2u] >> 4u8 & 0x1f_u8;
            dst[dst_curr + 4u] |= src[src_curr + 2u] << 1u8 & 0x1f_u8;
        } else if remain == 4u {
            dst[dst_curr + 0u] |= src[src_curr + 0u] >> 3u8;
            dst[dst_curr + 1u] |= src[src_curr + 0u] << 2u8 & 0x1f_u8;
            dst[dst_curr + 1u] |= src[src_curr + 1u] >> 6u8 & 0x1f_u8;
            dst[dst_curr + 2u] |= src[src_curr + 1u] >> 1u8 & 0x1f_u8;
            dst[dst_curr + 3u] |= src[src_curr + 1u] << 4u8 & 0x1f_u8;
            dst[dst_curr + 3u] |= src[src_curr + 2u] >> 4u8 & 0x1f_u8;
            dst[dst_curr + 4u] |= src[src_curr + 2u] << 1u8 & 0x1f_u8;
            dst[dst_curr + 4u] |= src[src_curr + 3u] >> 7u8;
            dst[dst_curr + 5u] |= src[src_curr + 3u] >> 2u8 & 0x1f_u8;
            dst[dst_curr + 6u] |= src[src_curr + 3u] << 3u8 & 0x1f_u8;
        } else {
            dst[dst_curr + 0u] |= src[src_curr + 0u] >> 3u8;
            dst[dst_curr + 1u] |= src[src_curr + 0u] << 2u8 & 0x1f_u8;
            dst[dst_curr + 1u] |= src[src_curr + 1u] >> 6u8 & 0x1f_u8;
            dst[dst_curr + 2u] |= src[src_curr + 1u] >> 1u8 & 0x1f_u8;
            dst[dst_curr + 3u] |= src[src_curr + 1u] << 4u8 & 0x1f_u8;
            dst[dst_curr + 3u] |= src[src_curr + 2u] >> 4u8 & 0x1f_u8;
            dst[dst_curr + 4u] |= src[src_curr + 2u] << 1u8 & 0x1f_u8;
            dst[dst_curr + 4u] |= src[src_curr + 3u] >> 7u8;
            dst[dst_curr + 5u] |= src[src_curr + 3u] >> 2u8 & 0x1f_u8;
            dst[dst_curr + 6u] |= src[src_curr + 3u] << 3u8 & 0x1f_u8;
            dst[dst_curr + 6u] |= src[src_curr + 4u] >> 5u8;
            dst[dst_curr + 7u] |= src[src_curr + 4u] & 0x1f_u8;
        }

        dst[dst_curr + 0u] = table[dst[dst_curr + 0u]];
        dst[dst_curr + 1u] = table[dst[dst_curr + 1u]];
        dst[dst_curr + 2u] = table[dst[dst_curr + 2u]];
        dst[dst_curr + 3u] = table[dst[dst_curr + 3u]];
        dst[dst_curr + 4u] = table[dst[dst_curr + 4u]];
        dst[dst_curr + 5u] = table[dst[dst_curr + 5u]];
        dst[dst_curr + 6u] = table[dst[dst_curr + 6u]];
        dst[dst_curr + 7u] = table[dst[dst_curr + 7u]];

        if remain < 5u {
            dst[dst_curr + 7u] = PAD;
            if remain < 4u {
                dst[dst_curr + 6u] = PAD;
                dst[dst_curr + 5u] = PAD;
                if remain < 3u {
                    dst[dst_curr + 4u] = PAD;
                    if remain < 2u {
                        dst[dst_curr + 3u] = PAD;
                        dst[dst_curr + 2u] = PAD;
                    }
                }
            }
            break;
        }
    }
}

fn b32decode(table: &[u8], dst: &[mut u8], src: &[u8]) -> uint {
    let buf = vec::to_mut(vec::from_elem(8u, 0u8));
    let mut src_length = src.len();
    let mut src_curr = 0u;
    let mut dst_curr = 0u;
    let mut buf_len = 8u;
    let mut end = false;

    while src_length > 0u && !end {
        buf[0] = 0xff_u8; buf[1] = 0xff_u8;
        buf[2] = 0xff_u8; buf[3] = 0xff_u8;
        buf[4] = 0xff_u8; buf[5] = 0xff_u8;
        buf[6] = 0xff_u8; buf[7] = 0xff_u8;

        let mut i = 0u;
        while i < 8u {
            if src_length == 0u {
                fail "malformed base32 string";
            }
            let chr = src[src_curr];
            src_curr += 1u;
            src_length -= 1u;
            if char::is_whitespace(chr as char) {
                again;
            }
            if chr == PAD && i >= 2u && src_length < 8u {
                for uint::range(0u, (8u - i - 1u)) |j| {
                    if src_length > j && src[src_curr + j] != PAD {
                        fail "malformed base32 string";
                    }
                }
                buf_len = i;
                end = true;
                break;
            }
            alt table.position_elem(chr) {
                some(n) { buf[i] = n as u8; }
                none { fail "malformed base32 string"; }
            }
            i += 1u;
        }

        alt buf_len {
            2u {
                dst[dst_curr + 0u] = buf[0u] << 3u8 | buf[1u] >> 2u8;
            }
            3u {
                dst[dst_curr + 0u] = buf[0u] << 3u8 | buf[1u] >> 2u8;
                dst[dst_curr + 1u] = (buf[1u] & 0x03_u8) << 6u8 | buf[2u] << 1u8;
            }
            4u {
                dst[dst_curr + 0u] = buf[0u] << 3u8 | buf[1u] >> 2u8;
                dst[dst_curr + 1u] = (buf[1u] & 0x03_u8) << 6u8 | buf[2u] << 1u8;
                dst[dst_curr + 1u] |= buf[3u] >> 4u8;
                dst[dst_curr + 2u] = (buf[3u] & 0x0f_u8) << 4u8;
            }
            5u | 6u {
                dst[dst_curr + 0u] = buf[0u] << 3u8 | buf[1u] >> 2u8;
                dst[dst_curr + 1u] = (buf[1u] & 0x03_u8) << 6u8 | buf[2u] << 1u8;
                dst[dst_curr + 1u] |= buf[3u] >> 4u8;
                dst[dst_curr + 2u] = (buf[3u] & 0x0f_u8) << 4u8;
                dst[dst_curr + 2u] |= buf[4u] >> 1u8;
                dst[dst_curr + 3u] = (buf[4u] & 0x01_u8) << 7u8 | buf[5u] << 2u8;
            }
            7u | 8u {
                dst[dst_curr + 0u] = buf[0u] << 3u8 | buf[1u] >> 2u8;
                dst[dst_curr + 1u] = (buf[1u] & 0x03_u8) << 6u8 | buf[2u] << 1u8;
                dst[dst_curr + 1u] |= buf[3u] >> 4u8;
                dst[dst_curr + 2u] = (buf[3u] & 0x0f_u8) << 4u8;
                dst[dst_curr + 2u] |= buf[4u] >> 1u8;
                dst[dst_curr + 3u] = (buf[4u] & 0x01_u8) << 7u8 | buf[5u] << 2u8;
                dst[dst_curr + 3u] |= buf[6u] >> 3u8;
                dst[dst_curr + 4u] = (buf[6u] & 0x07_u8) << 5u8 | buf[7u];
            }
            _ { fail "malformed base32 string"; }
        }

        alt buf_len {
            2u      { dst_curr += 1u; }
            3u | 4u { dst_curr += 2u; }
            5u      { dst_curr += 3u; }
            6u | 7u { dst_curr += 4u; }
            8u      { dst_curr += 5u; }
            _       { fail "malformed base32 string"; }
        }
    }

    dst_curr
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_encode_bytes() {
        let base32 = base32();

        let source = ["", "f", "fo", "foo", "foob", "fooba", "foobar"]/_;
        let expect = ["", "MY======", "MZXQ====", "MZXW6===", "MZXW6YQ=",
                      "MZXW6YTB", "MZXW6YTBOI======"]/_;
        let source = source.map(|e| str::bytes(e));
        let expect = expect.map(|e| str::bytes(e));

        let actual = source.map(|e| base32.encode_bytes(e));

        assert expect == actual;
    }
    #[test]
    fn test_encode_bytes_h() {
        let base32 = base32();

        let source = ["", "f", "fo", "foo", "foob", "fooba", "foobar"]/_;
        let expect = ["", "CO======", "CPNG====", "CPNMU===",
                      "CPNMUOG=", "CPNMUOJ1", "CPNMUOJ1E8======"]/_;
        let source = source.map(|e| str::bytes(e));
        let expect = expect.map(|e| str::bytes(e));

        let actual = source.map(|e| base32.encode_bytes_h(e));

        assert expect == actual;
    }
    #[test]
    fn test_decode_bytes() {
        let base32 = base32();

        let source = ["", "MY======", "MZXQ====", "MZXW6===",
                      "\tMZXW\r\n6YQ=", "MZXW6YTB", "MZXW6YTBOI======"]/_;
        let expect = ["", "f", "fo", "foo", "foob", "fooba", "foobar"]/_;
        let source = source.map(|e| str::bytes(e));
        let expect = expect.map(|e| str::bytes(e));

        let actual = source.map(|e| base32.decode_bytes(e));

        assert expect == actual;
    }
    #[test]
    fn test_decode_bytes_h() {
        let base32 = base32();

        let source = ["", "CO======", "CPNG====", "CPNMU===",
                      "\tCPNM\r\nUOG=", "CPNMUOJ1", "CPNMUOJ1E8======"]/_;
        let expect = ["", "f", "fo", "foo", "foob", "fooba", "foobar"]/_;

        let source = source.map(|e| str::bytes(e));
        let expect = expect.map(|e| str::bytes(e));

        let actual = source.map(|e| base32.decode_bytes_h(e));

        assert expect == actual;
    }
}
