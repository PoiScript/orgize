use memchr::memchr;

#[cfg_attr(test, derive(PartialEq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Debug)]
pub struct FnDef<'a> {
    pub label: &'a str,
}

impl FnDef<'_> {
    #[inline]
    pub fn parse(text: &str) -> Option<(FnDef<'_>, usize, usize)> {
        if text.starts_with("[fn:") {
            let (label, off) = memchr(b']', text.as_bytes())
                .filter(|&i| {
                    i != 4
                        && text.as_bytes()["[fn:".len()..i]
                            .iter()
                            .all(|&c| c.is_ascii_alphanumeric() || c == b'-' || c == b'_')
                })
                .map(|i| (&text["[fn:".len()..i], i + 1))?;

            let end = memchr(b'\n', text.as_bytes()).unwrap_or_else(|| text.len());

            Some((FnDef { label }, off, end))
        } else {
            None
        }
    }
}

#[test]
fn parse() {
    assert_eq!(
        FnDef::parse("[fn:1] https://orgmode.org"),
        Some((
            FnDef { label: "1" },
            "[fn:1]".len(),
            "[fn:1] https://orgmode.org".len()
        ))
    );
    assert_eq!(
        FnDef::parse("[fn:word_1] https://orgmode.org"),
        Some((
            FnDef { label: "word_1" },
            "[fn:word_1]".len(),
            "[fn:word_1] https://orgmode.org".len()
        ))
    );
    assert_eq!(
        FnDef::parse("[fn:WORD-1] https://orgmode.org"),
        Some((
            FnDef { label: "WORD-1" },
            "[fn:WORD-1]".len(),
            "[fn:WORD-1] https://orgmode.org".len()
        ))
    );
    assert_eq!(
        FnDef::parse("[fn:WORD]"),
        Some((
            FnDef { label: "WORD" },
            "[fn:WORD]".len(),
            "[fn:WORD]".len()
        ))
    );
    assert_eq!(FnDef::parse("[fn:] https://orgmode.org"), None);
    assert_eq!(FnDef::parse("[fn:wor d] https://orgmode.org"), None);
    assert_eq!(FnDef::parse("[fn:WORD https://orgmode.org"), None);
}
