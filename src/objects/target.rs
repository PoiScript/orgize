use jetscii::Substring;

#[inline]
pub fn parse(text: &str) -> Option<(&str, usize)> {
    debug_assert!(text.starts_with("<<"));

    let bytes = text.as_bytes();

    let (target, off) = Substring::new(">>")
        .find(text)
        .filter(|&i| {
            bytes[2] != b' '
                && bytes[i - 1] != b' '
                && bytes[2..i]
                    .iter()
                    .all(|&c| c != b'<' && c != b'\n' && c != b'>')
        })
        .map(|i| (&text[2..i], i + 2 /* >> */))?;

    Some((target, off))
}

#[cfg(test)]
mod tests {
    #[test]
    fn parse() {
        use super::parse;

        assert_eq!(parse("<<target>>"), Some(("target", "<<target>>".len())));
        assert_eq!(parse("<<tar get>>"), Some(("tar get", "<<tar get>>".len())));
        assert_eq!(parse("<<target >>"), None);
        assert_eq!(parse("<< target>>"), None);
        assert_eq!(parse("<<ta<get>>"), None);
        assert_eq!(parse("<<ta>get>>"), None);
        assert_eq!(parse("<<ta\nget>>"), None);
        assert_eq!(parse("<<target>"), None);
    }
}
