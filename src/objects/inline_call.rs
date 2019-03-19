use memchr::{memchr, memchr2};

// returns (name, args, inside_header, end_header)
#[inline]
pub fn parse(text: &str) -> Option<(&str, &str, Option<&str>, Option<&str>, usize)> {
    debug_assert!(text.starts_with("call_"));

    let bytes = text.as_bytes();

    let (name, off) = memchr2(b'[', b'(', bytes)
        .map(|i| (&text[5..i], i))
        .filter(|(name, _)| name.as_bytes().iter().all(u8::is_ascii_graphic))?;

    let (inside_header, off) = if bytes[off] == b'[' {
        memchr(b']', &bytes[off..])
            .filter(|&i| {
                bytes[off + i + 1] == b'(' && bytes[off + 1..off + i].iter().all(|&c| c != b'\n')
            })
            .map(|i| (Some(&text[off + 1..off + i]), off + i + 1))?
    } else {
        (None, off)
    };

    let (args, off) = memchr(b')', &bytes[off..])
        .map(|i| (&text[off + 1..off + i], off + i + 1))
        .filter(|(args, _)| args.as_bytes().iter().all(|&c| c != b'\n'))?;

    let (end_header, off) = if text.len() > off && text.as_bytes()[off] == b'[' {
        memchr(b']', &bytes[off..])
            .filter(|&i| bytes[off + 1..off + i].iter().all(|&c| c != b'\n'))
            .map(|i| (Some(&text[off + 1..off + i]), off + i + 1))?
    } else {
        (None, off)
    };

    Some((name, args, inside_header, end_header, off))
}

#[cfg(test)]
mod tests {
    #[test]
    fn parse() {
        use super::parse;

        assert_eq!(
            parse("call_square(4)"),
            Some(("square", "4", None, None, "call_square(4)".len()))
        );
        assert_eq!(
            parse("call_square[:results output](4)"),
            Some((
                "square",
                "4",
                Some(":results output"),
                None,
                "call_square[:results output](4)".len()
            ))
        );
        assert_eq!(
            parse("call_square(4)[:results html]"),
            Some((
                "square",
                "4",
                None,
                Some(":results html"),
                "call_square(4)[:results html]".len()
            ))
        );
        assert_eq!(
            parse("call_square[:results output](4)[:results html]"),
            Some((
                "square",
                "4",
                Some(":results output"),
                Some(":results html"),
                "call_square[:results output](4)[:results html]".len()
            ))
        );
    }
}
