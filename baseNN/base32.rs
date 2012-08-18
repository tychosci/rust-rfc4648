/*!
 * Base32 Module
 *
 * See <http://tools.ietf.org/html/rfc4648#section-6> for details.
 *
 * # Example
 *
 * ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
 * use encoding;
 * import encoding::Codec;
 *
 * let src = "base32";
 * let res = src.encode(encoding::Base32);
 * let res = str::from_bytes(res);
 *
 * io::println(fmt!("%s", res));
 * ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
 */

export BASE32_STD, BASE32_HEX, Base32Writer, Base32Reader;
export encode, hex_encode, decode, hex_decode;

macro_rules! abort {
    { $s:expr } => { fail str::from_slice($s) }
}

const PAD: u8 = 61u8;

// ABCDEFGHIJKLMNOPQRSTUVWXYZ234567
const TABLE_STD: [u8]/32 = [
    65, 66, 67, 68, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 80,
    81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 50, 51, 52, 53, 54, 55,
];

// 0123456789ABCDEFGHIJKLMNOPQRSTUV
const TABLE_HEX: [u8]/32 = [
    48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 65, 66, 67, 68, 69, 70,
    71, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86,
];

const DECODE_MAP_STD: [u8]/256 = [
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

const DECODE_MAP_HEX: [u8]/256 = [
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

const BASE32_STD: &Base32 = &Base32 {
    table: TABLE_STD,
    decode_map: DECODE_MAP_STD,
};

const BASE32_HEX: &Base32 = &Base32 {
    table: TABLE_HEX,
    decode_map: DECODE_MAP_HEX,
};

struct Base32 {
    table: [u8]/32;
    decode_map: [u8]/256;
}

#[inline(always)]
pure fn encoded_len(src_length: uint) -> uint {
    (src_length + 4) / 5 * 8
}

#[inline(always)]
pure fn decoded_len(src_length: uint) -> uint {
    src_length / 8 * 5
}

impl Base32 : BaseNNEncode {
    fn encode(dst: &[mut u8], src: &[u8]) {
        b32encode(self.table, dst, src);
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
        let mut dst = ~[mut];
        let dst_length = self.encoded_len(src.len());

        vec::reserve(dst, dst_length);
        unsafe { vec::unsafe::set_len(dst, dst_length); }

        self.encode(dst, src);

        vec::from_mut(dst)
    }
}

impl Base32 : BaseNNDecode {
    fn decode(dst: &[mut u8], src: &[u8]) -> DecodeResult {
        b32decode(self.decode_map, dst, src)
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
    BASE32_STD.encode_bytes(src)
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
 * base32-encoded bytes (extended hex alphabet)
 */
fn hex_encode(src: &[u8]) -> ~[u8] {
    BASE32_HEX.encode_bytes(src)
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
    BASE32_STD.decode_bytes(src)
}

/**
 * Shortcut for base32#decode_bytes
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
    BASE32_HEX.decode_bytes(src)
}

struct Base32Writer {
    base32: &Base32;
    writer: io::Writer;
    outbuf: [mut u8]/1024;
    buf: [mut u8]/5;
    mut nbuf: uint;
}

fn Base32Writer(base32: &Base32, writer: io::Writer) -> Base32Writer {
    Base32Writer {
        base32: base32,
        writer: writer,
        outbuf: [mut 0, ..1024],
        buf: [mut 0, ..5],
        nbuf: 0,
    }
}

impl Base32Writer {
    fn write(buf: &[u8]) {
        let buflen = buf.len();
        let mut buf = vec::view(buf, 0, buflen);

        if self.nbuf > 0 {
            let mut i = 0;
            while i < buflen && self.nbuf < 5 {
                self.buf[self.nbuf] = buf[i];
                self.nbuf += 1;
                i += 1;
            }

            buf = vec::view(buf, i, buflen);
            if self.nbuf < 5 {
                return;
            }

            self.base32.encode(self.outbuf, vec::slice(self.buf, 0, 5));
            self.writer.write(vec::mut_view(self.outbuf, 0, 8));
            self.nbuf = 0;
        }

        while buf.len() >= 5 {
            let buflen = buf.len();
            let nn = buflen / 5 * 8;
            let nn = if nn > buflen { buflen } else { nn };
            let nn = nn - nn % 8;

            if nn > 0 {
                self.base32.encode(self.outbuf, vec::view(buf, 0, nn));
                self.writer.write(vec::mut_view(self.outbuf, 0, nn / 8 * 5));
            }

            buf = vec::view(buf, nn, buflen);
        }

        for uint::range(0, buf.len()) |i| {
            self.buf[i] = buf[i];
        }
        self.nbuf = buf.len();
    }
    fn close() {
        if self.nbuf > 0 {
            let nbuf = self.nbuf;
            self.nbuf = 0;

            let buf = vec::slice(self.buf, 0, nbuf);
            self.base32.encode(self.outbuf, buf);
            self.writer.write(vec::mut_view(self.outbuf, 0, 8));
        }
    }
}

struct Base32Reader {
    base32: &Base32;
    reader: io::Reader;
    buf: [mut u8]/1024;
    outbuf: [mut u8]/640;
    mut nbuf: uint;
    mut noutbuf: uint;
    mut end: bool;
}

fn Base32Reader(base32: &Base32, reader: io::Reader) -> Base32Reader {
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

impl Base32Reader {
    fn read(p: &[mut u8], len: uint) -> uint {
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
        let nn = len / 5 * 8;
        let nn = if nn < 8 { 8 } else { nn };
        let nn = if nn > self.buf.len() { self.buf.len() } else { nn };

        let buf = vec::mut_view(self.buf, self.nbuf, nn);
        let nn  = self.reader.read(buf, buf.len());

        self.nbuf += nn;
        if self.nbuf < 8 {
            abort!("malformed base32 input");
        }

        let nr = self.nbuf / 8 * 8; // total read bytes (except fringe bytes)
        let nw = self.nbuf / 8 * 5; // size of decoded bytes

        // FIXME this copy is unfortunate
        let buf = vec::slice(self.buf, 0, nr);

        let ndecoded = if nw > len {
            let res = self.base32.decode(self.outbuf, buf);
            // copy self.outbuf[0:len] to p
            vec::u8::memcpy(p, self.outbuf, len);
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
    fn read_bytes(len: uint) -> ~[u8] {
        let mut buf = ~[mut];

        vec::reserve(buf, len);
        unsafe { vec::unsafe::set_len(buf, len); }

        let nread = self.read(buf, len);

        unsafe { vec::unsafe::set_len(buf, nread); }

        vec::from_mut(buf)
    }
    fn eof() -> bool {
        self.noutbuf == 0 && (self.end || self.reader.eof())
    }
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

fn b32encode(table: &[u8], dst: &[mut u8], src: &[u8]) {
    let src_length = src.len();
    let dst_length = dst.len();

    if src_length == 0 {
        return;
    }

    if dst_length % 8 != 0 {
        abort!("dst's length should be divisible by 8");
    }

    for uint::range(0, (src_length + 4) / 5) |i| {
        let src_curr = 5 * i;
        let dst_curr = 8 * i;
        let remain = src_length - src_curr;

        dst[dst_curr+0] = 0; dst[dst_curr+1] = 0;
        dst[dst_curr+2] = 0; dst[dst_curr+3] = 0;
        dst[dst_curr+4] = 0; dst[dst_curr+5] = 0;
        dst[dst_curr+6] = 0; dst[dst_curr+7] = 0;

        switch! { remain =>
        default: { dst[dst_curr+7] |= src[src_curr+4]    & 0x1f
                 ; dst[dst_curr+6] |= src[src_curr+4]>>5 }
        case 04: { dst[dst_curr+6] |= src[src_curr+3]<<3 & 0x1f
                 ; dst[dst_curr+5] |= src[src_curr+3]>>2 & 0x1f
                 ; dst[dst_curr+4] |= src[src_curr+3]>>7 }
        case 03: { dst[dst_curr+4] |= src[src_curr+2]<<1 & 0x1f
                 ; dst[dst_curr+3] |= src[src_curr+2]>>4 & 0x1f }
        case 02: { dst[dst_curr+3] |= src[src_curr+1]<<4 & 0x1f
                 ; dst[dst_curr+2] |= src[src_curr+1]>>1 & 0x1f
                 ; dst[dst_curr+1] |= src[src_curr+1]>>6 & 0x1f }
        case 01: { dst[dst_curr+1] |= src[src_curr+0]<<2 & 0x1f
                 ; dst[dst_curr+0] |= src[src_curr+0]>>3 }
        };

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

fn b32decode(decode_map: &[u8], dst: &[mut u8], src: &[u8]) -> DecodeResult {
    let mut ndecoded = 0u;
    let mut dst = vec::mut_view(dst, 0, dst.len());
    let mut src = vec::view(src, 0, src.len());
    let mut end = false;

    while src.len() > 0 && !end {
        let buf = [mut 0xff_u8, ..8];
        let mut buf_len = 8u;

        let mut i = 0u;
        while i < 8 {
            if src.len() == 0 {
                abort!("malformed base32 string");
            }
            let chr = src[0];
            src = vec::view(src, 1, src.len());
            if char::is_whitespace(chr as char) {
                again;
            }
            if chr == PAD && i >= 2 && src.len() < 8 {
                for uint::range(0, (8-i-1)) |j| {
                    if src.len() > j && src[j] != PAD {
                        abort!("malformed base32 string");
                    }
                }
                buf_len = i;
                end = true;
                break;
            }
            buf[i] = decode_map[chr];
            if buf[i] == 0xff {
                abort!("malformed base32 string");
            }
            i += 1;
        }

        dst[0] = 0; dst[1] = 0; dst[2] = 0;
        dst[3] = 0; dst[4] = 0;

        switch! { buf_len =>
        default:   { abort!("malformed base32 string") }
        case 7, 8: { dst[4] |= buf[6]<<5 | buf[7]
                   ; dst[3] |= buf[6]>>3 }
        case 5, 6: { dst[3] |= buf[4]<<7 | buf[5]<<2
                   ; dst[2] |= buf[4]>>1 }
        case 4:    { dst[2] |= buf[3]<<4
                   ; dst[1] |= buf[3]>>4 }
        case 3:    { dst[1] |= buf[1]<<6 | buf[2]<<1 }
        case 2:    { dst[0] |= buf[0]<<3 | buf[1]>>2 }
        };

        dst = vec::mut_view(dst, 5, dst.len());
        match buf_len {
            2     => ndecoded += 1,
            3 | 4 => ndecoded += 2,
            5     => ndecoded += 3,
            6 | 7 => ndecoded += 4,
            8     => ndecoded += 5,
            _     => abort!("malformed base32 string")
        }
    }

    DecodeResult { end: end, ndecoded: ndecoded }
}

#[cfg(test)]
mod tests {
    fn t(source: &[&str], expect: &[&str], cb: fn&((&[u8])) -> ~[u8]) {
        let source = source.map(str::bytes);
        let expect = expect.map(str::bytes);
        let actual = source.map(|e| cb(e));
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
        let source1 = str::bytes("f");
        let source2 = str::bytes("ooba");
        let expect  = str::bytes("MZXW6YTB");

        let actual  = io::with_buf_writer(|writer| {
            let writer = Base32Writer(BASE32_STD, writer);
            writer.write(source1);
            writer.write(source2);
            // FIXME Remove this line once we get drop intrinsics.
            writer.close();
        });

        assert expect == actual;
    }
    #[test]
    fn test_base32_reader() {
        let source = ["MY======", "MZXQ====", "MZXW6===",
                      "MZXW6YQ=", "MZXW6YTB", "MZXW6YTBOI======"];
        let expect = ["f", "fo", "foo", "foob", "fooba", "foobar"];

        let source = source.map(str::bytes);
        let expect = expect.map(str::bytes);

        let actual = source.map(|e| {
            io::with_bytes_reader(e, |reader| {
                let reader = Base32Reader(BASE32_STD, reader);

                io::with_buf_writer(|writer| {
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
