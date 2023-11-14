#![no_main]

libfuzzer_sys::fuzz_target!(|data: &[u8]| {
    if let Ok(utf8) = std::str::from_utf8(data) {
        let _ = orgize::Org::parse(utf8);
    }
});
