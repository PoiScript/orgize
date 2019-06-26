use memchr::memchr_iter;
use std::iter::once;

#[cfg_attr(test, derive(PartialEq))]
#[derive(Debug)]
pub struct List {
    pub indent: usize,
    pub ordered: bool,
}

impl List {
    #[inline]
    // return (list, begin, end)
    pub(crate) fn parse(text: &str) -> Option<(List, usize, usize)> {
        let (indent, tail) = text
            .find(|c| c != ' ')
            .map(|off| (off, &text[off..]))
            .unwrap_or((0, text));

        let ordered = is_item(tail)?;
        let bytes = text.as_bytes();
        let mut lines = memchr_iter(b'\n', bytes)
            .map(|i| i + 1)
            .chain(once(text.len()));
        let mut pos = lines.next()?;

        while let Some(i) = lines.next() {
            let line = &text[pos..i];
            return if let Some(line_indent) = line.find(|c: char| !c.is_whitespace()) {
                // this line is no empty
                if line_indent < indent
                    || (line_indent == indent && is_item(&line[line_indent..]).is_none())
                {
                    Some((List { indent, ordered }, pos, pos))
                } else {
                    pos = i;
                    continue;
                }
            } else if let Some(next_i) = lines.next() {
                // this line is empty
                let line = &text[i..next_i];
                if let Some(line_indent) = line.find(|c: char| !c.is_whitespace()) {
                    if line_indent < indent
                        || (line_indent == indent && is_item(&line[line_indent..]).is_none())
                    {
                        Some((List { indent, ordered }, pos, pos))
                    } else {
                        pos = next_i;
                        continue;
                    }
                } else {
                    Some((List { indent, ordered }, pos, next_i))
                }
            } else {
                Some((List { indent, ordered }, pos, i))
            };
        }

        Some((List { indent, ordered }, pos, pos))
    }
}

#[cfg_attr(test, derive(PartialEq))]
#[derive(Debug)]
pub struct ListItem<'a> {
    pub bullet: &'a str,
}

impl ListItem<'_> {
    pub fn parse(text: &str, indent: usize) -> (ListItem<'_>, usize, usize) {
        debug_assert!(&text[0..indent].trim().is_empty());
        let off = &text[indent..].find(' ').unwrap() + 1 + indent;

        let bytes = text.as_bytes();
        let mut lines = memchr_iter(b'\n', bytes)
            .map(|i| i + 1)
            .chain(once(text.len()));
        let mut pos = lines.next().unwrap();

        for i in lines {
            let line = &text[pos..i];
            if let Some(line_indent) = line.find(|c: char| !c.is_whitespace()) {
                if line_indent == indent {
                    return (
                        ListItem {
                            bullet: &text[indent..off],
                        },
                        off,
                        pos,
                    );
                }
            }
            pos = i;
        }

        (
            ListItem {
                bullet: &text[indent..off],
            },
            off,
            text.len(),
        )
    }
}

#[inline]
pub fn is_item(text: &str) -> Option<bool> {
    let bytes = text.as_bytes();
    match bytes.get(0)? {
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
                && text.len() > i + 1
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

#[test]
fn test_is_item() {
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
fn list_parse() {
    assert_eq!(
        List::parse("+ item1\n+ item2"),
        Some((
            List {
                indent: 0,
                ordered: false,
            },
            "+ item1\n+ item2".len(),
            "+ item1\n+ item2".len()
        ))
    );
    assert_eq!(
        List::parse("* item1\n  \n* item2"),
        Some((
            List {
                indent: 0,
                ordered: false
            },
            "* item1\n  \n* item2".len(),
            "* item1\n  \n* item2".len()
        ))
    );
    assert_eq!(
        List::parse("* item1\n  \n   \n* item2"),
        Some((
            List {
                indent: 0,
                ordered: false,
            },
            "* item1\n".len(),
            "* item1\n  \n   \n".len()
        ))
    );
    assert_eq!(
        List::parse("* item1\n  \n   "),
        Some((
            List {
                indent: 0,
                ordered: false,
            },
            "+ item1\n".len(),
            "* item1\n  \n   ".len()
        ))
    );
    assert_eq!(
        List::parse("+ item1\n  + item2\n   "),
        Some((
            List {
                indent: 0,
                ordered: false,
            },
            "+ item1\n  + item2\n".len(),
            "+ item1\n  + item2\n   ".len()
        ))
    );
    assert_eq!(
        List::parse("+ item1\n  \n  + item2\n   \n+ item 3"),
        Some((
            List {
                indent: 0,
                ordered: false,
            },
            "+ item1\n  \n  + item2\n   \n+ item 3".len(),
            "+ item1\n  \n  + item2\n   \n+ item 3".len()
        ))
    );
    assert_eq!(
        List::parse("  + item1\n  \n  + item2"),
        Some((
            List {
                indent: 2,
                ordered: false,
            },
            "  + item1\n  \n  + item2".len(),
            "  + item1\n  \n  + item2".len()
        ))
    );
    assert_eq!(
        List::parse("+ 1\n\n  - 2\n\n  - 3\n\n+ 4"),
        Some((
            List {
                indent: 0,
                ordered: false,
            },
            "+ 1\n\n  - 2\n\n  - 3\n\n+ 4".len(),
            "+ 1\n\n  - 2\n\n  - 3\n\n+ 4".len()
        ))
    );
}
