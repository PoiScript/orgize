use memchr::{memchr2, memchr2_iter};

/// returns (footnote reference label, footnote reference definition, offset)
#[inline]
pub fn parse(src: &str) -> Option<(Option<&str>, Option<&str>, usize)> {
    debug_assert!(src.starts_with("[fn:"));

    let bytes = src.as_bytes();
    let label = memchr2(b']', b':', &bytes[4..])
        .map(|i| i + 4)
        .filter(|&i| {
            bytes[4..i]
                .iter()
                .all(|&c| c.is_ascii_alphanumeric() || c == b'-' || c == b'_')
        })?;

    if bytes[label] == b':' {
        let mut pairs = 1;
        let def = memchr2_iter(b'[', b']', &bytes[label..])
            .map(|i| i + label)
            .find(|&i| {
                if bytes[i] == b'[' {
                    pairs += 1;
                } else {
                    pairs -= 1;
                }
                pairs == 0
            })?;

        Some((
            if label == 4 {
                None
            } else {
                Some(&src[4..label])
            },
            Some(&src[label + 1..def]),
            def + 1,
        ))
    } else {
        Some((
            if label == 4 {
                None
            } else {
                Some(&src[4..label])
            },
            None,
            label + 1,
        ))
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn parse() {
        use super::parse;

        assert_eq!(parse("[fn:1]").unwrap(), (Some("1"), None, "[fn:1]".len()));
        assert_eq!(
            parse("[fn:1:2]").unwrap(),
            (Some("1"), Some("2"), "[fn:1:2]".len())
        );
        assert_eq!(
            parse("[fn::2]").unwrap(),
            (None, Some("2"), "[fn::2]".len())
        );
        assert_eq!(
            parse("[fn::[]]").unwrap(),
            (None, Some("[]"), "[fn::[]]".len())
        );
        assert!(parse("[fn::[]").is_none());
    }
}
