#[cfg_attr(test, derive(PartialEq))]
#[derive(Debug)]
pub struct FnDef<'a> {
    pub label: &'a str,
    pub contents: &'a str,
}

#[inline]
fn valid_label(ch: u8) -> bool {
    ch.is_ascii_alphanumeric() || ch == b'-' || ch == b'_'
}

impl<'a> FnDef<'a> {
    pub fn parse(src: &'a str) -> Option<(FnDef<'a>, usize)> {
        starts_with!(src, "[fn:");

        let label = until_while!(src, 4, b']', valid_label);

        if label == 4 {
            return None;
        }

        let end = eol!(src);

        Some((
            FnDef {
                label: &src[4..label],
                contents: &src[label + 1..end],
            },
            end,
        ))
    }
}

#[test]
fn parse() {
    assert_eq!(
        FnDef::parse("[fn:1] https://orgmode.org").unwrap(),
        (
            FnDef {
                label: "1",
                contents: " https://orgmode.org",
            },
            "[fn:1] https://orgmode.org".len()
        )
    );
    assert_eq!(
        FnDef::parse("[fn:word_1] https://orgmode.org").unwrap(),
        (
            FnDef {
                label: "word_1",
                contents: " https://orgmode.org",
            },
            "[fn:word_1] https://orgmode.org".len()
        )
    );
    assert_eq!(
        FnDef::parse("[fn:WORD-1] https://orgmode.org").unwrap(),
        (
            FnDef {
                label: "WORD-1",
                contents: " https://orgmode.org",
            },
            "[fn:WORD-1] https://orgmode.org".len()
        )
    );
    assert_eq!(
        FnDef::parse("[fn:WORD]").unwrap(),
        (
            FnDef {
                label: "WORD",
                contents: "",
            },
            "[fn:WORD]".len()
        )
    );
    assert!(FnDef::parse("[fn:] https://orgmode.org").is_none());
    assert!(FnDef::parse("[fn:wor d] https://orgmode.org").is_none());
    assert!(FnDef::parse("[fn:WORD https://orgmode.org").is_none());
}
