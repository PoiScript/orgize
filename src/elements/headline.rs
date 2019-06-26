//! Headline

use jetscii::ByteSubstring;
use memchr::{memchr, memchr2, memrchr};

pub(crate) const DEFAULT_TODO_KEYWORDS: &[&str] =
    &["TODO", "DONE", "NEXT", "WAITING", "LATER", "CANCELLED"];

#[cfg_attr(test, derive(PartialEq))]
#[derive(Debug)]
pub struct Headline<'a> {
    /// headline level, number of stars
    pub level: usize,
    /// priority cookie
    pub priority: Option<char>,
    /// headline tags, including the sparated colons
    pub tags: Vec<&'a str>,
    /// headline title
    pub title: &'a str,
    /// headline keyword
    pub keyword: Option<&'a str>,
}

impl Headline<'_> {
    pub(crate) fn parse<'a>(text: &'a str, keywords: &[&str]) -> (Headline<'a>, usize, usize) {
        let level = memchr2(b'\n', b' ', text.as_bytes()).unwrap_or_else(|| text.len());

        debug_assert!(level > 0);
        debug_assert!(text.as_bytes()[0..level].iter().all(|&c| c == b'*'));

        let (off, end) = memchr(b'\n', text.as_bytes())
            .map(|i| {
                (
                    i + 1,
                    if i + 1 == text.len() {
                        i + 1
                    } else {
                        Headline::find_level(&text[i + 1..], level) + i + 1
                    },
                )
            })
            .unwrap_or_else(|| (text.len(), text.len()));

        if level == off {
            return (
                Headline {
                    level,
                    keyword: None,
                    priority: None,
                    title: "",
                    tags: Vec::new(),
                },
                off,
                end,
            );
        }

        let tail = text[level + 1..off].trim();

        let (keyword, tail) = {
            let (word, off) = memchr(b' ', tail.as_bytes())
                .map(|i| (&tail[0..i], i + 1))
                .unwrap_or_else(|| (tail, tail.len()));
            if keywords.contains(&word) {
                (Some(word), &tail[off..])
            } else {
                (None, tail)
            }
        };

        let (priority, tail) = {
            let bytes = tail.as_bytes();
            if bytes.len() > 4
                && bytes[0] == b'['
                && bytes[1] == b'#'
                && bytes[2].is_ascii_uppercase()
                && bytes[3] == b']'
                && bytes[4] == b' '
            {
                (Some(bytes[2] as char), tail[4..].trim_start())
            } else {
                (None, tail)
            }
        };

        let (title, tags) = if let Some(i) = memrchr(b' ', tail.as_bytes()) {
            let last = &tail[i + 1..];
            if last.len() > 2 && last.starts_with(':') && last.ends_with(':') {
                (tail[..i].trim(), last)
            } else {
                (tail, "")
            }
        } else {
            (tail, "")
        };

        (
            Headline {
                level,
                keyword,
                priority,
                title,
                tags: tags.split(':').filter(|s| !s.is_empty()).collect(),
            },
            off,
            end,
        )
    }

    pub(crate) fn find_level(text: &str, level: usize) -> usize {
        let bytes = text.as_bytes();
        if bytes[0] == b'*' {
            if let Some(stars) = memchr2(b'\n', b' ', bytes) {
                if stars <= level && bytes[0..stars].iter().all(|&c| c == b'*') {
                    return 0;
                }
            }
        }

        let mut pos = 0;
        while let Some(off) = ByteSubstring::new(b"\n*").find(&bytes[pos..]) {
            pos += off + 1;
            if let Some(stars) = memchr2(b'\n', b' ', &bytes[pos..]) {
                if stars > 0 && stars <= level && bytes[pos..pos + stars].iter().all(|&c| c == b'*')
                {
                    return pos;
                }
            }
        }

        text.len()
    }

    /// checks if this headline is "commented"
    pub fn is_commented(&self) -> bool {
        self.title.starts_with("COMMENT ")
    }

    /// checks if this headline is "archived"
    pub fn is_archived(&self) -> bool {
        self.tags.contains(&"ARCHIVE")
    }
}

