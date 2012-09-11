// encoding.rs

// import enum variants in misc.rs
use Base16         = misc::Base16;
use Base32         = misc::Base32;
use Base64         = misc::Base64;
use Base32Hex      = misc::Base32Hex;
use Base64Url      = misc::Base64Url;

// import constants in misc/base{16,32,64}.rs
use BASE16         = misc::base16::BASE16;
use BASE32         = misc::base32::BASE32_STD;
use BASE64         = misc::base64::BASE64_STD;
use BASE32_HEX     = misc::base32::BASE32_HEX;
use BASE64_URL     = misc::base64::BASE64_URL;

// import structs in misc/base{16,32,64}.rs
use Base16Writer   = misc::base16::Base16Writer;
use Base32Writer   = misc::base32::Base32Writer;
use Base64Writer   = misc::base64::Base64Writer;
use Base32Reader   = misc::base32::Base32Reader;
use Base64Reader   = misc::base64::Base64Reader;

// export all enum variants in misc.rs
export Base16;
export Base32;
export Base64;
export Base32Hex;
export Base64Url;

// export constants
export BASE16;
export BASE32;
export BASE64;
export BASE32_HEX;
export BASE64_URL;

// export structs
export Base16Writer;
export Base32Writer;
export Base64Writer;
export Base32Reader;
export Base64Reader;

export Convert;
export Encode;
export Decode;
export Codec;

struct Convert<T: Encode Decode> {
    from: T,
    to: T,
}

trait Encode {
    fn encode(buf: &[u8]) -> ~[u8];
}

trait Decode {
    fn decode(buf: &[u8]) -> ~[u8];
}

trait Codec<T: Encode Decode> {
    fn encode(encoder: T) -> ~[u8];
    fn decode(decoder: T) -> ~[u8];
}

impl<T: Encode Decode> Convert<T> : Encode {
    fn encode(buf: &[u8]) -> ~[u8] {
        let buf = self.from.decode(buf);
        let buf = self.to.encode(buf);
        move buf
    }
}

impl<T: Encode Decode> Convert<T> : Decode {
    fn decode(buf: &[u8]) -> ~[u8] {
        let buf = self.from.decode(buf);
        let buf = self.to.encode(buf);
        move buf
    }
}

impl<T: Encode Decode> &[u8] : Codec<T> {
    fn encode(encoder: T) -> ~[u8] {
        move encoder.encode(self)
    }
    fn decode(decoder: T) -> ~[u8] {
        move decoder.decode(self)
    }
}

impl<T: Encode Decode> &str : Codec<T> {
    fn encode(encoder: T) -> ~[u8] {
        move str::byte_slice(self, |b| encoder.encode(b))
    }
    fn decode(decoder: T) -> ~[u8] {
        move str::byte_slice(self, |b| decoder.decode(b))
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_codec_base64() {
        let source = str::to_bytes("foobar");
        let expect = str::to_bytes("Zm9vYmFy");

        let actual = source.encode(Base64);

        assert expect == actual;
    }
    #[test]
    fn test_codec_convert() {
        let string = "foobar";
        let source = string.encode(Base32);
        let expect = string.encode(Base64);

        let actual = source.encode(Convert{from: Base32, to: Base64});

        assert expect == actual;
    }
}
