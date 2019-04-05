use crate::lines::Lines;

#[inline]
pub fn is_item(text: &str) -> Option<bool> {
    if text.is_empty() {
        return None;
    }

    let bytes = text.as_bytes();
    match bytes[0] {
        b'*' | b'-' | b'+' => {
            if text.len() > 1 && (bytes[1] == b' ' || bytes[1] == b'\n') {
                Some(false)
            } else {
                None
            }
        }
        b'0'...b'9' => {
            let i = bytes
                .iter()
                .position(|&c| !c.is_ascii_digit())
                .unwrap_or_else(|| text.len() - 1);
            if (bytes[i] == b'.' || bytes[i] == b')')
                && i + 1 < text.len()
                && (bytes[i + 1] == b' ' || bytes[i + 1] == b'\n')
            {
                Some(true)
            } else {
                None
            }
        }
        _ => None,
    }
}

// returns (bullets, contents begin, contents end, end, has more)
#[inline]
pub fn parse(src: &str, ident: usize) -> (&str, usize, usize, usize, bool) {
    debug_assert!(
        is_item(&src[ident..]).is_some(),
        "{:?} is not a list item",
        src
    );
    debug_assert!(
        src[..ident].chars().all(|c| c == ' ' || c == '\t'),
        "{:?} doesn't starts with indentation {}",
        src,
        ident
    );

    let mut lines = Lines::new(src);
    let (mut pre_limit, mut pre_end, first_line) = lines.next().unwrap();
    let begin = match memchr::memchr(b' ', &first_line.as_bytes()[ident..]) {
        Some(i) => i + ident + 1,
        None => {
            let len = first_line.len();
            return (
                first_line,
                len,
                len,
                len,
                is_item(lines.next().unwrap().2).is_some(),
            );
        }
    };
    let bullet = &src[0..begin];

    while let Some((mut limit, mut end, mut line)) = lines.next() {
        // this line is emtpy
        if line.is_empty() {
            if let Some((next_limit, next_end, next_line)) = lines.next() {
                // next line is emtpy, too
                if next_line.is_empty() {
                    return (bullet, begin, pre_limit, next_end, false);
                } else {
                    // move to next line
                    pre_end = end;
                    limit = next_limit;
                    end = next_end;
                    line = next_line;
                }
            } else {
                return (bullet, begin, pre_limit, end, false);
            }
        }

        let line_ident = count_ident(line);

        if line_ident < ident {
            return (bullet, begin, pre_limit, pre_end, false);
        } else if line_ident == ident {
            return (
                bullet,
                begin,
                pre_limit,
                pre_end,
                is_item(&line[ident..]).is_some(),
            );
        }

        pre_end = end;
        pre_limit = limit;
    }

    (bullet, begin, src.len(), src.len(), false)
}

#[inline]
fn count_ident(src: &str) -> usize {
    src.as_bytes()
        .iter()
        .position(|&c| c != b' ' && c != b'\t')
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    #[test]
    fn is_item() {
        use super::is_item;

        assert_eq!(is_item("+ item"), Some(false));
        assert_eq!(is_item("- item"), Some(false));
        assert_eq!(is_item("10. item"), Some(true));
        assert_eq!(is_item("10) item"), Some(true));
        assert_eq!(is_item("1. item"), Some(true));
        assert_eq!(is_item("1) item"), Some(true));
        assert_eq!(is_item("10. "), Some(true));
        assert_eq!(is_item("10.\n"), Some(true));
        assert_eq!(is_item("10."), None);
        assert_eq!(is_item("+"), None);
        assert_eq!(is_item("-item"), None);
        assert_eq!(is_item("+item"), None);
    }

    #[test]
    fn parse() {
        use super::parse;

        assert_eq!(parse("+ item1\n+ item2\n+ item3", 0), ("+ ", 2, 7, 8, true));
        assert_eq!(
            parse("* item1\n\n* item2\n* item3", 0),
            ("* ", 2, 7, 9, true)
        );
        assert_eq!(
            parse("- item1\n\n\n- item2\n- item3", 0),
            ("- ", 2, 7, 10, false)
        );
        assert_eq!(
            parse("1. item1\n\n\n\n2. item2\n3. item3", 0),
            ("1. ", 3, 8, 11, false)
        );
        assert_eq!(
            parse("  + item1\n    + item2\n+ item3", 2),
            ("  + ", 4, 21, 22, false)
        );
        assert_eq!(
            parse("  + item1\n  + item2\n  + item3", 2),
            ("  + ", 4, 9, 10, true)
        );
        assert_eq!(parse("+\n", 0), ("+", 1, 1, 1, false));
        assert_eq!(parse("+\n+ item2\n+ item3", 0), ("+", 1, 1, 1, true));
        assert_eq!(parse("1) item1", 0), ("1) ", 3, 8, 8, false));
        assert_eq!(parse("1) item1\n", 0), ("1) ", 3, 8, 9, false));
    }
}
