// encoding.rs

// import baseNN's items with aliased names
import BaseNNCodec    = baseNN::Codec;
import BaseNNEncode   = baseNN::Encode;
import BaseNNDecode   = baseNN::Decode;
import Base16         = baseNN::Base16;
import Base32         = baseNN::Base32;
import Base64         = baseNN::Base64;
import Base32Hex      = baseNN::Base32Hex;
import Base64Urlsafe  = baseNN::Base64Urlsafe;

// export all aliased names
export BaseNNCodec;
export BaseNNEncode;
export BaseNNDecode;
export Base16;
export Base32;
export Base64;
export Base32Hex;
export Base64Urlsafe;

// NB this line is required to resolve traits in baseNN with aliased names
export baseNN;
