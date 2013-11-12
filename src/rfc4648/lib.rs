#[link(name = "rfc4648",
       vers = "0.2.0",
       uuid = "6dc641c7-3e00-4511-a9c7-04f8b7eafe17",
       url  = "http://github.com/tychosci/rust-rfc4648")];

#[comment = "RFC 4648: Base16, Base32, Base64 Data Encodings"];
#[license = "MIT license"];
#[crate_type = "lib"];

pub mod base16;
pub mod base32;
pub mod base64;
