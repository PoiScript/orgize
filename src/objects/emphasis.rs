use memchr::memchr;

#[inline]
/// returns offset
pub fn parse(src: &str, marker: u8) -> Option<usize> {
    debug_assert!(src.len() >= 3);

    let bytes = src.as_bytes();

    if bytes[1].is_ascii_whitespace() {
        return None;
    }

    let end = memchr(marker, &bytes[1..])
        .map(|i| i + 1)
        .filter(|&i| bytes[1..i].iter().filter(|&&c| c == b'\n').count() < 2)?;

    if bytes[end - 1].is_ascii_whitespace() {
        return None;
    }

    if end < src.len() - 1 {
        let post = bytes[end + 1];
        if post == b' '
            || post == b'-'
            || post == b'.'
            || post == b','
            || post == b':'
            || post == b'!'
            || post == b'?'
            || post == b'\''
            || post == b'\n'
            || post == b')'
            || post == b'}'
        {
            Some(end)
        } else {
            None
        }
    } else {
        Some(end)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn parse() {
        use super::parse;

        assert_eq!(parse("*bold*", b'*').unwrap(), "*bold".len());
        assert_eq!(parse("*bo\nld*", b'*').unwrap(), "*bo\nld".len());
        assert!(parse("*bold*a", b'*').is_none());
        assert!(parse("*bold*", b'/').is_none());
        assert!(parse("*bold *", b'*').is_none());
        assert!(parse("* bold*", b'*').is_none());
        assert!(parse("*b\nol\nd*", b'*').is_none());
    }
}
