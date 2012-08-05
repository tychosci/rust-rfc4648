/*!
 * Base32 Module
 *
 * See <http://tools.ietf.org/html/rfc4648#section-6> for details.
 *
 * # Example
 *
 * ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
 * use encoding;
 * import encoding::codec;
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

struct Base32 {
    table_std: ~[u8];
    table_hex: ~[u8];
    decode_map_std: ~[u8];
    decode_map_hex: ~[u8];
}

fn base32() -> @Base32 {
    let table_std = str::bytes(~"ABCDEFGHIJKLMNOPQRSTUVWXYZ234567");
    let table_hex = str::bytes(~"0123456789ABCDEFGHIJKLMNOPQRSTUV");

    let decode_map_std = vec::to_mut(vec::from_elem(256, 0xff_u8));
    let decode_map_hex = vec::to_mut(vec::from_elem(256, 0xff_u8));

    for u8::range(0, 32) |i| {
        decode_map_std[table_std[i]] = i;
        decode_map_hex[table_hex[i]] = i;
    }

    @Base32 {
        table_std: table_std,
        table_hex: table_hex,
        decode_map_std: vec::from_mut(decode_map_std),
        decode_map_hex: vec::from_mut(decode_map_hex)
    }
}

#[inline(always)]
pure fn encoded_len(src_length: uint) -> uint {
    (src_length + 4) / 5 * 8
}

#[inline(always)]
pure fn decoded_len(src_length: uint) -> uint {
    src_length / 8 * 5
}

impl Base32 : encode {
    fn encode(dst: &[mut u8], src: &[u8]) {
        b32encode(self.table_std, dst, src);
    }
    fn encode_h(dst: &[mut u8], src: &[u8]) {
        b32encode(self.table_hex, dst, src);
    }
    fn encoded_len(src_length: uint) -> uint {
        encoded_len(src_length)
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
        let dst_length = self.encoded_len(src.len());
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
        let dst_length = self.encoded_len(src.len());
        let dst = vec::to_mut(vec::from_elem(dst_length, 0u8));
        self.encode_h(dst, src);
        vec::from_mut(dst)
    }
}

impl Base32 : decode {
    fn decode(dst: &[mut u8], src: &[u8]) -> uint {
        b32decode(self.decode_map_std, dst, src)
    }
    fn decode_h(dst: &[mut u8], src: &[u8]) -> uint {
        b32decode(self.decode_map_hex, dst, src)
    }
    fn decoded_len(src_length: uint) -> uint {
        decoded_len(src_length)
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
        let dst_length = self.decoded_len(src.len());
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
        let dst_length = self.decoded_len(src.len());
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

fn b32encode(table: &[u8], dst: &[mut u8], src: &[u8]) {
    let src_length = src.len();
    let dst_length = dst.len();

    if src_length == 0 {
        return;
    }

    if dst_length % 8 != 0 {
        fail ~"dst's length should be divisible by 8";
    }

    for uint::range(0, (src_length + 4) / 5) |i| {
        let src_curr = 5 * i;
        let dst_curr = 8 * i;
        let remain = src_length - src_curr;

        dst[dst_curr+0] = 0;
        dst[dst_curr+1] = 0;
        dst[dst_curr+2] = 0;
        dst[dst_curr+3] = 0;
        dst[dst_curr+4] = 0;
        dst[dst_curr+5] = 0;
        dst[dst_curr+6] = 0;
        dst[dst_curr+7] = 0;

        if remain == 1 {
            dst[dst_curr+0] |= src[src_curr+0]>>3;
            dst[dst_curr+1] |= src[src_curr+0]<<2 & 0x1f;
        } else if remain == 2 {
            dst[dst_curr+0] |= src[src_curr+0]>>3;
            dst[dst_curr+1] |= src[src_curr+0]<<2 & 0x1f;
            dst[dst_curr+1] |= src[src_curr+1]>>6 & 0x1f;
            dst[dst_curr+2] |= src[src_curr+1]>>1 & 0x1f;
            dst[dst_curr+3] |= src[src_curr+1]<<4 & 0x1f;
        } else if remain == 3 {
            dst[dst_curr+0] |= src[src_curr+0]>>3;
            dst[dst_curr+1] |= src[src_curr+0]<<2 & 0x1f;
            dst[dst_curr+1] |= src[src_curr+1]>>6 & 0x1f;
            dst[dst_curr+2] |= src[src_curr+1]>>1 & 0x1f;
            dst[dst_curr+3] |= src[src_curr+1]<<4 & 0x1f;
            dst[dst_curr+3] |= src[src_curr+2]>>4 & 0x1f;
            dst[dst_curr+4] |= src[src_curr+2]<<1 & 0x1f;
        } else if remain == 4 {
            dst[dst_curr+0] |= src[src_curr+0]>>3;
            dst[dst_curr+1] |= src[src_curr+0]<<2 & 0x1f;
            dst[dst_curr+1] |= src[src_curr+1]>>6 & 0x1f;
            dst[dst_curr+2] |= src[src_curr+1]>>1 & 0x1f;
            dst[dst_curr+3] |= src[src_curr+1]<<4 & 0x1f;
            dst[dst_curr+3] |= src[src_curr+2]>>4 & 0x1f;
            dst[dst_curr+4] |= src[src_curr+2]<<1 & 0x1f;
            dst[dst_curr+4] |= src[src_curr+3]>>7;
            dst[dst_curr+5] |= src[src_curr+3]>>2 & 0x1f;
            dst[dst_curr+6] |= src[src_curr+3]<<3 & 0x1f;
        } else {
            dst[dst_curr+0] |= src[src_curr+0]>>3;
            dst[dst_curr+1] |= src[src_curr+0]<<2 & 0x1f;
            dst[dst_curr+1] |= src[src_curr+1]>>6 & 0x1f;
            dst[dst_curr+2] |= src[src_curr+1]>>1 & 0x1f;
            dst[dst_curr+3] |= src[src_curr+1]<<4 & 0x1f;
            dst[dst_curr+3] |= src[src_curr+2]>>4 & 0x1f;
            dst[dst_curr+4] |= src[src_curr+2]<<1 & 0x1f;
            dst[dst_curr+4] |= src[src_curr+3]>>7;
            dst[dst_curr+5] |= src[src_curr+3]>>2 & 0x1f;
            dst[dst_curr+6] |= src[src_curr+3]<<3 & 0x1f;
            dst[dst_curr+6] |= src[src_curr+4]>>5;
            dst[dst_curr+7] |= src[src_curr+4]    & 0x1f;
        }

        dst[dst_curr+0] = table[dst[dst_curr+0]];
        dst[dst_curr+1] = table[dst[dst_curr+1]];
        dst[dst_curr+2] = table[dst[dst_curr+2]];
        dst[dst_curr+3] = table[dst[dst_curr+3]];
        dst[dst_curr+4] = table[dst[dst_curr+4]];
        dst[dst_curr+5] = table[dst[dst_curr+5]];
        dst[dst_curr+6] = table[dst[dst_curr+6]];
        dst[dst_curr+7] = table[dst[dst_curr+7]];

        if remain < 5 {
            dst[dst_curr+7] = PAD;
            if remain < 4 {
                dst[dst_curr+6] = PAD;
                dst[dst_curr+5] = PAD;
                if remain < 3 {
                    dst[dst_curr+4] = PAD;
                    if remain < 2 {
                        dst[dst_curr+3] = PAD;
                        dst[dst_curr+2] = PAD;
                    }
                }
            }
            break;
        }
    }
}

fn b32decode(decode_map: &[u8], dst: &[mut u8], src: &[u8]) -> uint {
    let buf = [mut 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8]/_;
    let mut src_length = src.len();
    let mut src_curr = 0u;
    let mut dst_curr = 0u;
    let mut buf_len = 8u;
    let mut end = false;

    while src_length > 0 && !end {
        buf[0] = 0xff; buf[1] = 0xff;
        buf[2] = 0xff; buf[3] = 0xff;
        buf[4] = 0xff; buf[5] = 0xff;
        buf[6] = 0xff; buf[7] = 0xff;

        let mut i = 0u;
        while i < 8 {
            if src_length == 0 {
                fail ~"malformed base32 string";
            }
            let chr = src[src_curr];
            src_curr += 1;
            src_length -= 1;
            if char::is_whitespace(chr as char) {
                again;
            }
            if chr == PAD && i >= 2 && src_length < 8 {
                for uint::range(0, (8-i-1)) |j| {
                    if src_length > j && src[src_curr + j] != PAD {
                        fail ~"malformed base32 string";
                    }
                }
                buf_len = i;
                end = true;
                break;
            }
            buf[i] = decode_map[chr];
            if buf[i] == 0xff {
                fail ~"malformed base32 string";
            }
            i += 1;
        }

        match buf_len {
            2     => { dst[dst_curr+0]  = buf[0]<<3 | buf[1]>>2
                     }
            3     => { dst[dst_curr+0]  = buf[0]<<3 | buf[1]>>2
                     ; dst[dst_curr+1]  = buf[1]<<6 | buf[2]<<1
                     }
            4     => { dst[dst_curr+0]  = buf[0]<<3 | buf[1]>>2
                     ; dst[dst_curr+1]  = buf[1]<<6 | buf[2]<<1
                     ; dst[dst_curr+1] |= buf[3]>>4
                     ; dst[dst_curr+2]  = buf[3]<<4
                     }
            5 | 6 => { dst[dst_curr+0]  = buf[0]<<3 | buf[1]>>2
                     ; dst[dst_curr+1]  = buf[1]<<6 | buf[2]<<1
                     ; dst[dst_curr+1] |= buf[3]>>4
                     ; dst[dst_curr+2]  = buf[3]<<4
                     ; dst[dst_curr+2] |= buf[4]>>1
                     ; dst[dst_curr+3]  = buf[4]<<7 | buf[5]<<2
                     }
            7 | 8 => { dst[dst_curr+0]  = buf[0]<<3 | buf[1]>>2
                     ; dst[dst_curr+1]  = buf[1]<<6 | buf[2]<<1
                     ; dst[dst_curr+1] |= buf[3]>>4
                     ; dst[dst_curr+2]  = buf[3]<<4
                     ; dst[dst_curr+2] |= buf[4]>>1
                     ; dst[dst_curr+3]  = buf[4]<<7 | buf[5]<<2
                     ; dst[dst_curr+3] |= buf[6]>>3
                     ; dst[dst_curr+4]  = buf[6]<<5 | buf[7]
                     }
            _     => fail ~"malformed base32 string"
        }

        match buf_len {
            2     => dst_curr += 1
          , 3 | 4 => dst_curr += 2
          , 5     => dst_curr += 3
          , 6 | 7 => dst_curr += 4
          , 8     => dst_curr += 5
          , _     => fail ~"malformed base32 string"
        }
    }

    dst_curr
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_encode_bytes() {
        let base32 = base32();

        let source = [~"", ~"f", ~"fo", ~"foo", ~"foob", ~"fooba", ~"foobar"]/_;
        let expect = [~"", ~"MY======", ~"MZXQ====", ~"MZXW6===", ~"MZXW6YQ=",
                      ~"MZXW6YTB", ~"MZXW6YTBOI======"]/_;
        let source = source.map(|e| str::bytes(e));
        let expect = expect.map(|e| str::bytes(e));

        let actual = source.map(|e| base32.encode_bytes(e));

        assert expect == actual;
    }
    #[test]
    fn test_encode_bytes_h() {
        let base32 = base32();

        let source = [~"", ~"f", ~"fo", ~"foo", ~"foob", ~"fooba", ~"foobar"]/_;
        let expect = [~"", ~"CO======", ~"CPNG====", ~"CPNMU===",
                      ~"CPNMUOG=", ~"CPNMUOJ1", ~"CPNMUOJ1E8======"]/_;
        let source = source.map(|e| str::bytes(e));
        let expect = expect.map(|e| str::bytes(e));

        let actual = source.map(|e| base32.encode_bytes_h(e));

        assert expect == actual;
    }
    #[test]
    fn test_decode_bytes() {
        let base32 = base32();

        let source = [~"", ~"MY======", ~"MZXQ====", ~"MZXW6===",
                      ~"\tMZXW\r\n6YQ=", ~"MZXW6YTB", ~"MZXW6YTBOI======"]/_;
        let expect = [~"", ~"f", ~"fo", ~"foo", ~"foob", ~"fooba", ~"foobar"]/_;
        let source = source.map(|e| str::bytes(e));
        let expect = expect.map(|e| str::bytes(e));

        let actual = source.map(|e| base32.decode_bytes(e));

        assert expect == actual;
    }
    #[test]
    fn test_decode_bytes_h() {
        let base32 = base32();

        let source = [~"", ~"CO======", ~"CPNG====", ~"CPNMU===",
                      ~"\tCPNM\r\nUOG=", ~"CPNMUOJ1", ~"CPNMUOJ1E8======"]/_;
        let expect = [~"", ~"f", ~"fo", ~"foo", ~"foob", ~"fooba", ~"foobar"]/_;

        let source = source.map(|e| str::bytes(e));
        let expect = expect.map(|e| str::bytes(e));

        let actual = source.map(|e| base32.decode_bytes_h(e));

        assert expect == actual;
    }
}
