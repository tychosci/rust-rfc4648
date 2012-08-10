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

// 0123456789ABCDEF
const TABLE: [u8]/16 = [
    48, 49, 50, 51, 52, 53, 54, 55,
    56, 57, 65, 66, 67, 68, 69, 70,
];

const DECODE_MAP: [u8]/256 = [
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
    table: [u8]/16;
    decode_map: [u8]/256;
}

#[inline(always)]
pure fn encoded_len(src_length: uint) -> uint { src_length * 2 }
#[inline(always)]
pure fn decoded_len(src_length: uint) -> uint { src_length / 2 }

impl Base16 : Encode {
    fn encode(dst: &[mut u8], src: &[u8]) {
        b16encode(self.table, dst, src);
    }
    fn encoded_len(src_length: uint) -> uint {
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
    fn encode_bytes(src: &[u8]) -> ~[u8] {
        let dst_len = self.encoded_len(src.len());
        let dst = vec::to_mut(vec::from_elem(dst_len, 0u8));
        self.encode(dst, src);
        vec::from_mut(dst)
    }
}

impl Base16 : Decode {
    fn decode(dst: &[mut u8], src: &[u8]) -> uint {
        b16decode(self.decode_map, dst, src)
    }
    fn decoded_len(src_length: uint) -> uint {
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
    fn decode_bytes(src: &[u8]) -> ~[u8] {
        let dst_len = self.decoded_len(src.len());
        let dst = vec::to_mut(vec::from_elem(dst_len, 0u8));
        let end = self.decode(dst, src);
        vec::slice(vec::from_mut(dst), 0u, end)
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
    base16: &Base16;
    writer: &io::writer;
    outbuf: [mut u8]/1024;
}

fn Base16Writer(base16: &Base16, writer: &io::writer) -> Base16Writer {
    Base16Writer {
        base16: base16,
        writer: writer,
        outbuf: [mut 0, ..1024],
    }
}

impl Base16Writer {
    fn write(buf: &[u8]) {
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

macro_rules! abort {
    { $s:expr } => { fail str::from_slice($s) }
}

fn b16encode(table: &[u8], dst: &[mut u8], src: &[u8]) {
    for uint::range(0, src.len()) |j| {
        dst[j+1*j]     = table[src[j]>>4];
        dst[j+1*j + 1] = table[src[j] & 0x0f];
    }
}

fn b16decode(decode_map: &[u8], dst: &[mut u8], src: &[u8]) -> uint {
    let mut src_length = src.len();
    let mut i = 0u;
    let mut j = 0u;

    while src_length > 0 {
        if char::is_whitespace(src[i] as char) {
            src_length -= 1;
            i += 1;
            again;
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

    j
}

#[cfg(test)]
module tests {
    #[test]
    fn test_encode() {
        let source = str::bytes("foo");
        let expect = str::bytes("666F6F");
        let actual = encode(source);
        assert expect == actual;
    }
    #[test]
    fn test_decode() {
        let source = str::bytes("\t66 6f\r\n 6f");
        let expect = str::bytes("foo");
        let actual = decode(source);
        assert expect == actual;
    }
    #[test]
    fn test_base16_writer() {
        let source1 = str::bytes("fo");
        let source2 = str::bytes("o");
        let expect  = str::bytes("666F6F");
        let actual  = io::with_buf_writer(|writer| {
            let writer = Base16Writer(BASE16, &writer);
            writer.write(source1);
            writer.write(source2);
        });

        assert expect == actual;
    }
}
