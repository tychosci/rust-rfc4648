/*!
 * Base64 module
 *
 * See <http://tools.ietf.org/html/rfc4648#section-4> for details.
 *
 * # Example
 *
 * ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
 * extern mod rfc4648;
 * use rfc4648::ToBase64;
 *
 * let src = "base64";
 * let res = src.to_base64();
 * let res = str::from_bytes(res);
 *
 * io::println(fmt!("%s", res));
 * ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
 */

use super::util::DecodeResult;
use super::util::BinaryEncoder;
use super::util::BinaryDecoder;

static PAD: u8 = 61u8;

// ABCDEFGHIJKLMNOPQRSTUVWXYZ
// abcdefghijklmnopqrstuvwxyz
// 0123456789+/
static TABLE_STD: [u8, ..64] = [
     65,  66,  67,  68,  69,  70,  71,  72,  73,  74,  75,  76,  77,  78,  79,  80,
     81,  82,  83,  84,  85,  86,  87,  88,  89,  90,  97,  98,  99, 100, 101, 102,
    103, 104, 105, 106, 107, 108, 109, 110, 111, 112, 113, 114, 115, 116, 117, 118,
    119, 120, 121, 122,  48,  49,  50,  51,  52,  53,  54,  55,  56,  57,  43,  47,
];

// ABCDEFGHIJKLMNOPQRSTUVWXYZ
// abcdefghijklmnopqrstuvwxyz
// 0123456789-_
static TABLE_URL: [u8, ..64] = [
     65,  66,  67,  68,  69,  70,  71,  72,  73,  74,  75,  76,  77,  78,  79,  80,
     81,  82,  83,  84,  85,  86,  87,  88,  89,  90,  97,  98,  99, 100, 101, 102,
    103, 104, 105, 106, 107, 108, 109, 110, 111, 112, 113, 114, 115, 116, 117, 118,
    119, 120, 121, 122,  48,  49,  50,  51,  52,  53,  54,  55,  56,  57,  45,  95,
];

static DECODE_MAP_STD: [u8, ..256] = [
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

static DECODE_MAP_URL: [u8, ..256] = [
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

pub static BASE64_STD: &'static Base64 = &Base64 {
    table: TABLE_STD,
    decode_map: DECODE_MAP_STD,
};

pub static BASE64_URL: &'static Base64 = &Base64 {
    table: TABLE_URL,
    decode_map: DECODE_MAP_URL,
};

pub struct Base64 {
    priv table: [u8, ..64],
    priv decode_map: [u8, ..256],
}

impl BinaryEncoder for Base64 {
    #[inline]
    fn encode(&self, dst: &mut [u8], src: &const [u8]) {
        base64encode(self.table, dst, src);
    }

    #[inline]
    fn encoded_len(&self, src_length: uint) -> uint {
        (src_length + 2) / 3 * 4
    }

    #[inline]
    fn encode_bytes(&self, src: &const [u8]) -> ~[u8] {
        let dst_length = self.encoded_len(src.len());
        let mut dst = vec::with_capacity(dst_length);

        unsafe { vec::raw::set_len(&mut dst, dst_length); }

        self.encode(dst, src);

        dst
    }
}

impl BinaryDecoder for Base64 {
    #[inline]
    fn decode(&self, dst: &mut [u8], src: &const [u8]) -> DecodeResult {
        base64decode(self.decode_map, dst, src)
    }

    #[inline]
    fn decoded_len(&self, src_length: uint) -> uint {
        src_length / 4 * 3
    }

