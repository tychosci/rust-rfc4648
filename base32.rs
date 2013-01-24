/*!
 * Base32 Module
 *
 * See <http://tools.ietf.org/html/rfc4648#section-6> for details.
 *
 * # Example
 *
 * ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
 * extern mod rfc4648;
 * use rfc4648::ToBase32;
 *
 * let src = "base32";
 * let res = src.to_base32();
 * let res = str::from_bytes(res);
 *
 * io::println(fmt!("%s", res));
 * ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
 */

use core::vec::bytes;
use super::util::DecodeResult;
use super::util::BinaryEncoder;
use super::util::BinaryDecoder;

const PAD: u8 = 61u8;

// ABCDEFGHIJKLMNOPQRSTUVWXYZ234567
const TABLE_STD: [u8*32] = [
    65, 66, 67, 68, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 80,
    81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 50, 51, 52, 53, 54, 55,
];

// 0123456789ABCDEFGHIJKLMNOPQRSTUV
const TABLE_HEX: [u8*32] = [
    48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 65, 66, 67, 68, 69, 70,
    71, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86,
];

const DECODE_MAP_STD: [u8*256] = [
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255,  26,  27,  28,  29,  30,  31, 255, 255, 255, 255, 255, 255, 255, 255,
    255,   0,   1,   2,   3,   4,   5,   6,   7,   8,   9,  10,  11,  12,  13,  14,
     15,  16,  17,  18,  19,  20,  21,  22,  23,  24,  25, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
];

const DECODE_MAP_HEX: [u8*256] = [
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
      0,   1,   2,   3,   4,   5,   6,   7,   8,   9, 255, 255, 255, 255, 255, 255,
    255,  10,  11,  12,  13,  14,  15,  16,  17,  18,  19,  20,  21,  22,  23,  24,
     25,  26,  27,  28,  29,  30,  31, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
];

pub const BASE32_STD: &Base32 = &Base32 {
    table: TABLE_STD,
    decode_map: DECODE_MAP_STD,
};

pub const BASE32_HEX: &Base32 = &Base32 {
    table: TABLE_HEX,
    decode_map: DECODE_MAP_HEX,
};

pub struct Base32 {
    priv table: [u8*32],
    priv decode_map: [u8*256],
}

#[inline(always)]
pure fn encoded_len(src_length: uint) -> uint {
    (src_length + 4) / 5 * 8
}

#[inline(always)]
pure fn decoded_len(src_length: uint) -> uint {
    src_length / 8 * 5
}

impl Base32 : BinaryEncoder {
    fn encode(&self, dst: &[mut u8], src: &[const u8]) {
        base32encode(self.table, dst, src);
    }

    fn encoded_len(&self, src_length: uint) -> uint {
        encoded_len(src_length)
    }

    fn encode_bytes(&self, src: &[const u8]) -> ~[u8] {
        let dst_length = self.encoded_len(src.len());
        let mut dst = vec::with_capacity(dst_length);

        unsafe { vec::raw::set_len(&mut dst, dst_length); }

        self.encode(dst, src);

        dst
    }
}

impl Base32 : BinaryDecoder {
    fn decode(&self, dst: &[mut u8], src: &[const u8]) -> DecodeResult {
        base32decode(self.decode_map, dst, src)
    }

    fn decoded_len(&self, src_length: uint) -> uint {
        decoded_len(src_length)
    }

