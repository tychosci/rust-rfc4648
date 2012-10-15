// encoding.rs

// import enum variants in misc.rs
pub use Base16 = misc::Base16;
pub use Base32 = misc::Base32;
pub use Base64 = misc::Base64;
pub use Base32Hex = misc::Base32Hex;
pub use Base64Url = misc::Base64Url;

// import constants in misc/base{16,32,64}.rs
pub use BASE16 = misc::base16::BASE16;
pub use BASE32 = misc::base32::BASE32_STD;
pub use BASE64 = misc::base64::BASE64_STD;
pub use BASE32_HEX = misc::base32::BASE32_HEX;
pub use BASE64_URL = misc::base64::BASE64_URL;

// import structs in misc/base{16,32,64}.rs
pub use Base16Writer = misc::base16::Base16Writer;
pub use Base32Writer = misc::base32::Base32Writer;
pub use Base64Writer = misc::base64::Base64Writer;
pub use Base32Reader = misc::base32::Base32Reader;
pub use Base64Reader = misc::base64::Base64Reader;

pub trait Encode {
    fn encode(buf: &[const u8]) -> ~[u8];
}

pub trait Decode {
    fn decode(buf: &[const u8]) -> ~[u8];
}

pub trait Convert {
    static fn convert(buf: &[const u8], to: self, from: self) -> ~[u8];
}

pub trait Codec<T: Copy Encode Decode Convert> {
    fn encode(encoder: T) -> ~[u8];
    fn decode(decoder: T) -> ~[u8];
    fn convert(to: T, from: T) -> ~[u8];
}

pub impl<T: Copy Encode Decode Convert> &[const u8] : Codec<T> {
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
