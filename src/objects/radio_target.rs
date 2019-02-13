use jetscii::Substring;

// TODO: text-markup, entities, latex-fragments, subscript and superscript
#[inline]
pub fn parse(src: &str) -> Option<(&str, usize)> {
    debug_assert!(src.starts_with("<<<"));

    expect!(src, 3, |c| c != b' ')?;

    let bytes = src.as_bytes();
    let end = Substring::new(">>>").find(src).filter(|&i| {
        bytes[3..i]
            .iter()
            .all(|&c| c != b'<' && c != b'\n' && c != b'>')
    })?;

    if bytes[end - 1] == b' ' {
        return None;
    }

    Some((&src[3..end], end + 3))
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
