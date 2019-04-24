use memchr::memchr;

#[inline]
pub fn parse(text: &str) -> Option<(&str, &str, usize)> {
    debug_assert!(text.starts_with("[fn:"));

    let (label, off) = memchr(b']', text.as_bytes())
        .filter(|&i| {
            i != 4
                && text.as_bytes()["[fn:".len()..i]
                    .iter()
                    .all(|&c| c.is_ascii_alphanumeric() || c == b'-' || c == b'_')
        })
        .map(|i| (&text["[fn:".len()..i], i + 1))?;

    let (content, off) = memchr(b'\n', text.as_bytes())
        .map(|i| (&text[off..i], i))
        .unwrap_or_else(|| (&text[off..], text.len()));

    Some((label, content, off))
}

#[cfg(test)]
mod tests {
    #[test]
    fn parse() {
        use super::parse;

        assert_eq!(
            parse("[fn:1] https://orgmode.org"),
            Some((
                "1",
                " https://orgmode.org",
                "[fn:1] https://orgmode.org".len()
            ))
        );
        assert_eq!(
            parse("[fn:word_1] https://orgmode.org"),
            Some((
                "word_1",
                " https://orgmode.org",
                "[fn:word_1] https://orgmode.org".len()
            ))
        );
        assert_eq!(
            parse("[fn:WORD-1] https://orgmode.org"),
            Some((
                "WORD-1",
                " https://orgmode.org",
                "[fn:WORD-1] https://orgmode.org".len()
            ))
        );
        assert_eq!(parse("[fn:WORD]"), Some(("WORD", "", "[fn:WORD]".len())));
        assert_eq!(parse("[fn:] https://orgmode.org"), None);
        assert_eq!(parse("[fn:wor d] https://orgmode.org"), None);
        assert_eq!(parse("[fn:WORD https://orgmode.org"), None);
    }
}
