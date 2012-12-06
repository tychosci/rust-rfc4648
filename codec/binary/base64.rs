/*!
 * Base64 module
 *
 * See <http://tools.ietf.org/html/rfc4648#section-4> for details.
 *
 * # Example
 *
 * ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
 * extern mod codec;
 * use codec::BinaryCodec;
 *
 * let src = "base64";
 * let res = src.encode(codec::Base64);
 * let res = str::from_bytes(res);
 *
 * io::println(fmt!("%s", res));
 * ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
 */

macro_rules! abort (
    { $s:expr } => { fail str::from_slice($s) }
)

const PAD: u8 = 61u8;

// ABCDEFGHIJKLMNOPQRSTUVWXYZ
// abcdefghijklmnopqrstuvwxyz
// 0123456789+/
const TABLE_STD: [u8*64] = [
     65,  66,  67,  68,  69,  70,  71,  72,  73,  74,  75,  76,  77,  78,  79,  80,
     81,  82,  83,  84,  85,  86,  87,  88,  89,  90,  97,  98,  99, 100, 101, 102,
    103, 104, 105, 106, 107, 108, 109, 110, 111, 112, 113, 114, 115, 116, 117, 118,
    119, 120, 121, 122,  48,  49,  50,  51,  52,  53,  54,  55,  56,  57,  43,  47,
];

// ABCDEFGHIJKLMNOPQRSTUVWXYZ
// abcdefghijklmnopqrstuvwxyz
// 0123456789-_
const TABLE_URL: [u8*64] = [
     65,  66,  67,  68,  69,  70,  71,  72,  73,  74,  75,  76,  77,  78,  79,  80,
     81,  82,  83,  84,  85,  86,  87,  88,  89,  90,  97,  98,  99, 100, 101, 102,
    103, 104, 105, 106, 107, 108, 109, 110, 111, 112, 113, 114, 115, 116, 117, 118,
    119, 120, 121, 122,  48,  49,  50,  51,  52,  53,  54,  55,  56,  57,  45,  95,
];

