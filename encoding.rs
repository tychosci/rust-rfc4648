enum Encoding {
    Base16,
    Base32,
    Base64,
    Base32Hex,
    Base64Urlsafe,
}

trait Encode {
    fn encode(dst: &[mut u8], src: &[u8]);
    fn encoded_len(src_length: uint) -> uint;
    fn encode_bytes(src: &[u8]) -> ~[u8];
}

trait Decode {
    fn decode(dst: &[mut u8], src: &[u8]) -> uint;
    fn decoded_len(src_length: uint) -> uint;
    fn decode_bytes(src: &[u8]) -> ~[u8];
}

trait Codec {
    fn encode(e: Encoding) -> ~[u8];
    fn decode(e: Encoding) -> ~[u8];
}

type Buffer = &[u8];
type String = &str;

impl Buffer : Codec {
    fn encode(e: Encoding) -> ~[u8] {
        match e {
            Base16        => base16::encode(self)
          , Base32        => base32::encode(self)
          , Base64        => base64::encode(self)
          , Base32Hex     => base32::hex_encode(self)
          , Base64Urlsafe => base64::urlsafe_encode(self)
        }
    }
    fn decode(e: Encoding) -> ~[u8] {
        match e {
            Base16        => base16::decode(self)
          , Base32        => base32::decode(self)
          , Base64        => base64::decode(self)
          , Base32Hex     => base32::hex_decode(self)
          , Base64Urlsafe => base64::urlsafe_decode(self)
        }
    }
}

impl String : Codec {
    fn encode(e: Encoding) -> ~[u8] {
        let bytes = str::bytes(self);
        bytes.encode(e)
    }
    fn decode(e: Encoding) -> ~[u8] {
        let bytes = str::bytes(self);
        bytes.decode(e)
    }
}
