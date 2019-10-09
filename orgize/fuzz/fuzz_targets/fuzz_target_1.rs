#![no_main]

#[macro_use]
extern crate libfuzzer_sys;
extern crate orgize;

use orgize::Org;

#[cfg_attr(rustfmt, rustfmt_skip)]
libfuzzer_sys::fuzz_target!(|data: &[u8]| {
    if let Ok(s) = std::str::from_utf8(data) {
        let _ = Org::parse(s);
    }
});
