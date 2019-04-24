use jetscii::Substring;

// TODO: text-markup, entities, latex-fragments, subscript and superscript
#[inline]
pub fn parse(src: &str) -> Option<(&str, usize)> {
    debug_assert!(src.starts_with("<<<"));

    let bytes = src.as_bytes();
    let (target, off) = Substring::new(">>>")
        .find(src)
        .filter(|&i| {
            bytes[3] != b' '
                && bytes[i - 1] != b' '
                && bytes[3..i]
                    .iter()
                    .all(|&c| c != b'<' && c != b'\n' && c != b'>')
        })
        .map(|i| (&src[3..i], i + ">>>".len()))?;

    Some((target, off))
}

#[cfg(test)]
mod tests {
    #[test]
    fn parse() {
        use super::parse;

        assert_eq!(
            parse("<<<target>>>"),
            Some(("target", "<<<target>>>".len()))
        );
        assert_eq!(
            parse("<<<tar get>>>"),
            Some(("tar get", "<<<tar get>>>".len()))
        );
        assert_eq!(parse("<<<target >>>"), None);
        assert_eq!(parse("<<< target>>>"), None);
        assert_eq!(parse("<<<ta<get>>>"), None);
        assert_eq!(parse("<<<ta>get>>>"), None);
        assert_eq!(parse("<<<ta\nget>>>"), None);
        assert_eq!(parse("<<<target>>"), None);
    }
}
