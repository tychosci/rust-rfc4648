// `end` indicates whether the Decode method have encountered paddings or not.
pub struct DecodeResult {
    end: bool,
    ndecoded: uint,
}

pub trait BinaryEncoder {
    fn encode(&self, dst: &mut [u8], src: &[u8]);
    fn encoded_len(&self, src_length: uint) -> uint;
    fn encode_bytes(&self, src: &[u8]) -> ~[u8];
}

pub trait BinaryDecoder {
    fn decode(&self, dst: &mut [u8], src: &[u8]) -> DecodeResult;
    fn decoded_len(&self, src_length: uint) -> uint;
    fn decode_bytes(&self, src: &[u8]) -> ~[u8];
}
