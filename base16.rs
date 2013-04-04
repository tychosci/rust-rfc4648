/*!
 * Base16 Module
 *
 * See <http://tools.ietf.org/html/rfc4648#section-8> for details.
 *
 * # Example
 *
 * ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
 * extern mod rfc4648;
 * use rfc4648::ToBase16;
 *
 * let src = "base16";
 * let res = src.to_base16();
 * let res = str::from_bytes(res);
 *
 * io::println(fmt!("%s", res));
 * ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
 */

use super::util::DecodeResult;
use super::util::BinaryEncoder;
use super::util::BinaryDecoder;

// 0123456789ABCDEF
static TABLE: [u8, ..16] = [
    48, 49, 50, 51, 52, 53, 54, 55,
    56, 57, 65, 66, 67, 68, 69, 70,
];

static DECODE_MAP: [u8, ..256] = [
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
      0,   1,   2,   3,   4,   5,   6,   7,   8,   9, 255, 255, 255, 255, 255, 255,
    255,  10,  11,  12,  13,  14,  15, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255,  10,  11,  12,  13,  14,  15, 255, 255, 255, 255, 255, 255, 255, 255, 255,
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

pub static BASE16: &'static Base16 = &Base16 {
    table: TABLE,
    decode_map: DECODE_MAP,
};

pub struct Base16 {
    priv table: [u8, ..16],
    priv decode_map: [u8, ..256],
}

impl BinaryEncoder for Base16 {
    #[inline]
    fn encode(&self, dst: &mut [u8], src: &const [u8]) {
        base16encode(self.table, dst, src);
    }

