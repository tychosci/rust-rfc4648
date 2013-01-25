/*!
rfc4648 - RFC 4648: Base16, Base32, Base64 Data Encodings
*/
#[link(name = "rfc4648",
       vers = "0.1",
       uuid = "7F7C9B2E-84D6-484F-BBD5-96A2076BDC32",
       url  = "https://github.com/tychosci/rust-rfc4648")];

#[comment = "RFC 4648: Base16, Base32, Base64 Data Encodings"];
#[license = "MIT license"];
#[crate_type = "lib"];

extern mod std;

pub mod base16;
pub mod base32;
pub mod base64;
mod util;

pub trait ToBase16 {
    fn to_base16(&self) -> ~[u8];
}

pub trait ToBase32 {
    fn to_base32(&self) -> ~[u8];
    fn to_base32_hex(&self) -> ~[u8];
}

pub trait ToBase64 {
    fn to_base64(&self) -> ~[u8];
    fn to_base64_urlsafe(&self) -> ~[u8];
}

pub trait FromBase16 {
    fn from_base16(&self) -> ~[u8];
}

pub trait FromBase32 {
    fn from_base32(&self) -> ~[u8];
    fn from_base32_hex(&self) -> ~[u8];
}

pub trait FromBase64 {
    fn from_base64(&self) -> ~[u8];
    fn from_base64_urlsafe(&self) -> ~[u8];
}

pub trait Rfc4648:
    FromBase16 FromBase32 FromBase64
    ToBase16 ToBase32 ToBase64 {
}

pub impl &[const u8]: Rfc4648;
pub impl &str: Rfc4648;

macro_rules! mk_impl_for_bytes(
    ($trait_name:ident =>
    $( $method_name:ident -> $fn_name:path ;)+) => (
        pub impl &[const u8]: $trait_name {
            $(fn $method_name(&self) -> ~[u8] {
                $fn_name(*self)
            })+
        }
    )
)
macro_rules! mk_impl_for_str(
    ($trait_name:ident, [$($method_name:ident),+]) => (
        pub impl &str: $trait_name {
            $(fn $method_name(&self) -> ~[u8] {
                str::byte_slice(*self, |b| b.$method_name())
            })+
        }
    )
)

mk_impl_for_bytes! { ToBase16 =>
    to_base16 -> base16::encode;
}
mk_impl_for_bytes! { ToBase32 =>
    to_base32 -> base32::encode;
    to_base32_hex -> base32::hex_encode;
}
mk_impl_for_bytes! { ToBase64 =>
    to_base64 -> base64::encode;
    to_base64_urlsafe -> base64::urlsafe_encode;
}
mk_impl_for_bytes! { FromBase16 =>
    from_base16 -> base16::decode;
}
mk_impl_for_bytes! { FromBase32 =>
    from_base32 -> base32::decode;
    from_base32_hex -> base32::hex_decode;
}
mk_impl_for_bytes! { FromBase64 =>
    from_base64 -> base64::decode;
    from_base64_urlsafe -> base64::urlsafe_decode;
}

mk_impl_for_str!(ToBase16, [to_base16])
mk_impl_for_str!(ToBase32, [to_base32, to_base32_hex])
mk_impl_for_str!(ToBase64, [to_base64, to_base64_urlsafe])
mk_impl_for_str!(FromBase16, [from_base16])
mk_impl_for_str!(FromBase32, [from_base32, from_base32_hex])
mk_impl_for_str!(FromBase64, [from_base64, from_base64_urlsafe])
