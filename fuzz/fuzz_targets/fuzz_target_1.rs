#![no_main]
#[macro_use]
extern crate libfuzzer_sys;
extern crate orgize;

use orgize::Parser;

#[cfg_attr(rustfmt, rustfmt_skip)]
fuzz_target!(|data: &[u8]| {
    if let Ok(s) = std::str::from_utf8(data) {
        let _ = Parser::new(s).collect::<Vec<_>>();
    }
});
