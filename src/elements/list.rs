use memchr::memchr_iter;

#[inline]
pub fn is_item(text: &str) -> Option<(bool, &str)> {
    if text.is_empty() {
        return None;
    }

    let bytes = text.as_bytes();
    match bytes[0] {
        b'*' | b'-' | b'+' => {
            if text.len() > 1 && (bytes[1] == b' ' || bytes[1] == b'\n') {
                Some((false, &text[0..2]))
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
                Some((true, &text[0..i + 2]))
            } else {
                None
            }
        }
        _ => None,
    }
}

// check if list item ends at this line
#[inline]
fn is_item_ends(line: &str, ident: usize) -> Option<&str> {
    debug_assert!(!line.is_empty());

    let line_ident = line
        .as_bytes()
        .iter()
        .position(|&c| c != b' ' && c != b'\t')
        .unwrap_or(0);

    debug_assert!(line_ident >= ident, "{} >= {}", line_ident, ident);

    if line_ident == ident {
        is_item(&line[ident..]).map(|(_, bullet)| bullet)
    } else {
        None
    }
}

// return (limit, end, next item bullet)
#[inline]
pub fn parse(text: &str, ident: usize) -> (usize, usize, Option<&str>) {
    let bytes = text.as_bytes();
    let mut lines = memchr_iter(b'\n', bytes);
    let mut pos = if let Some(i) = lines.next() {
        i + 1
    } else {
        return (text.len(), text.len(), None);
    };

    while let Some(i) = lines.next() {
        return if bytes[pos..i].iter().all(u8::is_ascii_whitespace) {
            if let Some(nexti) = lines.next() {
                if bytes[i + 1..nexti].iter().all(u8::is_ascii_whitespace) {
                    // two consecutive empty lines
                    (pos - 1, nexti + 1, None)
                } else if let Some(next) = is_item_ends(&text[i + 1..nexti], ident) {
                    (pos - 1, i + 1, Some(next))
                } else {
                    pos = nexti + 1;
                    continue;
                }
            } else if bytes[i + 1..].iter().all(u8::is_ascii_whitespace) {
                // two consecutive empty lines
                (pos - 1, text.len(), None)
            } else if let Some(next) = is_item_ends(&text[i + 1..], ident) {
                (pos - 1, i + 1, Some(next))
            } else {
                (text.len(), text.len(), None)
            }
        } else if let Some(next) = is_item_ends(&text[pos..i], ident) {
            (pos - 1, pos, Some(next))
        } else {
            pos = i + 1;
            continue;
        };
    }

    if bytes[pos..].iter().all(u8::is_ascii_whitespace) {
        (pos - 1, text.len(), None)
    } else if let Some(next) = is_item_ends(&text[pos..], ident) {
        (pos - 1, pos, Some(next))
    } else {
        (text.len(), text.len(), None)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn is_item() {
        use super::is_item;

        assert_eq!(is_item("+ item"), Some((false, "+ ")));
        assert_eq!(is_item("- item"), Some((false, "- ")));
        assert_eq!(is_item("10. item"), Some((true, "10. ")));
        assert_eq!(is_item("10) item"), Some((true, "10) ")));
        assert_eq!(is_item("1. item"), Some((true, "1. ")));
        assert_eq!(is_item("1) item"), Some((true, "1) ")));
        assert_eq!(is_item("10. "), Some((true, "10. ")));
        assert_eq!(is_item("10.\n"), Some((true, "10.\n")));
        assert_eq!(is_item("10."), None);
        assert_eq!(is_item("+"), None);
        assert_eq!(is_item("-item"), None);
        assert_eq!(is_item("+item"), None);
    }

    #[test]
    fn parse() {
        use super::parse;

        assert_eq!(
            parse("item1\n+ item2", 0),
            ("item1".len(), "item1\n".len(), Some("+ "))
        );
        assert_eq!(
            parse("item1\n  \n* item2", 0),
            ("item1".len(), "item1\n  \n".len(), Some("* "))
        );
        assert_eq!(
            parse("item1\n  \n   \n* item2", 0),
            ("item1".len(), "item1\n  \n   \n".len(), None)
        );
        assert_eq!(
            parse("item1\n  \n   ", 0),
            ("item1".len(), "item1\n  \n   ".len(), None)
        );
        assert_eq!(
            parse("item1\n  + item2\n   ", 0),
            (
                "item1\n  + item2".len(),
                "item1\n  + item2\n   ".len(),
                None
            )
        );
        assert_eq!(
            parse("item1\n  \n  + item2\n   \n+ item 3", 0),
            (
                "item1\n  \n  + item2".len(),
                "item1\n  \n  + item2\n   \n".len(),
                Some("+ ")
            )
        );
        assert_eq!(
            parse("item1\n  \n  + item2", 2),
            ("item1".len(), "item1\n  \n".len(), Some("+ "))
        );
        assert_eq!(
            parse("1\n\n  - 2\n\n  - 3\n\n+ 4", 0),
            (
                "1\n\n  - 2\n\n  - 3".len(),
                "1\n\n  - 2\n\n  - 3\n\n".len(),
                Some("+ ")
            )
        );
    }
}
