use bytecount::count;
use memchr::memchr;

#[inline]
pub(crate) fn parse(text: &str, marker: u8) -> Option<(&str, &str)> {
    debug_assert!(text.len() >= 3);

    let bytes = text.as_bytes();

    if bytes[1].is_ascii_whitespace() {
        return None;
    }

    let end = memchr(marker, &bytes[1..]).filter(|&i| count(&bytes[1..=i], b'\n') < 2)?;

    if bytes[end].is_ascii_whitespace() {
        return None;
    }

    if let Some(&post) = bytes.get(end + 2) {
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
            Some((&text[end + 2..], &text[1..end + 1]))
        } else {
            None
        }
    } else {
        Some((&text[end + 2..], &text[1..end + 1]))
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn parse() {
        use super::parse;

        assert_eq!(parse("*bold*", b'*'), Some(("", "bold")));
        assert_eq!(parse("*bo\nld*", b'*'), Some(("", "bo\nld")));
        assert_eq!(parse("*bold*a", b'*'), None);
        assert_eq!(parse("*bold*", b'/'), None);
        assert_eq!(parse("*bold *", b'*'), None);
        assert_eq!(parse("* bold*", b'*'), None);
        assert_eq!(parse("*b\nol\nd*", b'*'), None);
    }
}
