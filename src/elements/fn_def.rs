#[cfg_attr(test, derive(PartialEq))]
#[derive(Debug)]
pub struct FnDef;

#[inline]
fn valid_label(ch: u8) -> bool {
    ch.is_ascii_alphanumeric() || ch == b'-' || ch == b'_'
}

impl FnDef {
    pub fn parse(src: &str) -> Option<(&str, &str, usize)> {
        starts_with!(src, "[fn:");

        let label = until_while!(src, 4, b']', valid_label)?;

        if label == 4 {
            return None;
        }

        let end = eol!(src);

        Some((&src[4..label], &src[label + 1..end], end))
    }
}

#[test]
fn parse() {
    assert_eq!(
        FnDef::parse("[fn:1] https://orgmode.org").unwrap(),
        (
            "1",
            " https://orgmode.org",
            "[fn:1] https://orgmode.org".len()
        )
    );
    assert_eq!(
        FnDef::parse("[fn:word_1] https://orgmode.org").unwrap(),
        (
            "word_1",
            " https://orgmode.org",
            "[fn:word_1] https://orgmode.org".len()
        )
    );
    assert_eq!(
        FnDef::parse("[fn:WORD-1] https://orgmode.org").unwrap(),
        (
            "WORD-1",
            " https://orgmode.org",
            "[fn:WORD-1] https://orgmode.org".len()
        )
    );
    assert_eq!(
        FnDef::parse("[fn:WORD]").unwrap(),
        ("WORD", "", "[fn:WORD]".len())
    );
    assert!(FnDef::parse("[fn:] https://orgmode.org").is_none());
    assert!(FnDef::parse("[fn:wor d] https://orgmode.org").is_none());
    assert!(FnDef::parse("[fn:WORD https://orgmode.org").is_none());
}
