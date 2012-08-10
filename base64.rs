/*!
 * Base64 module
 *
 * See <http://tools.ietf.org/html/rfc4648#section-4> for details.
 *
 * # Example
 *
 * ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
 * use encoding;
 * import encoding::Codec;
 *
 * let src = "base64";
 * let res = src.encode(encoding::Base64);
 * let res = str::from_bytes(res);
 *
 * io::println(fmt!("%s", res));
 * ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
 */

export BASE64_STD, BASE64_URL, Base64Writer;
export encode, urlsafe_encode, decode, urlsafe_decode;

macro_rules! abort {
    { $s:expr } => { fail str::from_slice($s) }
}

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

const BASE64_STD: &Base64 = &Base64 {
    table: TABLE_STD,
    decode_map: DECODE_MAP_STD,
};

const BASE64_URL: &Base64 = &Base64 {
    table: TABLE_URL,
    decode_map: DECODE_MAP_URL,
};

struct Base64 {
    table: [u8]/64;
    decode_map: [u8]/256;
}

#[inline(always)]
pure fn encoded_len(src_length: uint) -> uint {
    (src_length + 2) / 3 * 4
}

#[inline(always)]
pure fn decoded_len(src_length: uint) -> uint {
    src_length / 4 * 3
}

impl Base64 : Encode {
    fn encode(dst: &[mut u8], src: &[u8]) {
        b64encode(self.table, dst, src);
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
}

impl Base64 : Decode {
    fn decode(dst: &[mut u8], src: &[u8]) -> DecodeResult {
        b64decode(self.decode_map, dst, src)
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
        let res = self.decode(dst, src);
        match res {
            Continue(n) => vec::slice(vec::from_mut(dst), 0u, n),
            End(n)      => vec::slice(vec::from_mut(dst), 0u, n)
        }
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
    BASE64_STD.encode_bytes(src)
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
 * base64-encoded bytes (url and filename safe)
 */
fn urlsafe_encode(src: &[u8]) -> ~[u8] {
    BASE64_URL.encode_bytes(src)
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
    BASE64_STD.decode_bytes(src)
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
fn urlsafe_decode(src: &[u8]) -> ~[u8] {
    BASE64_URL.decode_bytes(src)
}

struct Base64Writer {
    base64: &Base64;
    writer: &io::writer;
    outbuf: [mut u8]/1024;
    buf: [mut u8]/3;
    mut nbuf: uint;
}

fn Base64Writer(base64: &Base64, writer: &io::writer) -> Base64Writer {
    Base64Writer {
        base64: base64,
        writer: writer,
        outbuf: [mut 0, ..1024],
        buf: [mut 0, ..3],
        nbuf: 0,
    }
}

impl Base64Writer {
    fn write(buf: &[u8]) {
        let buflen  = buf.len();
        let mut buf = vec::view(buf, 0, buflen);

        if self.nbuf > 0 {
            let mut i = 0;
            while i < buflen && self.nbuf < 3 {
                self.buf[self.nbuf] = buf[i];
                self.nbuf += 1;
                i += 1;
            }

            buf = vec::view(buf, i, buflen);
            if self.nbuf < 3 {
                return;
            }

            self.base64.encode(self.outbuf, vec::slice(self.buf, 0, 3));
            self.writer.write(vec::mut_view(self.outbuf, 0, 4));
            self.nbuf = 0;
        }

        while buf.len() >= 3 {
            let nleft = buf.len();
            let nn = self.outbuf.len() / 4 * 3;
            let nn = if nn > nleft { nleft } else { nn };
            let nn = nn - nn % 3;

            if nn > 0 {
                self.base64.encode(self.outbuf, vec::view(buf, 0, nn));
                self.writer.write(vec::mut_view(self.outbuf, 0, nn / 3 * 4));
            }

            buf = vec::view(buf, nn, nleft);
        }

        for uint::range(0, buf.len()) |i| {
            self.buf[i] = buf[i];
        }
        self.nbuf += buf.len();
    }
    // TODO call this method on dropping (or put these stmts to `drop {...}`)
    fn close() {
        if self.nbuf > 0 {
            let nbuf = self.nbuf;
            self.nbuf = 0;

            let buf = vec::slice(self.buf, 0, nbuf);
            self.base64.encode(self.outbuf, buf);
            self.writer.write(vec::mut_view(self.outbuf, 0, 4));
        }
    }
}

struct Base64Reader {
    base64: &Base64;
    reader: &io::reader;
    mut end: bool;
}

fn Base64Reader(base64: &Base64, reader: &io::reader) -> Base64Reader {
    Base64Reader {
        base64: base64,
        reader: reader,
        end: false,
    }
}

impl Base64Reader {
    fn read(buf: &[mut u8], len: uint) -> uint {
        // FIXME write
        return 0;
    }
    fn read_bytes(_nbytes: uint) -> ~[u8] {
        // FIXME write
        return ~[];
    }
    fn eof() -> bool { self.end || self.reader.eof() }
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

fn b64decode(decode_map: &[u8], dst: &[mut u8], src: &[u8]) -> DecodeResult {
    let buf = [mut 0u8, 0u8, 0u8, 0u8];
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

    if end {
        End(dst_curr)
    } else {
        Continue(dst_curr)
    }
}

#[cfg(test)]
module tests {
    #[test]
    fn test_encode() {
        let source = ["", "f", "fo", "foo", "foob", "fooba", "foobar"];
        let expect = ["", "Zg==", "Zm8=", "Zm9v", "Zm9vYg==", "Zm9vYmE=", "Zm9vYmFy"];
        let source = source.map(|e| str::bytes(e));
        let expect = expect.map(|e| str::bytes(e));

        let actual = source.map(|e| encode(e));

        assert expect == actual;
    }
    #[test]
    fn test_urlsafe_encode() {
        let source = ["", "f", "fo", "fo>", "foob", "fooba", "fo?ba?"];
        let expect = ["", "Zg==", "Zm8=", "Zm8-", "Zm9vYg==", "Zm9vYmE=", "Zm8_YmE_"];
        let source = source.map(|e| str::bytes(e));
        let expect = expect.map(|e| str::bytes(e));

        let actual = source.map(|e| urlsafe_encode(e));

        assert expect == actual;
    }
    #[test]
    fn test_decode() {
        let source = ["", "Zg==", "Zm8=", "Zm8+", "Zm9v\r\nYg==", "\tZm9vYmE=", "Zm8/YmE/"];
        let expect = ["", "f", "fo", "fo>", "foob", "fooba", "fo?ba?"];
        let source = source.map(|e| str::bytes(e));
        let expect = expect.map(|e| str::bytes(e));

        let actual = source.map(|e| decode(e));

        assert expect == actual;
    }
    #[test]
    fn test_urlsafe_decode() {
        let source = ["", "Zg==", "Zm8=", "Zm8-", "Zm9v\r\nYg==", "\tZm9vYmE=", "Zm8_YmE_"];
        let expect = ["", "f", "fo", "fo>", "foob", "fooba", "fo?ba?"];
        let source = source.map(|e| str::bytes(e));
        let expect = expect.map(|e| str::bytes(e));

        let actual = source.map(|e| urlsafe_decode(e));

        assert expect == actual;
    }
    #[test]
    fn test_base64_writer() {
        let source1 = str::bytes("f");
        let source2 = str::bytes("oobar");
        let expect  = str::bytes("Zm9vYmFy");
        let actual  = io::with_buf_writer(|writer| {
            let writer = Base64Writer(BASE64_STD, &writer);
            writer.write(source1);
            writer.write(source2);
            writer.close();
        });

        assert expect == actual;
    }
    // #[test]
    fn test_base64_reader() {
        let source = str::bytes("Zm9vYmFy");
        let expect = str::bytes("foobar");
        let actual = io::with_bytes_reader(source, |reader| {
            let reader = Base64Reader(BASE64_STD, &reader);

            io::with_buf_writer(|writer| {
                while !reader.eof() {
                    let buf = reader.read_bytes(1024);
                    writer.write(buf);
                }
            })
        });

        assert expect == actual;
    }
}
