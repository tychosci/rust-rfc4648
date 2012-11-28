// binary.rs

pub trait Encode {
    fn encode(&self, buf: &[const u8]) -> ~[u8];
}

pub trait Decode {
    fn decode(&self, buf: &[const u8]) -> ~[u8];
}

pub trait Convert {
    static fn convert(buf: &[const u8], to: self, from: self) -> ~[u8];
}

pub trait Codec : Encode, Decode, Convert {
}

// `end` indicates whether the Decode method have encountered paddings or not.
struct DecodeResult {
    end: bool,
    ndecoded: uint,
}

trait BinaryEncoder {
    fn encode(&self, dst: &[mut u8], src: &[const u8]);
    fn encoded_len(&self, src_length: uint) -> uint;
    fn encode_bytes(&self, src: &[const u8]) -> ~[u8];
}

trait BinaryDecoder {
    fn decode(&self, dst: &[mut u8], src: &[const u8]) -> DecodeResult;
    fn decoded_len(&self, src_length: uint) -> uint;
    fn decode_bytes(&self, src: &[const u8]) -> ~[u8];
}

pub enum Binary {
    Base16,
    Base32,
    Base64,
    Base32Hex,
    Base64Url,
}

pub impl Binary : Codec {
    fn encode(&self, buf: &[const u8]) -> ~[u8] {
        match *self {
            Base16    => base16::encode(buf),
            Base32    => base32::encode(buf),
            Base64    => base64::encode(buf),
            Base32Hex => base32::hex_encode(buf),
            Base64Url => base64::urlsafe_encode(buf)
        }
    }

    fn decode(&self, buf: &[const u8]) -> ~[u8] {
        match *self {
            Base16    => base16::decode(buf),
            Base32    => base32::decode(buf),
            Base64    => base64::decode(buf),
            Base32Hex => base32::hex_decode(buf),
            Base64Url => base64::urlsafe_decode(buf)
        }
    }

    static fn convert(buf: &[const u8], to: Binary, from: Binary) -> ~[u8] {
        let buf = from.decode(buf);
        let buf = to.encode(buf);
        move buf
    }
}
