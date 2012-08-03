enum encoding {
    base16,
    base32,
    base64,
    base32hex,
    base64urlsafe,
}

trait codec {
    fn encode(e: encoding) -> ~[u8];
    fn decode(e: encoding) -> ~[u8];
}

impl &[u8] : codec {
    fn encode(e: encoding) -> ~[u8] {
        alt e {
            base16        => { base16::encode(self) }
            base32        => { base32::encode(self) }
            base64        => { base64::encode(self) }
            base32hex     => { base32::hex_encode(self) }
            base64urlsafe => { base64::urlsafe_encode(self) }
        }
    }
    fn decode(e: encoding) -> ~[u8] {
        alt e {
            base16        => { base16::decode(self) }
            base32        => { base32::decode(self) }
            base64        => { base64::decode(self) }
            base32hex     => { base32::hex_decode(self) }
            base64urlsafe => { base64::urlsafe_decode(self) }
        }
    }
}

impl ~str : codec {
    fn encode(e: encoding) -> ~[u8] {
        let bytes = str::bytes(self);
        bytes.encode(e)
    }
    fn decode(e: encoding) -> ~[u8] {
        let bytes = str::bytes(self);
        bytes.decode(e)
    }
}