    fn decode_bytes(&self, src: &[const u8]) -> ~[u8] {
        let dst_length = self.decoded_len(src.len());
        let mut dst = vec::with_capacity(dst_length);

        unsafe { vec::raw::set_len(&mut dst, dst_length); }

        let res = self.decode(dst, src);

        unsafe { vec::raw::set_len(&mut dst, res.ndecoded); }

        dst
    }
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
pub fn encode(src: &[const u8]) -> ~[u8] {
    BASE32_STD.encode_bytes(src)
}

/**
 * Encode input bytes to base32-hex-encoded bytes.
 *
 * # Arguments
 *
 * * src - bytes for encoding
 *
 * # Return
 *
 * base32-hex-encoded bytes
 */
pub fn hex_encode(src: &[const u8]) -> ~[u8] {
    BASE32_HEX.encode_bytes(src)
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
pub fn decode(src: &[const u8]) -> ~[u8] {
    BASE32_STD.decode_bytes(src)
}

/**
 * Decode base32-hex-encoded bytes to its original bytes.
 *
 * # Arguments
 *
 * * src - base32-hex-encoded bytes
 *
 * # Return
 *
 * decoded bytes
 */
pub fn hex_decode(src: &[const u8]) -> ~[u8] {
    BASE32_HEX.decode_bytes(src)
}

pub struct Base32Writer<T: io::Writer> {
    priv base32: &Base32,
    priv writer: &T,
    priv outbuf: [mut u8*1024],
    priv buf: [mut u8*5],
    priv mut nbuf: uint,
}

pub impl<T: io::Writer> Base32Writer<T> {
    static fn new(base32: &a/Base32, writer: &a/T) -> Base32Writer/&a<T> {
        Base32Writer {
            base32: base32,
            writer: writer,
            outbuf: [mut 0, ..1024],
            buf: [mut 0, ..5],
            nbuf: 0,
        }
    }

    fn write(&self, buf: &[const u8]) {
        let buflen = buf.len();
        let mut buf = vec::const_view(buf, 0, buflen);

        if self.nbuf > 0 {
            let mut i = 0;
            while i < buflen && self.nbuf < 5 {
                self.buf[self.nbuf] = buf[i];
                self.nbuf += 1;
                i += 1;
            }

            buf = vec::const_view(buf, i, buflen);
            if self.nbuf < 5 {
                return;
            }

            self.base32.encode(self.outbuf, vec::mut_view(self.buf, 0, 5));
            self.writer.write(vec::mut_view(self.outbuf, 0, 8));
            self.nbuf = 0;
        }

        while buf.len() >= 5 {
            let buflen = buf.len();
            let nn = buflen / 5 * 8;
            let nn = if nn > buflen { buflen } else { nn };
            let nn = nn - nn % 8;

            if nn > 0 {
                self.base32.encode(self.outbuf, vec::const_view(buf, 0, nn));
                self.writer.write(vec::mut_view(self.outbuf, 0, nn / 8 * 5));
            }

            buf = vec::const_view(buf, nn, buflen);
        }

        for uint::range(0, buf.len()) |i| {
            self.buf[i] = buf[i];
        }
        self.nbuf = buf.len();
    }

    fn close(self) {
        if self.nbuf > 0 {
            let nbuf = self.nbuf;
            self.nbuf = 0;

            let buf = vec::mut_view(self.buf, 0, nbuf);
            self.base32.encode(self.outbuf, buf);
            self.writer.write(vec::mut_view(self.outbuf, 0, 8));
        }
    }
}

pub impl<T: io::Writer> Base32Writer<T>: Drop {
    fn finalize(&self) {}
}

pub struct Base32Reader<T: io::Reader> {
    priv base32: &Base32,
    priv reader: &T,
    priv buf: [mut u8*1024],
    priv outbuf: [mut u8*640],
    priv mut nbuf: uint,
    priv mut noutbuf: uint,
    priv mut end: bool,
}

pub impl<T: io::Reader> Base32Reader<T> {
    static fn new(base32: &a/Base32, reader: &a/T) -> Base32Reader/&a<T> {
        Base32Reader {
            base32: base32,
            reader: reader,
            buf: [mut 0, ..1024],
            outbuf: [mut 0, ..640],
            nbuf: 0,
            noutbuf: 0,
            end: false,
        }
    }

    fn read(&self, p: &[mut u8], len: uint) -> uint {
        // use leftover output (decoded bytes) if it exists
        if self.noutbuf > 0 {
            bytes::copy_memory(p, self.outbuf, len);

            let n = if len > self.noutbuf { self.noutbuf } else { len };
            self.noutbuf -= n;
            // shift unread bytes to head
            for uint::range(0, self.noutbuf) |i| {
                self.outbuf[i] = self.outbuf[i+n];
            }

            return n;
        }
        // calculate least required # of bytes to read
        let nn = len / 5 * 8;
        let nn = if nn < 8 { 8 } else { nn };
        let nn = if nn > self.buf.len() { self.buf.len() } else { nn };

        let buf = vec::mut_view(self.buf, self.nbuf, nn);
        let nn  = self.reader.read(buf, buf.len());

        self.nbuf += nn;
        if self.nbuf < 8 {
            fail ~"malformed base32 input";
        }

        let nr = self.nbuf / 8 * 8; // total read bytes (except fringe bytes)
        let nw = self.nbuf / 8 * 5; // size of decoded bytes

        let buf = vec::mut_view(self.buf, 0, nr);

        let ndecoded = if nw > len {
            let res = self.base32.decode(self.outbuf, buf);
            // copy self.outbuf[0:len] to p
            bytes::copy_memory(p, self.outbuf, len);
            // shift unread bytes to head
            for uint::range(0, res.ndecoded - len) |i| {
                self.outbuf[i] = self.outbuf[i+len];
            }
            self.noutbuf = res.ndecoded - len;
            self.end = res.end;
            len
        } else {
            let res = self.base32.decode(p, buf);
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

        buf
    }

    fn eof(&self) -> bool {
        self.noutbuf == 0 && (self.end || self.reader.eof())
    }
}

fn base32encode(table: &[u8], dst: &[mut u8], src: &[const u8]) {
    let src_length = src.len();
    let dst_length = dst.len();

    if dst_length % 8 != 0 {
        fail ~"dst's length should be divisible by 8";
    }

    for uint::range(0, (src_length + 4) / 5) |i| {
        let src_curr = 5 * i;
        let dst_curr = 8 * i;
        let remain = src_length - src_curr;

        dst[dst_curr+0] = 0; dst[dst_curr+1] = 0;
        dst[dst_curr+2] = 0; dst[dst_curr+3] = 0;
        dst[dst_curr+4] = 0; dst[dst_curr+5] = 0;
        dst[dst_curr+6] = 0; dst[dst_curr+7] = 0;

        let n = (src[src_curr+0] as u64)<<32
            | if remain > 1 { (src[src_curr+1] as u64)<<24 } else { 0 }
            | if remain > 2 { (src[src_curr+2] as u64)<<16 } else { 0 }
            | if remain > 3 { (src[src_curr+3] as u64)<< 8 } else { 0 }
            | if remain > 4 { (src[src_curr+4] as u64)     } else { 0 };

        dst[dst_curr+0] = table[n>>35 & 0x1f];
        dst[dst_curr+1] = table[n>>30 & 0x1f];
        dst[dst_curr+2] = if remain > 1 { table[n>>25 & 0x1f] } else { PAD };
        dst[dst_curr+3] = if remain > 1 { table[n>>20 & 0x1f] } else { PAD };
        dst[dst_curr+4] = if remain > 2 { table[n>>15 & 0x1f] } else { PAD };
        dst[dst_curr+5] = if remain > 3 { table[n>>10 & 0x1f] } else { PAD };
        dst[dst_curr+6] = if remain > 3 { table[n>> 5 & 0x1f] } else { PAD };
        dst[dst_curr+7] = if remain > 4 { table[n     & 0x1f] } else { PAD };
    }
}

fn base32decode(decode_map: &[u8], dst: &[mut u8], src: &[const u8]) -> DecodeResult {
    let mut ndecoded = 0u;
    let mut dst = vec::mut_view(dst, 0, dst.len());
    let mut src = vec::const_view(src, 0, src.len());
    let mut end = false;

    while src.len() > 0 && !end {
        let buf = [mut 0xff_u8, ..8];
        let mut buf_len = 8u;

        let mut i = 0u;
        while i < 8 {
            if src.len() == 0 {
                fail ~"malformed base32 string";
            }
            let chr = src[0];
            src = vec::const_view(src, 1, src.len());
            if char::is_whitespace(chr as char) {
                loop;
            }
            if chr == PAD && i >= 2 && src.len() < 8 {
                for uint::range(0, (8-i-1)) |j| {
                    if src.len() > j && src[j] != PAD {
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

        dst[0] = 0; dst[1] = 0; dst[2] = 0;
        dst[3] = 0; dst[4] = 0;

        if buf_len < 2 || 8 < buf_len {
            fail ~"malformed base32 string";
        }

        dst[0] |= buf[0]<<3 | buf[1]>>2;
        dst[1] |= if buf_len > 2 { buf[1]<<6 | buf[2]<<1 } else { 0 };
        dst[1] |= if buf_len > 3 { buf[3]>>4             } else { 0 };
        dst[2] |= if buf_len > 3 { buf[3]<<4             } else { 0 };
        dst[2] |= if buf_len > 4 { buf[4]>>1             } else { 0 };
        dst[3] |= if buf_len > 4 { buf[4]<<7 | buf[5]<<2 } else { 0 };
        dst[3] |= if buf_len > 6 { buf[6]>>3             } else { 0 };
        dst[4] |= if buf_len > 6 { buf[6]<<5 | buf[7]    } else { 0 };

        dst = vec::mut_view(dst, 5, dst.len());
        match buf_len {
            2     => ndecoded += 1,
            3 | 4 => ndecoded += 2,
            5     => ndecoded += 3,
            6 | 7 => ndecoded += 4,
            8     => ndecoded += 5,
            _     => fail ~"malformed base32 string"
        }
    }

    DecodeResult { end: end, ndecoded: ndecoded }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn t(source: &[&str], expect: &[&str], cb: fn&((&[u8])) -> ~[u8]) {
        let source = source.map(|b| str::to_bytes(*b));
        let expect = expect.map(|b| str::to_bytes(*b));
        let actual = source.map(|e| cb(*e));
        debug!("expect: %?, actual: %?", expect, actual);
        assert expect == actual;
    }

    #[test]
    fn test_encode() {
        let source = ["", "f", "fo", "foo", "foob", "fooba", "foobar"];
        let expect = ["", "MY======", "MZXQ====", "MZXW6===", "MZXW6YQ=",
                      "MZXW6YTB", "MZXW6YTBOI======"];

        t(source, expect, encode);
    }

    #[test]
    fn test_hex_encode() {
        let source = ["", "f", "fo", "foo", "foob", "fooba", "foobar"];
        let expect = ["", "CO======", "CPNG====", "CPNMU===",
                      "CPNMUOG=", "CPNMUOJ1", "CPNMUOJ1E8======"];

        t(source, expect, hex_encode);
    }

    #[test]
    fn test_decode() {
        let source = ["", "MY======", "MZXQ====", "MZXW6===",
                      "\tMZXW\r\n6YQ=", "MZXW6YTB", "MZXW6YTBOI======"];
        let expect = ["", "f", "fo", "foo", "foob", "fooba", "foobar"];

        t(source, expect, decode);
    }

    #[test]
    fn test_hex_decode() {
        let source = ["", "CO======", "CPNG====", "CPNMU===",
                      "\tCPNM\r\nUOG=", "CPNMUOJ1", "CPNMUOJ1E8======"];
        let expect = ["", "f", "fo", "foo", "foob", "fooba", "foobar"];

        t(source, expect, hex_decode);
    }

    #[test]
    fn test_base32_writer() {
        use super::Base32Writer;

        let source1 = str::to_bytes("f");
        let source2 = str::to_bytes("ooba");
        let expect  = str::to_bytes("MZXW6YTB");

        let actual  = io::with_bytes_writer(|writer| {
            let writer = Base32Writer::new(BASE32_STD, &writer);
            writer.write(source1);
            writer.write(source2);
            writer.close();
        });

        assert expect == actual;
    }

    #[test]
    fn test_base32_reader() {
        use super::Base32Reader;

        let source = ["MY======", "MZXQ====", "MZXW6===",
                      "MZXW6YQ=", "MZXW6YTB", "MZXW6YTBOI======"];
        let expect = ["f", "fo", "foo", "foob", "fooba", "foobar"];

        let source = source.map(|b| str::to_bytes(*b));
        let expect = expect.map(|b| str::to_bytes(*b));

        let actual = source.map(|e| {
            io::with_bytes_reader(*e, |reader| {
                let reader = Base32Reader::new(BASE32_STD, &reader);

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