    #[inline]
    fn encoded_len(&self, src_length: uint) -> uint {
        src_length * 2
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

impl BinaryDecoder for Base16 {
    #[inline]
    fn decode(&self, dst: &mut [u8], src: &const [u8]) -> DecodeResult {
        base16decode(self.decode_map, dst, src)
    }

    #[inline]
    fn decoded_len(&self, src_length: uint) -> uint {
        src_length / 2
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
#[inline]
pub fn encode(src: &[u8]) -> ~[u8] {
    BASE16.encode_bytes(src)
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
#[inline]
pub fn decode(src: &[u8]) -> ~[u8] {
    BASE16.decode_bytes(src)
}

pub struct Base16Writer {
    priv base16: &'static Base16,
    priv writer: @io::Writer,
    priv outbuf: [u8, ..1024]
}

pub impl Base16Writer {
    fn new(base16: &'static Base16, writer: @io::Writer) -> Base16Writer {
        Base16Writer { base16: base16, writer: writer, outbuf: [0, ..1024] }
    }

    fn write(&mut self, buf: &const [u8]) {
        let mut buf = vec::const_slice(buf, 0, buf.len());

        while buf.len() > 0 {
            let buflen = buf.len();
            let nn = self.outbuf.len() / 2;
            let nn = if nn > buflen { buflen } else { nn };
            if (nn > 0) {
                self.base16.encode(self.outbuf, vec::const_slice(buf, 0, nn));
                self.writer.write(vec::slice(self.outbuf, 0, nn * 2));
            }
            buf = vec::const_slice(buf, nn, buflen);
        }
    }
}

pub struct Base16Reader {
    priv base16: &'static Base16,
    priv reader: @io::Reader,
    priv buf: [u8, ..1024],
    priv outbuf: [u8, ..512],
    priv nbuf: uint,
    priv noutbuf: uint
}

// TODO: modernize

pub impl Base16Reader {
    fn new(base16: &'static Base16, reader: @io::Reader) -> Base16Reader {
        Base16Reader {
            base16: base16,
            reader: reader,
            buf: [0, ..1024],
            outbuf: [0, ..512],
            nbuf: 0,
            noutbuf: 0
        }
    }

    fn read(&mut self, p: &mut [u8], len: uint) -> uint {
        // NOTE: These borrowing is required to suppress odd loaning errors.
        let selfbuf: &mut [u8] = self.buf;
        let selfoutbuf: &mut [u8] = self.outbuf;
        let selfnbuf: &mut uint = &mut self.nbuf;
        let selfnoutbuf: &mut uint = &mut self.noutbuf;

        // use leftover output (decoded bytes) if it exists
        if self.noutbuf > 0 {
            unsafe { vec::raw::copy_memory(p, selfoutbuf, len); }

            let n = if len > self.noutbuf { self.noutbuf } else { len };
            *selfnoutbuf -= n;
            // shift unread bytes to head
            for uint::range(0, *selfnoutbuf) |i| {
                selfoutbuf[i] = selfoutbuf[i+n];
            }

            return n;
        }

        // calculate least required # of bytes to read
        let nn = len * 2;
        let nn = if nn < 2 { 2 } else { nn };
        let nn = if nn > self.buf.len() { self.buf.len() } else { nn };

        let buf = vec::mut_slice(selfbuf, self.nbuf, nn);
        let nn  = self.reader.read(buf, buf.len());

        *selfnbuf += nn;
        if self.nbuf < 2 {
            fail!(~"malformed base64 input");
        }

        let nr = self.nbuf / 2 * 2; // total read bytes (except fringe bytes)
        let nw = self.nbuf / 2;     // size of decoded bytes

        let buf = vec::mut_slice(selfbuf, 0, nr);

        let ndecoded = if nw > len {
            let res = self.base16.decode(selfoutbuf, buf);
            // copy self.outbuf[0:len] to p
            unsafe { vec::raw::copy_memory(p, selfoutbuf, len); }
            // shift unread bytes to head
            for uint::range(0, res.ndecoded - len) |i| {
                selfoutbuf[i] = selfoutbuf[i+len];
            }
            *selfnoutbuf = res.ndecoded - len;
            len
        } else {
            let res = self.base16.decode(p, buf);
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
        self.noutbuf == 0 && self.reader.eof()
    }
}

fn base16encode(table: &[u8], dst: &mut [u8], src: &const [u8]) {
    for uint::range(0, src.len()) |j| {
        dst[j+1*j]     = table[src[j]>>4];
        dst[j+1*j + 1] = table[src[j] & 0x0f];
    }
}

fn base16decode(decode_map: &[u8], dst: &mut [u8], src: &const [u8]) -> DecodeResult {
    let mut src_length = src.len();
    let mut i = 0u;
    let mut j = 0u;

    while src_length > 0 {
        if char::is_whitespace(src[i] as char) {
            src_length -= 1;
            i += 1;
            loop;
        }

        let chr1 = decode_map[src[i]];
        let chr2 = decode_map[src[i+1]];
        if chr1 == 0xff_u8 || chr2 == 0xff_u8 {
            fail!(~"malformed base16 string");
        }
        dst[j] = chr1<<4 | chr2;

        src_length -= 2;
        i += 2;
        j += 1;
    }

    DecodeResult { end: false, ndecoded: j }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode() {
        let source = str::to_bytes("foo");
        let expect = str::to_bytes("666F6F");

        let actual = encode(source);

        assert_eq!(expect, actual);
    }

    #[test]
    fn test_decode() {
        let source = str::to_bytes("\t66 6f\r\n 6f");
        let expect = str::to_bytes("foo");

        let actual = decode(source);

        assert_eq!(expect, actual);
    }

    #[test]
    fn test_base16_writer() {
        let source1 = str::to_bytes("fo");
        let source2 = str::to_bytes("o");
        let expect = str::to_bytes("666F6F");

        let actual = io::with_bytes_writer(|writer| {
            let mut writer = Base16Writer::new(BASE16, writer);
            writer.write(source1);
            writer.write(source2);
        });

        assert_eq!(expect, actual);
    }

    // FIXME: temporary fix for the testcase
    fn read_bytes(rd: &mut Base16Reader, len: uint) -> ~[u8] {
        let mut buf = vec::with_capacity(len);

        unsafe { vec::raw::set_len(&mut buf, len); }

        let nread = rd.read(buf, len);

        unsafe { vec::raw::set_len(&mut buf, nread); }

        buf
    }

    #[test]
    fn test_base16_reader() {
        let source = str::to_bytes("666f6f");
        let expect = str::to_bytes("foo");

        let actual = io::with_bytes_reader(source, |reader| {
            let mut reader = Base16Reader::new(BASE16, reader);

            io::with_bytes_writer(|writer| {
                while !reader.eof() {
                    let buf = read_bytes(&mut reader, 1);
                    writer.write(buf);
                }
            })
        });

        assert_eq!(expect, actual);
    }
}