const DECODE_MAP_STD: [u8*256] = [
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

const DECODE_MAP_URL: [u8*256] = [
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

pub const BASE64_STD: &Base64 = &Base64 {
    table: TABLE_STD,
    decode_map: DECODE_MAP_STD,
};

pub const BASE64_URL: &Base64 = &Base64 {
    table: TABLE_URL,
    decode_map: DECODE_MAP_URL,
};

pub struct Base64 {
    priv table: [u8*64],
    priv decode_map: [u8*256],
}

#[inline(always)]
pure fn encoded_len(src_length: uint) -> uint {
    (src_length + 2) / 3 * 4
}

#[inline(always)]
pure fn decoded_len(src_length: uint) -> uint {
    src_length / 4 * 3
}

pub impl Base64 : BinaryEncoder {
    fn encode(&self, dst: &[mut u8], src: &[const u8]) {
        base64encode(self.table, dst, src);
    }

    fn encoded_len(&self, src_length: uint) -> uint {
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
    fn encode_bytes(&self, src: &[const u8]) -> ~[u8] {
        let dst_length = self.encoded_len(src.len());
        let mut dst = vec::with_capacity(dst_length);

        unsafe { vec::raw::set_len(&mut dst, dst_length); }

        self.encode(dst, src);

        move dst
    }
}

pub impl Base64 : BinaryDecoder {
    fn decode(&self, dst: &[mut u8], src: &[const u8]) -> DecodeResult {
        base64decode(self.decode_map, dst, src)
    }

    fn decoded_len(&self, src_length: uint) -> uint {
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
    fn decode_bytes(&self, src: &[const u8]) -> ~[u8] {
        let dst_length = self.decoded_len(src.len());
        let mut dst = vec::with_capacity(dst_length);

        unsafe { vec::raw::set_len(&mut dst, dst_length); }

        let res = self.decode(dst, src);

        unsafe { vec::raw::set_len(&mut dst, res.ndecoded); }

        move dst
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
pub fn encode(src: &[const u8]) -> ~[u8] {
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
pub fn urlsafe_encode(src: &[const u8]) -> ~[u8] {
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
pub fn decode(src: &[const u8]) -> ~[u8] {
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
pub fn urlsafe_decode(src: &[const u8]) -> ~[u8] {
    BASE64_URL.decode_bytes(src)
}

pub struct Base64Writer<T: io::Writer> {
    priv base64: &Base64,
    priv writer: &T,
    priv outbuf: [mut u8*1024],
    priv buf: [mut u8*3],
    priv mut nbuf: uint,
}

pub impl<T: io::Writer> Base64Writer<T> {
    static fn new(base64: &a/Base64, writer: &a/T) -> Base64Writer/&a<T> {
        Base64Writer {
            base64: base64,
            writer: writer,
            outbuf: [mut 0, ..1024],
            buf: [mut 0, ..3],
            nbuf: 0,
        }
    }

    fn write(&self, buf: &[const u8]) {
        let buflen  = buf.len();
        let mut buf = vec::const_view(buf, 0, buflen);

        if self.nbuf > 0 {
            let mut i = 0;
            while i < buflen && self.nbuf < 3 {
                self.buf[self.nbuf] = buf[i];
                self.nbuf += 1;
                i += 1;
            }

            buf = vec::const_view(buf, i, buflen);
            if self.nbuf < 3 {
                return;
            }

            self.base64.encode(self.outbuf, vec::mut_view(self.buf, 0, 3));
            self.writer.write(vec::mut_view(self.outbuf, 0, 4));
            self.nbuf = 0;
        }

        while buf.len() >= 3 {
            let nleft = buf.len();
            let nn = self.outbuf.len() / 4 * 3;
            let nn = if nn > nleft { nleft } else { nn };
            let nn = nn - nn % 3;

            if nn > 0 {
                self.base64.encode(self.outbuf, vec::const_view(buf, 0, nn));
                self.writer.write(vec::mut_view(self.outbuf, 0, nn / 3 * 4));
            }

            buf = vec::const_view(buf, nn, nleft);
        }

        for uint::range(0, buf.len()) |i| {
            self.buf[i] = buf[i];
        }
        self.nbuf += buf.len();
    }
}

pub impl<T: io::Writer> Base64Writer<T> : Drop {
    fn finalize(&self) {
        if self.nbuf > 0 {
            let nbuf = self.nbuf;
            self.nbuf = 0;

            let buf = vec::mut_view(self.buf, 0, nbuf);
            self.base64.encode(self.outbuf, buf);
            self.writer.write(vec::mut_view(self.outbuf, 0, 4));
        }
    }
}

pub struct Base64Reader<T: io::Reader> {
    priv base64: &Base64,
    priv reader: &T,
    priv buf: [mut u8*1024],
    priv outbuf: [mut u8*768],
    priv mut nbuf: uint,
    priv mut noutbuf: uint,
    priv mut end: bool,
}

pub impl<T: io::Reader> Base64Reader<T> {
    static fn new(base64: &a/Base64, reader: &a/T) -> Base64Reader/&a<T> {
        Base64Reader {
            base64: base64,
            reader: reader,
            buf: [mut 0, ..1024],
            outbuf: [mut 0, ..768],
            nbuf: 0,
            noutbuf: 0,
            end: false,
        }
    }

    fn read(&self, p: &[mut u8], len: uint) -> uint {
        // use leftover output (decoded bytes) if it exists
        if self.noutbuf > 0 {
            vec::bytes::memcpy(p, self.outbuf, len);

            let n = if len > self.noutbuf { self.noutbuf } else { len };
            self.noutbuf -= n;
            // shift unread bytes to head
            for uint::range(0, self.noutbuf) |i| {
                self.outbuf[i] = self.outbuf[i+n];
            }

            return n;
        }
        // calculate least required # of bytes to read
        let nn = len / 3 * 4;
        let nn = if nn < 4 { 4 } else { nn };
        let nn = if nn > self.buf.len() { self.buf.len() } else { nn };

        let buf = vec::mut_view(self.buf, self.nbuf, nn);
        let nn  = self.reader.read(buf, buf.len());

        self.nbuf += nn;
        if self.nbuf < 4 {
            abort!("malformed base64 input");
        }

        let nr = self.nbuf / 4 * 4; // total read bytes (except fringe bytes)
        let nw = self.nbuf / 4 * 3; // size of decoded bytes

        let buf = vec::mut_view(self.buf, 0, nr);

        let ndecoded = if nw > len {
            let res = self.base64.decode(self.outbuf, buf);
            // copy self.outbuf[0:len] to p
            vec::bytes::memcpy(p, self.outbuf, len);
            // shift unread bytes to head
            for uint::range(0, res.ndecoded - len) |i| {
                self.outbuf[i] = self.outbuf[i+len];
            }
            self.noutbuf = res.ndecoded - len;
            self.end = res.end;
            len
        } else {
            let res = self.base64.decode(p, buf);
            self.end = res.end;
            res.ndecoded
        };
        self.nbuf -= nr;
        // shift undecoded bytes to head
        for uint::range(0, self.nbuf) |i| {
            self.buf[i] = self.buf[i+nr];
        }

        return ndecoded;
    }

    fn read_bytes(&self, len: uint) -> ~[u8] {
        let mut buf = vec::with_capacity(len);

        unsafe { vec::raw::set_len(&mut buf, len); }

        let nread = self.read(buf, len);

        unsafe { vec::raw::set_len(&mut buf, nread); }

        move buf
    }

    fn eof(&self) -> bool {
        self.noutbuf == 0 && (self.end || self.reader.eof())
    }
}

fn base64encode(table: &[u8], dst: &[mut u8], src: &[const u8]) {
    let src_length = src.len();
    let dst_length = dst.len();

    if dst_length % 4 != 0 {
        abort!("dst's length should be divisible by 4");
    }

    for uint::range(0, (src_length + 2) / 3) |i| {
        let src_curr = 3 * i;
        let dst_curr = 4 * i;
        let remain = src_length - src_curr;

        let n = (src[src_curr+0] as uint)<<16
            | if remain > 1 { (src[src_curr+1] as uint)<<8 } else { 0 }
            | if remain > 2 { (src[src_curr+2] as uint)    } else { 0 };

        dst[dst_curr+0] = table[n>>18 & 0x3f];
        dst[dst_curr+1] = table[n>>12 & 0x3f];
        dst[dst_curr+2] = if remain > 1 { table[n>>6 & 0x3f] } else { PAD };
        dst[dst_curr+3] = if remain > 2 { table[n    & 0x3f] } else { PAD };
    }
}

fn base64decode(decode_map: &[u8], dst: &[mut u8], src: &[const u8]) -> DecodeResult {
    let mut ndecoded = 0u;
    let mut dst = vec::mut_view(dst, 0, dst.len());
    let mut src = vec::const_view(src, 0, src.len());
    let mut end = false;

    while src.len() > 0 && !end {
        let buf = [mut 0xff_u8, ..4];
        let mut buf_len = 4u;

        let mut i = 0u;
        while i < 4 {
            if src.len() == 0 {
                abort!("malformed base64 string");
            }
            let chr = src[0];
            src = vec::const_view(src, 1, src.len());
            if char::is_whitespace(chr as char) {
                loop;
            }
            if chr == PAD && i >= 2 && src.len() < 4 {
                if src.len() > 0 && src[0] != PAD {
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

        dst[0] = buf[0]<<2 | buf[1]>>4;
        dst[1] = if buf_len > 2 { buf[1]<<4 | buf[2]>>2 } else { 0 };
        dst[2] = if buf_len > 3 { buf[2]<<6 | buf[3]    } else { 0 };

        dst = vec::mut_view(dst, 3, dst.len());
        ndecoded += buf_len - 1;
    }

    DecodeResult { end: end, ndecoded: ndecoded }
}

#[cfg(test)]
mod tests {
    fn t(source: &[&str], expect: &[&str], cb: fn((&[u8])) -> ~[u8]) {
        let source = source.map(|b| str::to_bytes(*b));
        let expect = expect.map(|b| str::to_bytes(*b));
        let actual = source.map(|e| cb(*e));
        debug!("expect: %?, actual: %?", expect, actual);
        assert expect == actual;
    }

    #[test]
    fn test_encode() {
        let source = ["", "f", "fo", "foo", "foob", "fooba", "foobar"];
        let expect = ["", "Zg==", "Zm8=", "Zm9v", "Zm9vYg==", "Zm9vYmE=", "Zm9vYmFy"];

        t(source, expect, encode);
    }

    #[test]
    fn test_urlsafe_encode() {
        let source = ["", "f", "fo", "fo>", "foob", "fooba", "fo?ba?"];
        let expect = ["", "Zg==", "Zm8=", "Zm8-", "Zm9vYg==", "Zm9vYmE=", "Zm8_YmE_"];

        t(source, expect, urlsafe_encode);
    }

    #[test]
    fn test_decode() {
        let source = ["", "Zg==", "Zm8=", "Zm8+", "Zm9v\r\nYg==", "\tZm9vYmE=", "Zm8/YmE/"];
        let expect = ["", "f", "fo", "fo>", "foob", "fooba", "fo?ba?"];

        t(source, expect, decode);
    }

    #[test]
    fn test_urlsafe_decode() {
        let source = ["", "Zg==", "Zm8=", "Zm8-", "Zm9v\r\nYg==", "\tZm9vYmE=", "Zm8_YmE_"];
        let expect = ["", "f", "fo", "fo>", "foob", "fooba", "fo?ba?"];

        t(source, expect, urlsafe_decode);
    }

    #[test]
    fn test_base64_writer() {
        let source1 = str::to_bytes("f");
        let source2 = str::to_bytes("oobar");
        let expect  = str::to_bytes("Zm9vYmFy");

        let actual  = io::with_bytes_writer(|writer| {
            let writer = Base64Writer::new(BASE64_STD, &writer);
            writer.write(source1);
            writer.write(source2);
        });

        assert expect == actual;
    }

    #[test]
    fn test_base64_reader() {
        let source = ["Zg==", "Zm8=", "Zm8+", "Zm9vYg==", "Zm9vYmE=", "Zm8/YmE/"];
        let expect = ["f", "fo", "fo>", "foob", "fooba", "fo?ba?"];
        let source = source.map(|b| str::to_bytes(*b));
        let expect = expect.map(|b| str::to_bytes(*b));

        let actual = source.map(|e| {
            io::with_bytes_reader(*e, |reader| {
                let reader = Base64Reader::new(BASE64_STD, &reader);

                io::with_bytes_writer(|writer| {
                    while !reader.eof() {
                        let buf = reader.read_bytes(1);
                        writer.write(buf);
                    }
                })
            })
        });

        assert expect == actual;
    }
}