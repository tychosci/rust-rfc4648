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

// import constants in baseNN/base{16,32,64}.rs
import BASE16         = baseNN::base16::BASE16;
import BASE32         = baseNN::base32::BASE32_STD;
import BASE64         = baseNN::base64::BASE64_STD;
import BASE32_HEX     = baseNN::base32::BASE32_HEX;
import BASE64_URLSAFE = baseNN::base64::BASE64_URL;

// import structs in baseNN/base{16,32,64}.rs
import Base16Writer   = baseNN::base16::Base16Writer;
import Base32Writer   = baseNN::base32::Base32Writer;
import Base64Writer   = baseNN::base64::Base64Writer;
import Base32Reader   = baseNN::base32::Base32Reader;
import Base64Reader   = baseNN::base64::Base64Reader;

//===-------------------------------------------------------------------===//
//                              b a s e N N
//===-------------------------------------------------------------------===//

// NB this line is required to resolve traits in baseNN by aliased names
export baseNN;

// export all aliased names
export BaseNNCodec;
export BaseNNEncode;
export BaseNNDecode;
export Base16;
export Base32;
export Base64;
export Base32Hex;
export Base64Urlsafe;

// export constants
export BASE16;
export BASE32;
export BASE64;
export BASE32_HEX;
export BASE64_URLSAFE;

// export structs
export Base16Writer;
export Base32Writer;
export Base64Writer;
export Base32Reader;
export Base64Reader;

