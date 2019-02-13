use jetscii::Substring;

#[inline]
pub fn parse(src: &str) -> Option<(&str, usize)> {
    debug_assert!(src.starts_with("<<"));

    expect!(src, 2, |c| c != b' ')?;

    let end = Substring::new(">>").find(src).filter(|&i| {
        src.as_bytes()[2..i]
            .iter()
            .all(|&c| c != b'<' && c != b'\n' && c != b'>')
    })?;

    if src.as_bytes()[end - 1] == b' ' {
        return None;
    }

    Some((&src[2..end], end + 2))
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
