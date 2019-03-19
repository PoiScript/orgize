use memchr::{memchr2, memchr2_iter};

/// returns (footnote reference label, footnote reference definition, offset)
#[inline]
pub fn parse(text: &str) -> Option<(Option<&str>, Option<&str>, usize)> {
    debug_assert!(text.starts_with("[fn:"));

    let bytes = text.as_bytes();
    let (label, off) = memchr2(b']', b':', &bytes[4..])
        .filter(|&i| {
            bytes[4..i + 4]
                .iter()
                .all(|&c| c.is_ascii_alphanumeric() || c == b'-' || c == b'_')
        })
        .map(|i| (if i == 0 { None } else { Some(&text[4..i + 4]) }, i + 4))?;

    let (def, off) = if bytes[off] == b':' {
        let mut pairs = 1;
        memchr2_iter(b'[', b']', &bytes[off..])
            .find(|&i| {
                if bytes[i + off] == b'[' {
                    pairs += 1;
                } else {
                    pairs -= 1;
                }
                pairs == 0
            })
            .map(|i| (Some(&text[off + 1..off + i]), i + off + 1))?
    } else {
        (None, off + 1)
    };

    Some((label, def, off))
}

#[cfg(test)]
mod tests {
    #[test]
    fn parse() {
        use super::parse;

        assert_eq!(parse("[fn:1]"), Some((Some("1"), None, "[fn:1]".len())));
        assert_eq!(
            parse("[fn:1:2]"),
            Some((Some("1"), Some("2"), "[fn:1:2]".len()))
        );
        assert_eq!(parse("[fn::2]"), Some((None, Some("2"), "[fn::2]".len())));
        assert_eq!(
            parse("[fn::[]]"),
            Some((None, Some("[]"), "[fn::[]]".len()))
        );
        assert_eq!(parse("[fn::[]"), None);
    }
}
