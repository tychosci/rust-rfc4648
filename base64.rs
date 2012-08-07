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
 * io::println(fmt!("%s", res));
 * ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
 */

export base64, encode, urlsafe_encode, decode, urlsafe_decode;

const PAD: u8 = 61u8;

// ABCDEFGHIJKLMNOPQRSTUVWXYZ
// abcdefghijklmnopqrstuvwxyz
// 0123456789+/
const TABLE_STD: [u8]/64 = [
     65,  66,  67,  68,  69,  70,  71,  72,  73,  74,  75,  76,  77,  78,  79,  80,
     81,  82,  83,  84,  85,  86,  87,  88,  89,  90,  97,  98,  99, 100, 101, 102,
    103, 104, 105, 106, 107, 108, 109, 110, 111, 112, 113, 114, 115, 116, 117, 118,
    119, 120, 121, 122,  48,  49,  50,  51,  52,  53,  54,  55,  56,  57,  43,  47,
];

// ABCDEFGHIJKLMNOPQRSTUVWXYZ
// abcdefghijklmnopqrstuvwxyz
// 0123456789-_
const TABLE_URL: [u8]/64 = [
     65,  66,  67,  68,  69,  70,  71,  72,  73,  74,  75,  76,  77,  78,  79,  80,
     81,  82,  83,  84,  85,  86,  87,  88,  89,  90,  97,  98,  99, 100, 101, 102,
    103, 104, 105, 106, 107, 108, 109, 110, 111, 112, 113, 114, 115, 116, 117, 118,
    119, 120, 121, 122,  48,  49,  50,  51,  52,  53,  54,  55,  56,  57,  45,  95,
];

const DECODE_MAP_STD: [u8]/256 = [
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,  62, 255, 255, 255,  63,
     52,  53,  54,  55,  56,  57,  58,  59,  60,  61, 255, 255, 255, 255, 255, 255,
    255,   0,   1,   2,   3,   4,   5,   6,   7,   8,   9,  10,  11,  12,  13,  14,
     15,  16,  17,  18,  19,  20,  21,  22,  23,  24,  25, 255, 255, 255, 255, 255,
    255,  26,  27,  28,  29,  30,  31,  32,  33,  34,  35,  36,  37,  38,  39,  40,
     41,  42,  43,  44,  45,  46,  47,  48,  49,  50,  51, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
];

const DECODE_MAP_URL: [u8]/256 = [
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,  62, 255, 255,
     52,  53,  54,  55,  56,  57,  58,  59,  60,  61, 255, 255, 255, 255, 255, 255,
    255,   0,   1,   2,   3,   4,   5,   6,   7,   8,   9,  10,  11,  12,  13,  14,
     15,  16,  17,  18,  19,  20,  21,  22,  23,  24,  25, 255, 255, 255, 255,  63,
    255,  26,  27,  28,  29,  30,  31,  32,  33,  34,  35,  36,  37,  38,  39,  40,
     41,  42,  43,  44,  45,  46,  47,  48,  49,  50,  51, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
];

struct Base64 {
    table_std: [u8]/64;
    table_url: [u8]/64;
    decode_map_std: [u8]/256;
    decode_map_url: [u8]/256;
}

