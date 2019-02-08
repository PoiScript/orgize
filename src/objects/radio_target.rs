use jetscii::Substring;

// TODO: text-markup, entities, latex-fragments, subscript and superscript
#[inline]
pub fn parse(src: &str) -> Option<(&str, usize)> {
    debug_assert!(src.starts_with("<<<"));

    expect!(src, 3, |c| c != b' ')?;

    let end = Substring::new(">>>").find(src).filter(|&i| {
        src.as_bytes()[3..i]
            .iter()
            .all(|&c| c != b'<' && c != b'\n' && c != b'>')
    })?;

    if src.as_bytes()[end - 1] == b' ' {
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
            parse("<<<target>>>").unwrap(),
            ("target", "<<<target>>>".len())
        );
        assert_eq!(
            parse("<<<tar get>>>").unwrap(),
            ("tar get", "<<<tar get>>>".len())
        );
        assert_eq!(parse("<<<target >>>"), None);
        assert_eq!(parse("<<< target>>>"), None);
        assert_eq!(parse("<<<ta<get>>>"), None);
        assert_eq!(parse("<<<ta>get>>>"), None);
        assert_eq!(parse("<<<ta\nget>>>"), None);
        assert_eq!(parse("<<<target>>"), None);
    }
}
