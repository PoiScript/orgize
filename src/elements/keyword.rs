use memchr::{memchr, memchr2};

#[cfg_attr(test, derive(PartialEq))]
#[derive(Debug)]
pub struct Keyword<'a> {
    pub key: &'a str,
    pub option: Option<&'a str>,
    pub value: &'a str,
}

impl Keyword<'_> {
    #[inline]
    // return (key, option, value, offset)
    pub(crate) fn parse(text: &str) -> Option<(&str, Option<&str>, &str, usize)> {
        debug_assert!(text.starts_with("#+"));

        let bytes = text.as_bytes();

        let (key, off) = memchr2(b':', b'[', bytes)
            .filter(|&i| {
                bytes[2..i]
                    .iter()
                    .all(|&c| c.is_ascii_alphabetic() || c == b'_')
            })
            .map(|i| (&text[2..i], i + 1))?;

        let (option, off) = if bytes[off - 1] == b'[' {
            memchr(b']', bytes)
                .filter(|&i| {
                    bytes[off..i].iter().all(|&c| c != b'\n')
                        && i < text.len()
                        && bytes[i + 1] == b':'
                })
                .map(|i| (Some(&text[off..i]), i + "]:".len()))?
        } else {
            (None, off)
        };

        let end = memchr(b'\n', bytes)
            .map(|i| i + 1)
            .unwrap_or_else(|| text.len());

        Some((key, option, &text[off..end].trim(), end))
    }
}

#[test]
fn parse() {
    assert_eq!(
        Keyword::parse("#+KEY:"),
        Some(("KEY", None, "", "#+KEY:".len()))
    );
    assert_eq!(
        Keyword::parse("#+KEY: VALUE"),
        Some(("KEY", None, "VALUE", "#+KEY: VALUE".len()))
    );
    assert_eq!(
        Keyword::parse("#+K_E_Y: VALUE"),
        Some(("K_E_Y", None, "VALUE", "#+K_E_Y: VALUE".len()))
    );
    assert_eq!(
        Keyword::parse("#+KEY:VALUE\n"),
        Some(("KEY", None, "VALUE", "#+KEY:VALUE\n".len()))
    );
    assert_eq!(Keyword::parse("#+KE Y: VALUE"), None);
    assert_eq!(Keyword::parse("#+ KEY: VALUE"), None);

    assert_eq!(
        Keyword::parse("#+RESULTS:"),
        Some(("RESULTS", None, "", "#+RESULTS:".len()))
    );

    assert_eq!(
        Keyword::parse("#+ATTR_LATEX: :width 5cm"),
        Some((
            "ATTR_LATEX",
            None,
            ":width 5cm",
            "#+ATTR_LATEX: :width 5cm".len()
        ))
    );

    assert_eq!(
        Keyword::parse("#+CALL: double(n=4)"),
        Some(("CALL", None, "double(n=4)", "#+CALL: double(n=4)".len()))
    );

    assert_eq!(
        Keyword::parse("#+CAPTION[Short caption]: Longer caption."),
        Some((
            "CAPTION",
            Some("Short caption"),
            "Longer caption.",
            "#+CAPTION[Short caption]: Longer caption.".len()
        ))
    );
}