fn base64() -> Base64 {
    Base64 {
        table_std: TABLE_STD,
        table_url: TABLE_URL,
        decode_map_std: DECODE_MAP_STD,
        decode_map_url: DECODE_MAP_URL
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

macro_rules! switch {
    {
        $name:ident =>
        default : $default:expr
        $(case $($v:expr),+ : $blk:expr)+
    } => {
        $(if $($v < $name)&&+ { $blk })+
        $(if $($v == $name)||+ { $blk } else)+ { $default }
    }
}

macro_rules! abort {
    { $s:expr } => { fail str::from_slice($s) }
}

fn b64encode(table: &[u8], dst: &[mut u8], src: &[u8]) {
    let src_length = src.len();
    let dst_length = dst.len();

    if src_length == 0 {
        return;
    }

    if dst_length % 4 != 0 {
        abort!("dst's length should be divisible by 4");
    }

    for uint::range(0, (src_length + 2) / 3) |i| {
        let src_curr = 3 * i;
        let dst_curr = 4 * i;
        let remain = src_length - src_curr;

        dst[dst_curr+0] = 0; dst[dst_curr+1] = 0;
        dst[dst_curr+2] = 0; dst[dst_curr+3] = 0;

        switch! { remain =>
        default: { dst[dst_curr+3] |= src[src_curr+2]    & 0x3f
                 ; dst[dst_curr+2] |= src[src_curr+2]>>6 }
        case 02: { dst[dst_curr+2] |= src[src_curr+1]<<2 & 0x3f
                 ; dst[dst_curr+1] |= src[src_curr+1]>>4 }
        case 01: { dst[dst_curr+1] |= src[src_curr+0]<<4 & 0x3f
                 ; dst[dst_curr+0] |= src[src_curr+0]>>2 }
        };

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
        buf[0] = 0xff; buf[1] = 0xff;
        buf[2] = 0xff; buf[3] = 0xff;

        let mut i = 0u;
        while i < 4 {
            if src_length == 0 {
                abort!("malformed base64 string");
            }
            let chr = src[src_curr];
            src_curr += 1;
            src_length -= 1;
            if char::is_whitespace(chr as char) {
                again;
            }
            if chr == PAD && i >= 2 && src_length < 4 {
                if src_length > 0 && src[src_curr] != PAD {
                    abort!("malformed base64 string");
                }
                buf_len = i;
                end = true;
                break;
            }
            buf[i] = decode_map[chr];
            if buf[i] == 0xff {
                abort!("malformed base64 string");
            }
            i += 1;
        }

        switch! { buf_len =>
        default: { dst[dst_curr+2] = buf[2]<<6 | buf[3] }
        case 03: { dst[dst_curr+1] = buf[1]<<4 | buf[2]>>2 }
        case 02: { dst[dst_curr+0] = buf[0]<<2 | buf[1]>>4 }
        };

        dst_curr += buf_len - 1;
    }

    dst_curr
}

#[cfg(test)]
module tests {
    #[test]
    fn test_encode_bytes() {
        let base64 = base64();

        let source = ["", "f", "fo", "foo", "foob", "fooba", "foobar"]/_;
        let expect = ["", "Zg==", "Zm8=", "Zm9v", "Zm9vYg==", "Zm9vYmE=", "Zm9vYmFy"]/_;
        let source = source.map(|e| str::bytes(e));
        let expect = expect.map(|e| str::bytes(e));

        let actual = source.map(|e| base64.encode_bytes(e));

        assert expect == actual;
    }
    #[test]
    fn test_encode_bytes_u() {
        let base64 = base64();

        let source = ["", "f", "fo", "fo>", "foob", "fooba", "fo?ba?"]/_;
        let expect = ["", "Zg==", "Zm8=", "Zm8-", "Zm9vYg==", "Zm9vYmE=", "Zm8_YmE_"]/_;
        let source = source.map(|e| str::bytes(e));
        let expect = expect.map(|e| str::bytes(e));

        let actual = source.map(|e| base64.encode_bytes_u(e));

        assert expect == actual;
    }
    #[test]
    fn test_decode_bytes() {
        let base64 = base64();

        let source = ["", "Zg==", "Zm8=", "Zm8+", "Zm9v\r\nYg==", "\tZm9vYmE=", "Zm8/YmE/"]/_;
        let expect = ["", "f", "fo", "fo>", "foob", "fooba", "fo?ba?"]/_;
        let source = source.map(|e| str::bytes(e));
        let expect = expect.map(|e| str::bytes(e));

        let actual = source.map(|e| base64.decode_bytes(e));

        assert expect == actual;
    }
    #[test]
    fn test_decode_bytes_u() {
        let base64 = base64();

        let source = ["", "Zg==", "Zm8=", "Zm8-", "Zm9v\r\nYg==", "\tZm9vYmE=", "Zm8_YmE_"]/_;
        let expect = ["", "f", "fo", "fo>", "foob", "fooba", "fo?ba?"]/_;
        let source = source.map(|e| str::bytes(e));
        let expect = expect.map(|e| str::bytes(e));

        let actual = source.map(|e| base64.decode_bytes_u(e));

        assert expect == actual;
    }
}
