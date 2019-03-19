use jetscii::Substring;
use memchr::memchr;

/// returns (snippet name, snippet value, offset)
#[inline]
pub fn parse(text: &str) -> Option<(&str, &str, usize)> {
    debug_assert!(text.starts_with("@@"));

    let (name, off) = memchr(b':', text.as_bytes())
        .filter(|&i| {
            i != 2
                && text.as_bytes()[2..i]
                    .iter()
                    .all(|&c| c.is_ascii_alphanumeric() || c == b'-')
        })
        .map(|i| (&text[2..i], i + 1))?;

    let (value, off) = Substring::new("@@")
        .find(&text[off..])
        .map(|i| (&text[off..off + i], off + i + 2 /* @@ */))?;

    Some((name, value, off))
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
