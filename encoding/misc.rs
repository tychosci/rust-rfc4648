// misc.rs

// `end` indicates whether the Decode method have encountered paddings or not.
struct DecodeResult {
    end: bool,
    ndecoded: uint,
}

trait MiscEncode {
    fn encode(&self, dst: &[mut u8], src: &[u8]);
    fn encoded_len(&self, src_length: uint) -> uint;
    fn encode_bytes(&self, src: &[u8]) -> ~[u8];
}

trait MiscDecode {
    fn decode(&self, dst: &[mut u8], src: &[u8]) -> DecodeResult;
    fn decoded_len(&self, src_length: uint) -> uint;
    fn decode_bytes(&self, src: &[u8]) -> ~[u8];
}

enum Misc {
    Base16,
    Base32,
    Base64,
    Base32Hex,
    Base64Url,
}

impl Misc : Encode {
    fn encode(buf: &[u8]) -> ~[u8] {
        move match self {
            Base16    => base16::encode(buf),
            Base32    => base32::encode(buf),
            Base64    => base64::encode(buf),
            Base32Hex => base32::hex_encode(buf),
            Base64Url => base64::urlsafe_encode(buf)
        }
    }
}

impl Misc : Decode {
    fn decode(buf: &[u8]) -> ~[u8] {
        move match self {
            Base16    => base16::decode(buf),
            Base32    => base32::decode(buf),
            Base64    => base64::decode(buf),
            Base32Hex => base32::hex_decode(buf),
            Base64Url => base64::urlsafe_decode(buf)
        }
    }
}

impl Misc : Convert {
    static fn convert(buf: &[u8], to: Misc, from: Misc) -> ~[u8] {
        let buf = from.decode(buf);
        let buf = to.encode(buf);
        move buf
    }
}
