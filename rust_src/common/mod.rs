#![allow(non_upper_case_globals)]

#[no_mangle]
pub static kApplication: &[u8] = b"Input Leap";

#[no_mangle]
pub static kCopyright: &[u8] = b"
    Copyright (C) 2018 The InputLeap Developers\n
    Copyright (C) 2012-2016 Symless Ltd\n
    Copyright (C) 2008-2014 Nick Bolton\n
    Copyright (C) 2002-2014 Chris Schoeneman\0";

#[no_mangle]
pub static kContact: &[u8] = b"Email: todo@mail.com";

#[no_mangle]
pub static kWebsite: &[u8] = b"https://github.com/input-leap/input-leap";

#[no_mangle]
pub static kVersion: &[u8] = env!("CARGO_PKG_VERSION").as_bytes();

#[no_mangle]
pub static kAppVersion: &[u8] = kVersion;
