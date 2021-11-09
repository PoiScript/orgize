#![no_main]

use libfuzzer_sys::fuzz_target;
use orgize::syntax::{HtmlHandler, Org};
use std::str;

fuzz_target!(|data: &[u8]| {
    if let Ok(utf8) = str::from_utf8(data) {
        let _ = Org::parse(utf8);
    }
});