#[test]
fn parse() {
    assert_eq!(
        Headline::parse("**** TODO [#A] COMMENT Title :tag:a2%:", &["TODO"]).0,
        Headline {
            level: 4,
            priority: Some('A'),
            keyword: Some("TODO"),
            title: "COMMENT Title",
            tags: vec!["tag", "a2%"],
        },
    );
    assert_eq!(
        Headline::parse("**** ToDO [#A] COMMENT Title :tag:a2%:", &["TODO"]).0,
        Headline {
            level: 4,
            priority: None,
            tags: vec!["tag", "a2%"],
            title: "ToDO [#A] COMMENT Title",
            keyword: None,
        },
    );
    assert_eq!(
        Headline::parse("**** T0DO [#A] COMMENT Title :tag:a2%:", &["TODO"]).0,
        Headline {
            level: 4,
            priority: None,
            tags: vec!["tag", "a2%"],
            title: "T0DO [#A] COMMENT Title",
            keyword: None,
        },
    );
    assert_eq!(
        Headline::parse("**** TODO [#1] COMMENT Title :tag:a2%:", &["TODO"]).0,
        Headline {
            level: 4,
            priority: None,
            tags: vec!["tag", "a2%"],
            title: "[#1] COMMENT Title",
            keyword: Some("TODO")
        },
    );
    assert_eq!(
        Headline::parse("**** TODO [#a] COMMENT Title :tag:a2%:", &["TODO"]).0,
        Headline {
            level: 4,
            priority: None,
            tags: vec!["tag", "a2%"],
            title: "[#a] COMMENT Title",
            keyword: Some("TODO")
        },
    );
    assert_eq!(
        Headline::parse("**** TODO [#A] COMMENT Title :tag:a2%", &["TODO"]).0,
        Headline {
            level: 4,
            priority: Some('A'),
            tags: Vec::new(),
            title: "COMMENT Title :tag:a2%",
            keyword: Some("TODO"),
        },
    );
    assert_eq!(
        Headline::parse("**** TODO [#A] COMMENT Title tag:a2%:", &["TODO"]).0,
        Headline {
            level: 4,
            priority: Some('A'),
            tags: Vec::new(),
            title: "COMMENT Title tag:a2%:",
            keyword: Some("TODO"),
        },
    );
    assert_eq!(
        Headline::parse("**** COMMENT Title tag:a2%:", &["TODO"]).0,
        Headline {
            level: 4,
            priority: None,
            tags: Vec::new(),
            title: "COMMENT Title tag:a2%:",
            keyword: None,
        },
    );
}

#[test]
fn parse_todo_keywords() {
    assert_eq!(
        Headline::parse("**** TODO [#A] COMMENT Title :tag:a2%:", &[]).0,
        Headline {
            level: 4,
            priority: None,
            keyword: None,
            title: "TODO [#A] COMMENT Title",
            tags: vec!["tag", "a2%"],
        },
    );
    assert_eq!(
        Headline::parse("**** TASK [#A] COMMENT Title :tag:a2%:", &["TASK"]).0,
        Headline {
            level: 4,
            priority: Some('A'),
            keyword: Some("TASK"),
            title: "COMMENT Title",
            tags: vec!["tag", "a2%"],
        },
    );
}

#[test]
fn is_commented() {
    assert!(Headline::parse("* COMMENT Title", &[]).0.is_commented());
    assert!(!Headline::parse("* Title", &[]).0.is_commented());
    assert!(!Headline::parse("* C0MMENT Title", &[]).0.is_commented());
    assert!(!Headline::parse("* comment Title", &[]).0.is_commented());
}

#[test]
fn is_archived() {
    assert!(Headline::parse("* Title :ARCHIVE:", &[]).0.is_archived());
    assert!(Headline::parse("* Title :t:ARCHIVE:", &[]).0.is_archived());
    assert!(Headline::parse("* Title :ARCHIVE:t:", &[]).0.is_archived());
    assert!(!Headline::parse("* Title", &[]).0.is_commented());
    assert!(!Headline::parse("* Title :ARCHIVED:", &[]).0.is_archived());
    assert!(!Headline::parse("* Title :ARCHIVES:", &[]).0.is_archived());
    assert!(!Headline::parse("* Title :archive:", &[]).0.is_archived());
}

#[test]
fn find_level() {
    assert_eq!(
        Headline::find_level("\n** Title\n* Title\n** Title\n", 1),
        "\n** Title\n".len()
    );
}
