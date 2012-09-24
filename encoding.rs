// encoding.rs

#[legacy_exports];

// import enum variants in misc.rs
use Base16 = misc::Base16;
use Base32 = misc::Base32;
use Base64 = misc::Base64;
use Base32Hex = misc::Base32Hex;
use Base64Url = misc::Base64Url;

// import constants in misc/base{16,32,64}.rs
use BASE16 = misc::base16::BASE16;
use BASE32 = misc::base32::BASE32_STD;
use BASE64 = misc::base64::BASE64_STD;
use BASE32_HEX = misc::base32::BASE32_HEX;
use BASE64_URL = misc::base64::BASE64_URL;

// import structs in misc/base{16,32,64}.rs
use Base16Writer = misc::base16::Base16Writer;
use Base32Writer = misc::base32::Base32Writer;
use Base64Writer = misc::base64::Base64Writer;
use Base32Reader = misc::base32::Base32Reader;
use Base64Reader = misc::base64::Base64Reader;

export Base16;
export Base32;
export Base64;
export Base32Hex;
export Base64Url;
export BASE16;
export BASE32;
export BASE64;
export BASE32_HEX;
export BASE64_URL;
export Base16Writer;
export Base32Writer;
export Base64Writer;
export Base32Reader;
export Base64Reader;

export Encode;
export Decode;
export Convert;
export Codec;

pub trait Encode {
    fn encode(buf: &[u8]) -> ~[u8];
}

pub trait Decode {
    fn decode(buf: &[u8]) -> ~[u8];
}

pub trait Convert {
    static fn convert(buf: &[u8], to: self, from: self) -> ~[u8];
}

pub trait Codec<T: Copy Encode Decode Convert> {
    fn encode(encoder: T) -> ~[u8];
    fn decode(decoder: T) -> ~[u8];
    fn convert(to: T, from: T) -> ~[u8];
}

pub impl<T: Copy Encode Decode Convert> &[u8] : Codec<T> {
    fn encode(encoder: T) -> ~[u8] {
        move encoder.encode(self)
    }

    fn decode(decoder: T) -> ~[u8] {
        move decoder.decode(self)
    }

    fn convert(to: T, from: T) -> ~[u8] {
        move convert(self, to, from)
    }
}

pub impl<T: Copy Encode Decode Convert> &str : Codec<T> {
    fn encode(encoder: T) -> ~[u8] {
        move str::byte_slice(self, |b| encoder.encode(b))
    }

    fn decode(decoder: T) -> ~[u8] {
        move str::byte_slice(self, |b| decoder.decode(b))
    }

    fn convert(to: T, from: T) -> ~[u8] {
        move str::byte_slice(self, |b| convert(b, to, from))
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
        let s = "foobar";
        let source = s.encode(Base64);
        let expect = s.encode(Base32);

        let actual = source.convert(Base32, Base64);

        assert expect == actual;
    }
}
