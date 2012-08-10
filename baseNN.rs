// baseNN.rs

enum DecodeResult {
    Continue(uint), // Decode method have not encountered paddings yet
    End(uint),      // Decode method have encountered paddings
}

impl DecodeResult {
    fn get() -> uint {
        match self {
            Continue(n) => n,
            End(n)      => n
        }
    }
}

trait Encode {
    fn encode(dst: &[mut u8], src: &[u8]);
    fn encoded_len(src_length: uint) -> uint;
    fn encode_bytes(src: &[u8]) -> ~[u8];
}

trait Decode {
    fn decode(dst: &[mut u8], src: &[u8]) -> DecodeResult;
    fn decoded_len(src_length: uint) -> uint;
    fn decode_bytes(src: &[u8]) -> ~[u8];
}

trait Base64 {
    fn from_base64() -> ~[u8];
    fn to_base64() -> ~[u8];
    fn from_base64_url() -> ~[u8];
    fn to_base64_url() -> ~[u8];
}

trait Base32 {
    fn from_base32() -> ~[u8];
    fn to_base32() -> ~[u8];
    fn from_base32_hex() -> ~[u8];
    fn to_base32_hex() -> ~[u8];
}

trait Base16 {
    fn from_base16() -> ~[u8];
    fn to_base16() -> ~[u8];
}

type Buffer = &[u8];
type String = &str;

impl Buffer : Base64 {
    fn from_base64() -> ~[u8] {
        base64::decode(self)
    }
    fn to_base64() -> ~[u8] {
        base64::encode(self)
    }
    fn from_base64_url() -> ~[u8] {
        base64::urlsafe_decode(self)
    }
    fn to_base64_url() -> ~[u8] {
        base64::urlsafe_encode(self)
    }
}

impl Buffer : Base32 {
    fn from_base32() -> ~[u8] {
        base32::decode(self)
    }
    fn to_base32() -> ~[u8] {
        base32::encode(self)
    }
    fn from_base32_hex() -> ~[u8] {
        base32::hex_decode(self)
    }
    fn to_base32_hex() -> ~[u8] {
        base32::hex_encode(self)
    }
}

impl Buffer : Base16 {
    fn from_base16() -> ~[u8] {
        base16::decode(self)
    }
    fn to_base16() -> ~[u8] {
        base16::encode(self)
    }
}

impl String : Base64 {
    fn from_base64() -> ~[u8] {
        let bytes = str::bytes(self);
        bytes.from_base64()
    }
    fn to_base64() -> ~[u8] {
        let bytes = str::bytes(self);
        bytes.to_base64()
    }
    fn from_base64_url() -> ~[u8] {
        let bytes = str::bytes(self);
        bytes.from_base64_url()
    }
    fn to_base64_url() -> ~[u8] {
        let bytes = str::bytes(self);
        bytes.to_base64_url()
    }
}

impl String : Base32 {
    fn from_base32() -> ~[u8] {
        let bytes = str::bytes(self);
        bytes.from_base32()
    }
    fn to_base32() -> ~[u8] {
        let bytes = str::bytes(self);
        bytes.to_base32()
    }
    fn from_base32_hex() -> ~[u8] {
        let bytes = str::bytes(self);
        bytes.from_base32_hex()
    }
    fn to_base32_hex() -> ~[u8] {
        let bytes = str::bytes(self);
        bytes.to_base32_hex()
    }
}

impl String : Base16 {
    fn from_base16() -> ~[u8] {
        let bytes = str::bytes(self);
        bytes.from_base16()
    }
    fn to_base16() -> ~[u8] {
        let bytes = str::bytes(self);
        bytes.to_base16()
    }
}
