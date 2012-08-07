enum encoding {
    base16,
    base32,
    base64,
    base32hex,
    base64urlsafe,
}

trait encode {
    fn encode(dst: &[mut u8], src: &[u8]);
    fn encoded_len(src_length: uint) -> uint;
    fn encode_bytes(src: &[u8]) -> ~[u8];
}

trait decode {
    fn decode(dst: &[mut u8], src: &[u8]) -> uint;
    fn decoded_len(src_length: uint) -> uint;
    fn decode_bytes(src: &[u8]) -> ~[u8];
}

trait codec {
    fn encode(e: encoding) -> ~[u8];
    fn decode(e: encoding) -> ~[u8];
}

type Buffer = &[u8];
type String = &str;

impl Buffer : codec {
    fn encode(e: encoding) -> ~[u8] {
        match e {
            base16        => base16::encode(self)
          , base32        => base32::encode(self)
          , base64        => base64::encode(self)
          , base32hex     => base32::hex_encode(self)
          , base64urlsafe => base64::urlsafe_encode(self)
        }
    }
    fn decode(e: encoding) -> ~[u8] {
        match e {
            base16        => base16::decode(self)
          , base32        => base32::decode(self)
          , base64        => base64::decode(self)
          , base32hex     => base32::hex_decode(self)
          , base64urlsafe => base64::urlsafe_decode(self)
        }
    }
}

impl String : codec {
    fn encode(e: encoding) -> ~[u8] {
        let bytes = str::bytes(self);
        bytes.encode(e)
    }
    fn decode(e: encoding) -> ~[u8] {
        let bytes = str::bytes(self);
        bytes.decode(e)
    }
}
