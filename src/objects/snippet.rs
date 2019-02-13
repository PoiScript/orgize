use jetscii::Substring;
use memchr::memchr;

/// returns (snippet name, snippet value, offset)
#[inline]
pub fn parse(src: &str) -> Option<(&str, &str, usize)> {
    debug_assert!(src.starts_with("@@"));

    let name = memchr(b':', src.as_bytes()).filter(|&i| {
        i != 2
            && src.as_bytes()[2..i]
                .iter()
                .all(|&c| c.is_ascii_alphanumeric() || c == b'-')
    })?;

    let end = Substring::new("@@")
        .find(&src[name + 1..])
        .map(|i| i + name + 1)?;

    Some((&src[2..name], &src[name + 1..end], end + 2))
}

#[cfg(test)]
mod tests {
    #[test]
    fn parse() {
        use super::parse;

        assert_eq!(
            parse("@@html:<b>@@"),
            Some(("html", "<b>", "@@html:<b>@@".len()))
        );
        assert_eq!(
            parse("@@latex:any arbitrary LaTeX code@@"),
            Some((
                "latex",
                "any arbitrary LaTeX code",
                "@@latex:any arbitrary LaTeX code@@".len()
            ))
        );
        assert_eq!(parse("@@html:@@"), Some(("html", "", "@@html:@@".len())));
        assert_eq!(parse("@@html:<b>@"), None);
        assert_eq!(parse("@@html<b>@@"), None);
        assert_eq!(parse("@@:<b>@@"), None);
    }
}
