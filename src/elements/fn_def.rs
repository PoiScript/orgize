use memchr::memchr;

#[inline]
pub fn parse(src: &str) -> Option<(&str, &str, usize)> {
    debug_assert!(src.starts_with("[fn:"));

    let label = memchr(b']', src.as_bytes()).filter(|&i| {
        i != 4
            && src.as_bytes()[4..i]
                .iter()
                .all(|&c| c.is_ascii_alphanumeric() || c == b'-' || c == b'_')
    })?;

    let end = memchr(b'\n', src.as_bytes()).unwrap_or_else(|| src.len());

    Some((&src[4..label], &src[label + 1..end], end))
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
