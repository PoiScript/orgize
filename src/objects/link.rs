use jetscii::Substring;
use memchr::memchr;

/// returns (link path, link description, offset)
#[inline]
pub fn parse(text: &str) -> Option<(&str, Option<&str>, usize)> {
    debug_assert!(text.starts_with("[["));

    let (path, off) = memchr(b']', text.as_bytes())
        .map(|i| (&text[2..i], i))
        .filter(|(path, _)| {
            path.as_bytes()
                .iter()
                .all(|&c| c != b'<' && c != b'>' && c != b'\n')
        })?;

    if *text.as_bytes().get(off + 1)? == b']' {
        Some((path, None, off + 2))
    } else if text.as_bytes()[off + 1] == b'[' {
        let (desc, off) = Substring::new("]]")
            .find(&text[off + 1..])
            .map(|i| (&text[off + 2..off + i + 1], off + i + 3))
            .filter(|(desc, _)| desc.as_bytes().iter().all(|&c| c != b'[' && c != b']'))?;
        Some((path, Some(desc), off))
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn parse() {
        use super::parse;

        assert_eq!(parse("[[#id]]"), Some(("#id", None, "[[#id]]".len())));
        assert_eq!(
            parse("[[#id][desc]]"),
            Some(("#id", Some("desc"), "[[#id][desc]]".len()))
        );
        assert_eq!(parse("[[#id][desc]"), None);
    }
}
