use bytecount::count;
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
        .filter(|&i| count(&bytes[1..i], b'\n') < 2)?;

    if bytes[end - 1].is_ascii_whitespace() {
        return None;
    }

    if let Some(&post) = bytes.get(end + 1) {
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

        assert_eq!(parse("*bold*", b'*'), Some("*bold".len()));
        assert_eq!(parse("*bo\nld*", b'*'), Some("*bo\nld".len()));
        assert_eq!(parse("*bold*a", b'*'), None);
        assert_eq!(parse("*bold*", b'/'), None);
        assert_eq!(parse("*bold *", b'*'), None);
        assert_eq!(parse("* bold*", b'*'), None);
        assert_eq!(parse("*b\nol\nd*", b'*'), None);
    }
}
