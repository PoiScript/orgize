use std::borrow::Cow;
use std::iter::once;

use memchr::memchr_iter;

#[cfg_attr(test, derive(PartialEq))]
#[cfg_attr(feature = "ser", derive(serde::Serialize))]
#[derive(Debug)]
pub struct List {
    pub indent: usize,
    pub ordered: bool,
}

impl List {
    #[inline]
    pub(crate) fn parse(text: &str) -> Option<(&str, List, &str)> {
        let (indent, tail) = text
            .find(|c| c != ' ')
            .map(|off| (off, &text[off..]))
            .unwrap_or((0, text));

        let ordered = is_item(tail)?;

        let mut last_end = 0;
        let mut start = 0;

        for i in memchr_iter(b'\n', text.as_bytes())
            .map(|i| i + 1)
            .chain(once(text.len()))
        {
            let line = &text[start..i];
            if let Some(line_indent) = line.find(|c: char| !c.is_whitespace()) {
                if line_indent < indent
                    || (line_indent == indent && is_item(&line[line_indent..]).is_none())
                {
                    return Some((
                        &text[start..],
                        List { indent, ordered },
                        &text[0..start - 1],
                    ));
                } else {
                    last_end = 0;
                    start = i;
                    continue;
                }
            } else {
                // this line is empty
                if last_end != 0 {
                    return Some((&text[i..], List { indent, ordered }, &text[0..last_end]));
                } else {
                    last_end = start;
                    start = i;
                    continue;
                }
            }
        }

        if last_end != 0 {
            Some(("", List { indent, ordered }, &text[0..last_end]))
        } else {
            Some(("", List { indent, ordered }, text))
        }
    }
}

#[cfg_attr(test, derive(PartialEq))]
#[cfg_attr(feature = "ser", derive(serde::Serialize))]
#[derive(Debug)]
pub struct ListItem<'a> {
    pub bullet: Cow<'a, str>,
}

impl ListItem<'_> {
    #[inline]
    pub(crate) fn parse(text: &str, indent: usize) -> (&str, ListItem<'_>, &str) {
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
                        &text[pos..],
                        ListItem {
                            bullet: text[indent..off].into(),
                        },
                        &text[off..pos],
                    );
                }
            }
            pos = i;
        }

        (
            "",
            ListItem {
                bullet: text[indent..off].into(),
            },
            &text[off..],
        )
    }

    pub fn into_owned(self) -> ListItem<'static> {
        ListItem {
            bullet: self.bullet.into_owned().into(),
        }
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
        b'0'..=b'9' => {
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
            "",
            List {
                indent: 0,
                ordered: false,
            },
            "+ item1\n+ item2"
        ))
    );
    assert_eq!(
        List::parse("* item1\n  \n* item2"),
        Some((
            "",
            List {
                indent: 0,
                ordered: false,
            },
            "* item1\n  \n* item2"
        ))
    );
    assert_eq!(
        List::parse("* item1\n  \n   \n* item2"),
        Some((
            "* item2",
            List {
                indent: 0,
                ordered: false,
            },
            "* item1\n"
        ))
    );
    assert_eq!(
        List::parse("* item1\n  \n   "),
        Some((
            "",
            List {
                indent: 0,
                ordered: false,
            },
            "* item1\n"
        ))
    );
    assert_eq!(
        List::parse("+ item1\n  + item2\n   "),
        Some((
            "",
            List {
                indent: 0,
                ordered: false,
            },
            "+ item1\n  + item2\n"
        ))
    );
    assert_eq!(
        List::parse("+ item1\n  \n  + item2\n   \n+ item 3"),
        Some((
            "",
            List {
                indent: 0,
                ordered: false,
            },
            "+ item1\n  \n  + item2\n   \n+ item 3"
        ))
    );
    assert_eq!(
        List::parse("  + item1\n  \n  + item2"),
        Some((
            "",
            List {
                indent: 2,
                ordered: false,
            },
            "  + item1\n  \n  + item2"
        ))
    );
    assert_eq!(
        List::parse("+ 1\n\n  - 2\n\n  - 3\n\n+ 4"),
        Some((
            "",
            List {
                indent: 0,
                ordered: false,
            },
            "+ 1\n\n  - 2\n\n  - 3\n\n+ 4"
        ))
    );
}
