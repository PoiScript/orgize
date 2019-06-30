use memchr::memchr;

#[cfg_attr(test, derive(PartialEq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Debug)]
pub struct FnDef<'a> {
    pub label: &'a str,
    #[cfg_attr(all(feature = "serde", not(feature = "extra-serde-info")), serde(skip))]
    pub contents: &'a str,
}

impl FnDef<'_> {
    #[inline]
    pub(crate) fn parse(text: &str) -> Option<(&str, FnDef<'_>)> {
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

            Some((
                &text[end..],
                FnDef {
                    label,
                    contents: &text[off..end],
                },
            ))
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
            "",
            FnDef {
                label: "1",
                contents: " https://orgmode.org"
            },
        ))
    );
    assert_eq!(
        FnDef::parse("[fn:word_1] https://orgmode.org"),
        Some((
            "",
            FnDef {
                label: "word_1",
                contents: " https://orgmode.org"
            },
        ))
    );
    assert_eq!(
        FnDef::parse("[fn:WORD-1] https://orgmode.org"),
        Some((
            "",
            FnDef {
                label: "WORD-1",
                contents: " https://orgmode.org"
            },
        ))
    );
    assert_eq!(
        FnDef::parse("[fn:WORD]"),
        Some((
            "",
            FnDef {
                label: "WORD",
                contents: ""
            },
        ))
    );
    assert_eq!(FnDef::parse("[fn:] https://orgmode.org"), None);
    assert_eq!(FnDef::parse("[fn:wor d] https://orgmode.org"), None);
    assert_eq!(FnDef::parse("[fn:WORD https://orgmode.org"), None);
}