    #[inline]
    fn decode_bytes(&self, src: &const [u8]) -> ~[u8] {
        let dst_length = self.decoded_len(src.len());
        let mut dst = vec::with_capacity(dst_length);

        unsafe { vec::raw::set_len(&mut dst, dst_length); }

        let res = self.decode(dst, src);

        unsafe { vec::raw::set_len(&mut dst, res.ndecoded); }

        dst
    }
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
#[inline]
pub fn encode(src: &const [u8]) -> ~[u8] {
    BASE64_STD.encode_bytes(src)
}

/**
 * Encode input bytes to base64-urlsafe-encoded bytes.
 *
 * # Arguments
 *
 * * src - bytes for encoding
 *
 * # Return
 *
 * base64-urlsafe-encoded bytes
 */
#[inline]
pub fn urlsafe_encode(src: &const [u8]) -> ~[u8] {
    BASE64_URL.encode_bytes(src)
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
#[inline]
pub fn decode(src: &const [u8]) -> ~[u8] {
    BASE64_STD.decode_bytes(src)
}

/**
 * Decode base64-urlsafe-encoded bytes to original bytes.
 *
 * # Arguments
 *
 * * src - base64-urlsafe-encoded bytes
 *
 * # Return
 *
 * decoded bytes
 */
#[inline]
pub fn urlsafe_decode(src: &const [u8]) -> ~[u8] {
    BASE64_URL.decode_bytes(src)
}

pub struct Base64Writer {
    priv base64: &'static Base64,
    priv writer: @io::Writer,
    priv outbuf: [u8, ..1024],
    priv buf: [u8, ..3],
    priv nbuf: uint
}

pub impl Base64Writer {
    fn new(base64: &'static Base64, writer: @io::Writer) -> Base64Writer {
        Base64Writer {
            base64: base64,
            writer: writer,
            outbuf: [0, ..1024],
            buf: [0, ..3],
            nbuf: 0
        }
    }

    fn write(&mut self, buf: &const [u8]) {
        let buflen  = buf.len();
        let mut buf = vec::const_slice(buf, 0, buflen);

        if self.nbuf > 0 {
            let mut i = 0;

            while i < buflen && self.nbuf < 3 {
                self.buf[self.nbuf] = buf[i];
                self.nbuf += 1;
                i += 1;
            }

            buf = vec::const_slice(buf, i, buflen);
            if self.nbuf < 3 { return; }

            self.base64.encode(self.outbuf, vec::slice(self.buf, 0, 3));
            self.writer.write(vec::slice(self.outbuf, 0, 4));

            self.nbuf = 0;
        }

        while buf.len() >= 3 {
            let buflen = buf.len();
            let nn = self.outbuf.len() / 4 * 3;
            let nn = if nn > buflen { buflen } else { nn };
            let nn = nn - nn % 3;

            if nn > 0 {
                self.base64.encode(self.outbuf, vec::const_slice(buf, 0, nn));
                self.writer.write(vec::slice(self.outbuf, 0, nn / 3 * 4));
            }

            buf = vec::const_slice(buf, nn, buflen);
        }

        for uint::range(0, buf.len()) |i| {
            self.buf[i] = buf[i];
        }
        self.nbuf += buf.len();
    }

    fn close(self) {
        let mut self = self;

        if self.nbuf > 0 {
            let nbuf = self.nbuf;

            let buf = vec::slice(self.buf, 0, nbuf);
            self.base64.encode(self.outbuf, buf);
            self.writer.write(vec::slice(self.outbuf, 0, 4));
        }
    }
}

pub struct Base64Reader {
    priv base64: &'static Base64,
    priv reader: @io::Reader,
    priv buf: [u8, ..1024],
    priv outbuf: [u8, ..768],
    priv nbuf: uint,
    priv noutbuf: uint,
    priv end: bool
}

pub impl Base64Reader {
    fn new(base64: &'static Base64, reader: @io::Reader) -> Base64Reader {
        Base64Reader {
            base64: base64,
            reader: reader,
            buf: [0, ..1024],
            outbuf: [0, ..768],
            nbuf: 0,
            noutbuf: 0,
            end: false
        }
    }

    fn read(&mut self, p: &mut [u8], len: uint) -> uint {
        // NOTE: These borrowing is required to suppress odd loaning errors.
        let selfbuf: &mut [u8] = self.buf;
        let selfoutbuf: &mut [u8] = self.outbuf;
        let selfnbuf: &mut uint = &mut self.nbuf;
        let selfnoutbuf: &mut uint = &mut self.noutbuf;
        let selfend: &mut bool = &mut self.end;

        // use leftover output (decoded bytes) if it exists
        if self.noutbuf > 0 {
            unsafe { vec::raw::copy_memory(p, selfoutbuf, len); }

            let n = if len > self.noutbuf { self.noutbuf } else { len };
            *selfnoutbuf -= n;
            // shift unread bytes to head
            for uint::range(0, self.noutbuf) |i| {
                selfoutbuf[i] = selfoutbuf[i+n];
            }

            return n;
        }
        // calculate least required # of bytes to read
        let nn = len / 3 * 4;
        let nn = if nn < 4 { 4 } else { nn };
        let nn = if nn > self.buf.len() { self.buf.len() } else { nn };

        let buf = vec::mut_slice(selfbuf, self.nbuf, nn);
        let nn  = self.reader.read(buf, buf.len());

        *selfnbuf += nn;
        if self.nbuf < 4 {
            fail!(~"malformed base64 input");
        }

        let nr = self.nbuf / 4 * 4; // total read bytes (except fringe bytes)
        let nw = self.nbuf / 4 * 3; // size of decoded bytes

        let buf = vec::mut_slice(selfbuf, 0, nr);

        let ndecoded = if nw > len {
            let res = self.base64.decode(selfoutbuf, buf);
            // copy self.outbuf[0:len] to p
            unsafe { vec::raw::copy_memory(p, selfoutbuf, len); }
            // shift unread bytes to head
            for uint::range(0, res.ndecoded - len) |i| {
                selfoutbuf[i] = selfoutbuf[i+len];
            }
            *selfnoutbuf = res.ndecoded - len;
            *selfend = res.end;
            len
        } else {
            let res = self.base64.decode(p, buf);
            *selfend = res.end;
            res.ndecoded
        };
        *selfnbuf -= nr;
        // shift undecoded bytes to head
        for uint::range(0, self.nbuf) |i| {
            selfbuf[i] = selfbuf[i+nr];
        }

        ndecoded
    }

    fn eof(&self) -> bool {
        self.noutbuf == 0 && (self.end || self.reader.eof())
    }
}

fn base64encode(table: &[u8], dst: &mut [u8], src: &const [u8]) {
    let src_length = src.len();
    let dst_length = dst.len();

    if dst_length % 4 != 0 {
        fail!(~"dst's length should be divisible by 4");
    }

    let mut i = 0;
    let mut dst = dst;
    if src_length > 2 {
        while i < src_length - 2 {
            let n = (src[i] as uint)<<16 | (src[i+1] as uint)<<8 | (src[i+2] as uint);

            dst[0] = table[n>>18 & 0x3f];
            dst[1] = table[n>>12 & 0x3f];
            dst[2] = table[n>>6  & 0x3f];
            dst[3] = table[n     & 0x3f];
            dst = vec::mut_slice(dst, 4, dst.len());

            i += 3;
        }
    }

    let pad = src_length - i;
    if (pad == 1) {
        let n = (src[i] as uint)<<16;
        dst[0] = table[n>>18 & 0x3f];
        dst[1] = table[n>>12 & 0x3f];
        dst[2] = PAD;
        dst[3] = PAD;
    } else if (pad == 2) {
        let n = (src[i] as uint)<<16 | (src[i+1] as uint)<<8;
        dst[0] = table[n>>18 & 0x3f];
        dst[1] = table[n>>12 & 0x3f];
        dst[2] = table[n>>6  & 0x3f];
        dst[3] = PAD;
    }
}

fn base64decode(decode_map: &[u8], dst: &mut [u8], src: &const [u8]) -> DecodeResult {
    let mut ndecoded = 0u;
    let mut dst = vec::mut_slice(dst, 0, dst.len());
    let mut src = vec::const_slice(src, 0, src.len());
    let mut end = false;

    while src.len() > 0 && !end {
        let mut buf = [0xff_u8, ..4];
        let mut buf_len = 4u;

        let mut i = 0u;
        while i < 4 {
            if src.len() == 0 {
                fail!(~"malformed base64 string");
            }
            let chr = src[0];
            src = vec::const_slice(src, 1, src.len());
            if char::is_whitespace(chr as char) {
                loop;
            }
            if chr == PAD && i >= 2 && src.len() < 4 {
                if src.len() > 0 && src[0] != PAD {
                    fail!(~"malformed base64 string");
                }
                buf_len = i;
                end = true;
                break;
            }
            buf[i] = decode_map[chr];
            if buf[i] == 0xff {
                fail!(~"malformed base64 string");
            }
            i += 1;
        }

        dst[0] = buf[0]<<2 | buf[1]>>4;
        dst[1] = if buf_len > 2 { buf[1]<<4 | buf[2]>>2 } else { 0 };
        dst[2] = if buf_len > 3 { buf[2]<<6 | buf[3]    } else { 0 };

        dst = vec::mut_slice(dst, 3, dst.len());
        ndecoded += buf_len - 1;
    }

    DecodeResult { end: end, ndecoded: ndecoded }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn t(source: &[&str], expect: &[&str], cb: &fn((&[u8])) -> ~[u8]) {
        let source = source.map(|b| str::to_bytes(*b));
        let expect = expect.map(|b| str::to_bytes(*b));
        let actual = source.map(|e| cb(*e));
        assert_eq!(expect, actual);
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
        let expect = str::to_bytes("Zm9vYmFy");

        let actual = io::with_bytes_writer(|writer| {
            let mut writer = Base64Writer::new(BASE64_STD, writer);
            writer.write(source1);
            writer.write(source2);
            writer.close();
        });

        assert_eq!(expect, actual);
    }

    fn read_bytes(rd: &mut Base64Reader, len: uint) -> ~[u8] {
        let mut buf = vec::with_capacity(len);

        unsafe { vec::raw::set_len(&mut buf, len); }

        let nread = rd.read(buf, len);

        unsafe { vec::raw::set_len(&mut buf, nread); }

        buf
    }

    #[test]
    fn test_base64_reader() {
        let source = ["Zg==", "Zm8=", "Zm8+", "Zm9vYg==", "Zm9vYmE=", "Zm8/YmE/"];
        let expect = ["f", "fo", "fo>", "foob", "fooba", "fo?ba?"];
        let source = source.map(|b| str::to_bytes(*b));
        let expect = expect.map(|b| str::to_bytes(*b));

        let actual = source.map(|e| {
            io::with_bytes_reader(*e, |reader| {
                let mut reader = Base64Reader::new(BASE64_STD, reader);

                io::with_bytes_writer(|writer| {
                    while !reader.eof() {
                        let buf = read_bytes(&mut reader, 1);
                        writer.write(buf);
                    }
                })
            })
        });

        assert_eq!(expect, actual);
    }
}
