use memchr::memchr;

/// returns (link path, link description, offset)
#[inline]
pub fn parse(src: &str) -> Option<(&str, Option<&str>, usize)> {
    debug_assert!(src.starts_with("[["));

    let bytes = src.as_bytes();
    let path = memchr(b']', bytes).filter(|&i| {
        bytes[2..i]
            .iter()
            .all(|&c| c != b'<' && c != b'>' && c != b'\n')
    })?;

    if *bytes.get(path + 1)? == b']' {
        Some((&src[2..path], None, path + 2))
    } else if bytes[path + 1] == b'[' {
        let desc = memchr(b']', &bytes[path + 2..])
            .map(|i| i + path + 2)
            .filter(|&i| bytes[path + 2..i].iter().all(|&c| c != b'['))?;
        expect!(src, desc + 1, b']')?;

        Some((&src[2..path], Some(&src[path + 2..desc]), desc + 2))
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn parse() {
        use super::parse;

        assert_eq!(parse("[[#id]]").unwrap(), ("#id", None, "[[#id]]".len()));
        assert_eq!(
            parse("[[#id][desc]]").unwrap(),
            ("#id", Some("desc"), "[[#id][desc]]".len())
        );
        assert!(parse("[[#id][desc]").is_none());
    }
}
