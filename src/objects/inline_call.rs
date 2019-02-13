use memchr::{memchr, memchr2};

/// returns (name, args, inside_header, end_header)
#[inline]
pub fn parse(src: &str) -> Option<(&str, &str, Option<&str>, Option<&str>, usize)> {
    debug_assert!(src.starts_with("call_"));

    // TODO: refactor
    let bytes = src.as_bytes();
    let mut pos =
        memchr2(b'[', b'(', bytes).filter(|&i| bytes[5..i].iter().all(|c| c.is_ascii_graphic()))?;
    let mut pos_;

    let name = &src[5..pos];

    let inside_header = if bytes[pos] == b'[' {
        pos_ = pos;
        pos = memchr(b']', &bytes[pos..])
            .map(|i| i + pos)
            .filter(|&i| bytes[pos..i].iter().all(|&c| c != b'\n'))?
            + 1;
        expect!(src, pos, b'(')?;
        Some(&src[pos_ + 1..pos - 1])
    } else {
        None
    };

    pos_ = pos;
    pos = memchr(b')', &bytes[pos..])
        .map(|i| i + pos)
        .filter(|&i| bytes[pos..i].iter().all(|&c| c != b'\n'))?;
    let args = &src[pos_ + 1..pos];

    let end_header = if src.len() > pos + 1 && src.as_bytes()[pos + 1] == b'[' {
        pos_ = pos;
        pos = memchr(b']', &bytes[pos_ + 1..])
            .map(|i| i + pos_ + 1)
            .filter(|&i| bytes[pos_ + 1..i].iter().all(|&c| c != b'\n' && c != b')'))?;
        Some(&src[pos_ + 2..pos])
    } else {
        None
    };

    Some((name, args, inside_header, end_header, pos + 1))
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
