// codec.rs

pub use Base16       = binary::Base16;
pub use Base32       = binary::Base32;
pub use Base64       = binary::Base64;
pub use Base32Hex    = binary::Base32Hex;
pub use Base64Url    = binary::Base64Url;
pub use BASE16       = binary::base16::BASE16;
pub use BASE32       = binary::base32::BASE32_STD;
pub use BASE64       = binary::base64::BASE64_STD;
pub use BASE32_HEX   = binary::base32::BASE32_HEX;
pub use BASE64_URL   = binary::base64::BASE64_URL;
pub use Base16Writer = binary::base16::Base16Writer;
pub use Base32Writer = binary::base32::Base32Writer;
pub use Base64Writer = binary::base64::Base64Writer;
pub use Base32Reader = binary::base32::Base32Reader;
pub use Base64Reader = binary::base64::Base64Reader;

pub trait BinaryCodec<T: Copy binary::Codec> {
    fn encode(encoder: T) -> ~[u8];
    fn decode(decoder: T) -> ~[u8];
    fn convert(to: T, from: T) -> ~[u8];
}

pub impl<T: Copy binary::Codec> &[const u8] : BinaryCodec<T> {
    fn encode(encoder: T) -> ~[u8] {
        encoder.encode(self)
    }

    fn decode(decoder: T) -> ~[u8] {
        decoder.decode(self)
    }

    fn convert(to: T, from: T) -> ~[u8] {
        binary::convert(self, to, from)
    }
}

pub impl<T: Copy binary::Codec> &str : BinaryCodec<T> {
    fn encode(encoder: T) -> ~[u8] {
        str::byte_slice(self, |b| encoder.encode(b))
    }

    fn decode(decoder: T) -> ~[u8] {
        str::byte_slice(self, |b| decoder.decode(b))
    }

    fn convert(to: T, from: T) -> ~[u8] {
        str::byte_slice(self, |b| binary::convert(b, to, from))
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
