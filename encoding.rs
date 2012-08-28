// encoding.rs

// import enum variants in baseNN.rs
import Base16         = baseNN::Base16;
import Base32         = baseNN::Base32;
import Base64         = baseNN::Base64;
import Base32Hex      = baseNN::Base32Hex;
import Base64Url      = baseNN::Base64Url;

// import constants in baseNN/base{16,32,64}.rs
import BASE16         = baseNN::base16::BASE16;
import BASE32         = baseNN::base32::BASE32_STD;
import BASE64         = baseNN::base64::BASE64_STD;
import BASE32_HEX     = baseNN::base32::BASE32_HEX;
import BASE64_URL     = baseNN::base64::BASE64_URL;

// import structs in baseNN/base{16,32,64}.rs
import Base16Writer   = baseNN::base16::Base16Writer;
import Base32Writer   = baseNN::base32::Base32Writer;
import Base64Writer   = baseNN::base64::Base64Writer;
import Base32Reader   = baseNN::base32::Base32Reader;
import Base64Reader   = baseNN::base64::Base64Reader;

//===-------------------------------------------------------------------===//
//                              b a s e N N
//===-------------------------------------------------------------------===//

// XXX this line is required to resolve traits in baseNN.
export baseNN;

// export all BaseNN enum variants
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

//===-------------------------------------------------------------------===//
//                               c o m m o n
//===-------------------------------------------------------------------===//

export Convert;
export Encode;
export Decode;
export Codec;

type Buffer = &[u8];
type String = &str;

type Convert<T: Encode Decode> = {from: T, to: T};

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
        return buf;
    }
}

impl<T: Encode Decode> Convert<T> : Decode {
    fn decode(buf: &[u8]) -> ~[u8] {
        let buf = self.from.decode(buf);
        let buf = self.to.encode(buf);
        return buf;
    }
}

impl<T: Encode Decode> Buffer : Codec<T> {
    fn encode(encoder: T) -> ~[u8] {
        encoder.encode(self)
    }
    fn decode(decoder: T) -> ~[u8] {
        decoder.decode(self)
    }
}

impl<T: Encode Decode> String : Codec<T> {
    fn encode(encoder: T) -> ~[u8] {
        encoder.encode(str::to_bytes(self))
    }
    fn decode(decoder: T) -> ~[u8] {
        decoder.decode(str::to_bytes(self))
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_codec_baseNN() {
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

        let actual = source.encode({
            from: Base32,
            to:   Base64,
        });

        assert expect == actual;
    }
}
