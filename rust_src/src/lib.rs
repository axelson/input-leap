//! Crate for the Input Leap's Rust core.
#![deny(
    warnings,
    missing_copy_implementations,
    missing_debug_implementations,
//    missing_docs,
    clippy::all,
    clippy::cargo,
    trivial_casts,
    trivial_numeric_casts,
    unused_import_braces,
    unused_qualifications,
    unused_extern_crates,
    variant_size_differences
)]
#![allow(non_upper_case_globals, non_snake_case)]

pub static kApplication: &'static str = "Input Leap";

#[no_mangle]
pub static kCopyright: &'static str = "
    Copyright (C) 2018 The InputLeap Developers\n
    Copyright (C) 2012-2016 Symless Ltd\n
    Copyright (C) 2008-2014 Nick Bolton\n
    Copyright (C) 2002-2014 Chris Schoeneman";

#[no_mangle]
pub static kContact: &'static str = "Email: todo@mail.com";

#[no_mangle]
pub static kWebsite: &'static str = "https://github.com/input-leap/input-leap";

#[no_mangle]
pub static kVersion: &'static str = env!("CARGO_PKG_VERSION");

#[no_mangle]
pub static kAppVersion: &'static str = kVersion;

#[cxx::bridge]
mod ffi {
    extern "Rust" {
        fn get_kApplication() -> &'static str;
        fn get_kCopyright() -> &'static str;
    }
}

fn get_kApplication() -> &'static str {
    kApplication
}

fn get_kCopyright() -> &'static str {
    kCopyright
}
