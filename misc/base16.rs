/*!
 * Base16 Module
 *
 * See <http://tools.ietf.org/html/rfc4648#section-8> for details.
 *
 * # Example
 *
 * ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
 * use encoding;
 * import encoding::Codec;
 *
 * let src = "base16";
 * let res = src.encode(encoding::Base16);
 * let res = str::from_bytes(res);
 *
 * io::println(fmt!("%s", res));
 * ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
 */

export BASE16, Base16Writer;
export encode, decode;

macro_rules! abort (
    { $s:expr } => { fail str::from_slice($s) }
)

// 0123456789ABCDEF
const TABLE: [u8*16] = [
    48, 49, 50, 51, 52, 53, 54, 55,
    56, 57, 65, 66, 67, 68, 69, 70,
];

const DECODE_MAP: [u8*256] = [
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

const BASE16: &Base16 = &Base16 {
    table: TABLE,
    decode_map: DECODE_MAP,
};

struct Base16 {
    priv table: [u8*16],
    priv decode_map: [u8*256],
}

#[inline(always)]
pure fn encoded_len(src_length: uint) -> uint { src_length * 2 }
#[inline(always)]
pure fn decoded_len(src_length: uint) -> uint { src_length / 2 }

impl Base16 : BaseNNEncode {
    fn encode(&self, dst: &[mut u8], src: &[u8]) {
        b16encode(self.table, dst, src);
    }
    fn encoded_len(&self, src_length: uint) -> uint {
        encoded_len(src_length)
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
    fn encode_bytes(&self, src: &[u8]) -> ~[u8] {
        let mut dst = ~[mut];
        let dst_length = self.encoded_len(src.len());

        vec::reserve(dst, dst_length);
        unsafe { vec::unsafe::set_len(dst, dst_length); }

        self.encode(dst, src);

        vec::from_mut(dst)
    }
}

impl Base16 : BaseNNDecode {
    fn decode(&self, dst: &[mut u8], src: &[u8]) -> DecodeResult {
        b16decode(self.decode_map, dst, src)
    }
    fn decoded_len(&self, src_length: uint) -> uint {
        decoded_len(src_length)
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
    fn decode_bytes(&self, src: &[u8]) -> ~[u8] {
        let mut dst = ~[mut];
        let dst_length = self.decoded_len(src.len());

        vec::reserve(dst, dst_length);
        unsafe { vec::unsafe::set_len(dst, dst_length); }

        let res = self.decode(dst, src);

        unsafe { vec::unsafe::set_len(dst, res.ndecoded); }

        vec::from_mut(dst)
    }
}

/**
 * Shortcut for base16#encode_bytes
 *
 * # Arguments
 *
 * * src - bytes for encoding
 *
 * # Return
 *
 * hex-encoded bytes
 */
fn encode(src: &[u8]) -> ~[u8] {
    BASE16.encode_bytes(src)
}

/**
 * Shortcut for base16#decode_bytes
 *
 * # Arguments
 *
 * * src - hex-encoded bytes
 *
 * # Return
 *
 * decoded bytes
 */
fn decode(src: &[u8]) -> ~[u8] {
    BASE16.decode_bytes(src)
}

struct Base16Writer {
    priv base16: &Base16,
    priv writer: io::Writer,
    priv outbuf: [mut u8*1024],
}

fn Base16Writer(base16: &a/Base16, writer: io::Writer) -> Base16Writer/&a {
    Base16Writer {
        base16: base16,
        writer: writer,
        outbuf: [mut 0, ..1024],
    }
}

impl Base16Writer {
    fn write(&self, buf: &[u8]) {
        let mut buf = vec::view(buf, 0, buf.len());

        while buf.len() > 0 {
            let buflen = buf.len();
            let nn = self.outbuf.len() / 2;
            let nn = if nn > buflen { buflen } else { nn };

            if nn > 0 {
                self.base16.encode(self.outbuf, vec::view(buf, 0, nn));
                self.writer.write(vec::mut_view(self.outbuf, 0, nn * 2));
            }

            buf = vec::view(buf, nn, buflen);
        }
    }
}

struct Base16Reader {
    priv base16: &Base16,
    priv reader: io::Reader,
    priv buf: [mut u8*1024],
    priv outbuf: [mut u8*512],
    priv mut nbuf: uint,
    priv mut noutbuf: uint,
}

fn Base16Reader(base16: &a/Base16, reader: io::Reader) -> Base16Reader/&a {
    Base16Reader {
        base16: base16,
        reader: reader,
        buf: [mut 0, ..1024],
        outbuf: [mut 0, ..512],
        nbuf: 0,
        noutbuf: 0,
    }
}

impl Base16Reader {
    fn read(&self, p: &[mut u8], len: uint) -> uint {
        // use leftover output (decoded bytes) if it exists
        if self.noutbuf > 0 {
            vec::u8::memcpy(p, self.outbuf, len);

            let n = if len > self.noutbuf { self.noutbuf } else { len };
            self.noutbuf -= n;
            // shift unread bytes to head
            for uint::range(0, self.noutbuf) |i| {
                self.outbuf[i] = self.outbuf[i+n];
            }

            return n;
        }
        // calculate least required # of bytes to read
        let nn = len * 2;
        let nn = if nn < 2 { 2 } else { nn };
        let nn = if nn > self.buf.len() { self.buf.len() } else { nn };

        let buf = vec::mut_view(self.buf, self.nbuf, nn);
        let nn  = self.reader.read(buf, buf.len());

        self.nbuf += nn;
        if self.nbuf < 2 {
            abort!("malformed base64 input");
        }

        let nr = self.nbuf / 2 * 2; // total read bytes (except fringe bytes)
        let nw = self.nbuf / 2;     // size of decoded bytes

        // FIXME this copy is unfortunate
        let buf = vec::slice(self.buf, 0, nr);

        let ndecoded = if nw > len {
            let res = self.base16.decode(self.outbuf, buf);
            // copy self.outbuf[0:len] to p
            vec::u8::memcpy(p, self.outbuf, len);
            // shift unread bytes to head
            for uint::range(0, res.ndecoded - len) |i| {
                self.outbuf[i] = self.outbuf[i+len];
            }
            self.noutbuf = res.ndecoded - len;
            len
        } else {
            let res = self.base16.decode(p, buf);
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
        let mut buf = ~[mut];

        vec::reserve(buf, len);
        unsafe { vec::unsafe::set_len(buf, len); }

        let nread = self.read(buf, len);

        unsafe { vec::unsafe::set_len(buf, nread); }

        vec::from_mut(buf)
    }
    fn eof(&self) -> bool {
        self.noutbuf == 0 && self.reader.eof()
    }
}

fn b16encode(table: &[u8], dst: &[mut u8], src: &[u8]) {
    for uint::range(0, src.len()) |j| {
        dst[j+1*j]     = table[src[j]>>4];
        dst[j+1*j + 1] = table[src[j] & 0x0f];
    }
}

fn b16decode(decode_map: &[u8], dst: &[mut u8], src: &[u8]) -> DecodeResult {
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
            abort!("malformed base16 string");
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
    #[test]
    fn test_encode() {
        let source = str::to_bytes("foo");
        let expect = str::to_bytes("666F6F");

        let actual = encode(source);

        assert expect == actual;
    }
    #[test]
    fn test_decode() {
        let source = str::to_bytes("\t66 6f\r\n 6f");
        let expect = str::to_bytes("foo");

        let actual = decode(source);

        assert expect == actual;
    }
    #[test]
    fn test_base16_writer() {
        let source1 = str::to_bytes("fo");
        let source2 = str::to_bytes("o");
        let expect  = str::to_bytes("666F6F");

        let actual  = io::with_buf_writer(|writer| {
            let writer = &Base16Writer(BASE16, writer);
            writer.write(source1);
            writer.write(source2);
        });

        assert expect == actual;
    }
    #[test]
    fn test_base16_reader() {
        let source = str::to_bytes("666f6f");
        let expect = str::to_bytes("foo");

        let actual = io::with_bytes_reader(source, |reader| {
            let reader = &Base16Reader(BASE16, reader);

            io::with_buf_writer(|writer| {
                while !reader.eof() {
                    let buf = reader.read_bytes(1);
                    writer.write(buf);
                }
            })
        });

        assert expect == actual;
    }
}
