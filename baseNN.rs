// baseNN.rs

// `end` indicates whether the Decode method have encountered paddings or not.
struct DecodeResult {
    end: bool;
    ndecoded: uint;
}

trait BaseNNEncode {
    fn encode(dst: &[mut u8], src: &[u8]);
    fn encoded_len(src_length: uint) -> uint;
    fn encode_bytes(src: &[u8]) -> ~[u8];
}

trait BaseNNDecode {
    fn decode(dst: &[mut u8], src: &[u8]) -> DecodeResult;
    fn decoded_len(src_length: uint) -> uint;
    fn decode_bytes(src: &[u8]) -> ~[u8];
}

enum BaseNN {
    Base16,
    Base32,
    Base64,
    Base32Hex,
    Base64Url,
}

impl BaseNN : Encode {
    fn encode(buf: &[u8]) -> ~[u8] {
        match self {
            Base16    => base16::encode(buf),
            Base32    => base32::encode(buf),
            Base64    => base64::encode(buf),
            Base32Hex => base32::hex_encode(buf),
            Base64Url => base64::urlsafe_encode(buf)
        }
    }
}

impl BaseNN : Decode {
    fn decode(buf: &[u8]) -> ~[u8] {
        match self {
            Base16    => base16::decode(buf),
            Base32    => base32::decode(buf),
            Base64    => base64::decode(buf),
            Base32Hex => base32::hex_decode(buf),
            Base64Url => base64::urlsafe_decode(buf)
        }
    }
}
