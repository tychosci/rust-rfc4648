extern mod rfc4648;

use std::vec;

use rfc4648::base16;
use rfc4648::base32;
use rfc4648::base64;

fn t(source: ~[~str], expect: ~[~str], cb: &fn(&[u8]) -> ~[u8]) {
    let mut source_b = vec::with_capacity(source.len());
    let mut expect_b = vec::with_capacity(expect.len());

    for s in source.move_iter() { source_b.push(s.into_bytes()); }
    for e in expect.move_iter() { expect_b.push(e.into_bytes()); }

    let actual_b = source_b.map(|e| cb(*e));

    assert_eq!(expect_b, actual_b);
}

#[test]
fn test_base64_standard_encode() {
    let source = ~[~"", ~"f", ~"fo", ~"foo", ~"foob", ~"fooba", ~"foobar"];
    let expect = ~[~"", ~"Zg==", ~"Zm8=", ~"Zm9v", ~"Zm9vYg==", ~"Zm9vYmE=", ~"Zm9vYmFy"];

    do t(source, expect) |src| {
        base64::Standard.encode(src)
    }
}

#[test]
fn test_base64_urlsafe_encode() {
    let source = ~[~"", ~"f", ~"fo", ~"fo>", ~"foob", ~"fooba", ~"fo?ba?"];
    let expect = ~[~"", ~"Zg==", ~"Zm8=", ~"Zm8-", ~"Zm9vYg==", ~"Zm9vYmE=", ~"Zm8_YmE_"];

    do t(source, expect) |src| {
        base64::UrlSafe.encode(src)
    }
}

#[test]
fn test_base64_standard_decode() {
    let source = ~[~"", ~"Zg==", ~"Zm8=", ~"Zm8+", ~"Zm9vYg==", ~"Zm9vYmE=", ~"Zm8/YmE/"];
    let expect = ~[~"", ~"f", ~"fo", ~"fo>", ~"foob", ~"fooba", ~"fo?ba?"];

    do t(source, expect) |src| {
        base64::Standard.decode(src)
    }
}

#[test]
fn test_base64_urlsafe_decode() {
    let source = ~[~"", ~"Zg==", ~"Zm8=", ~"Zm8-", ~"Zm9vYg==", ~"Zm9vYmE=", ~"Zm8_YmE_"];
    let expect = ~[~"", ~"f", ~"fo", ~"fo>", ~"foob", ~"fooba", ~"fo?ba?"];

    do t(source, expect) |src| {
        base64::UrlSafe.decode(src)
    }
}

#[test]
fn test_base32_standard_encode() {
    let source = ~[~"", ~"f", ~"fo", ~"foo", ~"foob", ~"fooba", ~"foobar"];
    let expect = ~[~"", ~"MY======", ~"MZXQ====", ~"MZXW6===", ~"MZXW6YQ=",
                   ~"MZXW6YTB", ~"MZXW6YTBOI======"];

    do t(source, expect) |src| {
        base32::Standard.encode(src)
    }
}

#[test]
fn test_base32_hex_encode() {
    let source = ~[~"", ~"f", ~"fo", ~"foo", ~"foob", ~"fooba", ~"foobar"];
    let expect = ~[~"", ~"CO======", ~"CPNG====", ~"CPNMU===",
                   ~"CPNMUOG=", ~"CPNMUOJ1", ~"CPNMUOJ1E8======"];

    do t(source, expect) |src| {
        base32::Hex.encode(src)
    }
}

#[test]
fn test_base32_standard_decode() {
    let source = ~[~"", ~"MY======", ~"MZXQ====", ~"MZXW6===",
                   ~"MZXW6YQ=", ~"MZXW6YTB", ~"MZXW6YTBOI======"];
    let expect = ~[~"", ~"f", ~"fo", ~"foo", ~"foob", ~"fooba", ~"foobar"];

    do t(source, expect) |src| {
        base32::Standard.decode(src)
    }
}

#[test]
fn test_base32_hex_decode() {
    let source = ~[~"", ~"CO======", ~"CPNG====", ~"CPNMU===",
                   ~"CPNMUOG=", ~"CPNMUOJ1", ~"CPNMUOJ1E8======"];
    let expect = ~[~"", ~"f", ~"fo", ~"foo", ~"foob", ~"fooba", ~"foobar"];

    do t(source, expect) |src| {
        base32::Hex.decode(src)
    }
}

#[test]
fn test_base16_encode() {
    let source = bytes!("foo");
    let expect = bytes!("666F6F");

    let actual = base16::encode(source);

    assert_eq!(expect, actual.as_slice());
}

#[test]
fn test_base16_decode() {
    let source = bytes!("666f6f");
    let expect = bytes!("foo");

    let actual = base16::decode(source);

    assert_eq!(expect, actual.as_slice());
}
