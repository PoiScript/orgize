//! Headline Title

use memchr::{memchr, memchr2, memrchr};

use crate::config::ParseConfig;

#[cfg_attr(test, derive(PartialEq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Debug)]
pub struct Title<'a> {
    /// headline level, number of stars
    pub level: usize,
    /// priority cookie
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub priority: Option<char>,
    /// headline tags, including the sparated colons
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub tags: Vec<&'a str>,
    /// headline keyword
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub keyword: Option<&'a str>,
    pub raw: &'a str,
}

impl Title<'_> {
    #[inline]
    pub(crate) fn parse<'a>(text: &'a str, config: &ParseConfig) -> (&'a str, Title<'a>, &'a str) {
        let level = memchr2(b'\n', b' ', text.as_bytes()).unwrap_or_else(|| text.len());

        debug_assert!(level > 0);
        debug_assert!(text.as_bytes()[0..level].iter().all(|&c| c == b'*'));

        let off = memchr(b'\n', text.as_bytes())
            .map(|i| i + 1)
            .unwrap_or_else(|| text.len());

        if level == off {
            return (
                "",
                Title {
                    level,
                    keyword: None,
                    priority: None,
                    tags: Vec::new(),
                    raw: "",
                },
                "",
            );
        }

        let tail = text[level + 1..off].trim();

        let (keyword, tail) = {
            let (word, off) = memchr(b' ', tail.as_bytes())
                .map(|i| (&tail[0..i], i + 1))
                .unwrap_or_else(|| (tail, tail.len()));
            if config.todo_keywords.iter().any(|x| x == word)
                || config.done_keywords.iter().any(|x| x == word)
            {
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
            &text[off..],
            Title {
                level,
                keyword,
                priority,
                tags: tags.split(':').filter(|s| !s.is_empty()).collect(),
                raw: title,
            },
            title,
        )
    }

    /// checks if this headline is "archived"
    pub fn is_archived(&self) -> bool {
        self.tags.contains(&"ARCHIVE")
    }
}

#[cfg(test)]
lazy_static::lazy_static! {
    static ref CONFIG: ParseConfig = ParseConfig::default();
}

#[test]
fn parse() {
    assert_eq!(
        Title::parse("**** DONE [#A] COMMENT Title :tag:a2%:", &CONFIG),
        (
            "",
            Title {
                level: 4,
                priority: Some('A'),
                keyword: Some("DONE"),
                tags: vec!["tag", "a2%"],
                raw: "COMMENT Title"
            },
            "COMMENT Title"
        )
    );
    assert_eq!(
        Title::parse("**** ToDO [#A] COMMENT Title :tag:a2%:", &CONFIG),
        (
            "",
            Title {
                level: 4,
                priority: None,
                tags: vec!["tag", "a2%"],
                keyword: None,
                raw: "ToDO [#A] COMMENT Title"
            },
            "ToDO [#A] COMMENT Title"
        )
    );
    assert_eq!(
        Title::parse("**** T0DO [#A] COMMENT Title :tag:a2%:", &CONFIG),
        (
            "",
            Title {
                level: 4,
                priority: None,
                tags: vec!["tag", "a2%"],
                keyword: None,
                raw: "T0DO [#A] COMMENT Title"
            },
            "T0DO [#A] COMMENT Title"
        )
    );
    assert_eq!(
        Title::parse("**** DONE [#1] COMMENT Title :tag:a2%:", &CONFIG),
        (
            "",
            Title {
                level: 4,
                priority: None,
                tags: vec!["tag", "a2%"],
                keyword: Some("DONE"),
                raw: "[#1] COMMENT Title"
            },
            "[#1] COMMENT Title"
        )
    );
    assert_eq!(
        Title::parse("**** DONE [#a] COMMENT Title :tag:a2%:", &CONFIG),
        (
            "",
            Title {
                level: 4,
                priority: None,
                tags: vec!["tag", "a2%"],
                keyword: Some("DONE"),
                raw: "[#a] COMMENT Title"
            },
            "[#a] COMMENT Title"
        )
    );
    assert_eq!(
        Title::parse("**** DONE [#A] COMMENT Title :tag:a2%", &CONFIG),
        (
            "",
            Title {
                level: 4,
                priority: Some('A'),
                tags: Vec::new(),
                keyword: Some("DONE"),
                raw: "COMMENT Title :tag:a2%"
            },
            "COMMENT Title :tag:a2%"
        )
    );
    assert_eq!(
        Title::parse("**** DONE [#A] COMMENT Title tag:a2%:", &CONFIG),
        (
            "",
            Title {
                level: 4,
                priority: Some('A'),
                tags: Vec::new(),
                keyword: Some("DONE"),
                raw: "COMMENT Title tag:a2%:"
            },
            "COMMENT Title tag:a2%:"
        )
    );
    assert_eq!(
        Title::parse("**** COMMENT Title tag:a2%:", &CONFIG),
        (
            "",
            Title {
                level: 4,
                priority: None,
                tags: Vec::new(),
                keyword: None,
                raw: "COMMENT Title tag:a2%:"
            },
            "COMMENT Title tag:a2%:"
        )
    );

    assert_eq!(
        Title::parse(
            "**** DONE [#A] COMMENT Title :tag:a2%:",
            &ParseConfig {
                done_keywords: vec![],
                ..Default::default()
            }
        ),
        (
            "",
            Title {
                level: 4,
                priority: None,
                keyword: None,
                tags: vec!["tag", "a2%"],
                raw: "DONE [#A] COMMENT Title"
            },
            "DONE [#A] COMMENT Title"
        )
    );
    assert_eq!(
        Title::parse(
            "**** TASK [#A] COMMENT Title :tag:a2%:",
            &ParseConfig {
                todo_keywords: vec!["TASK".to_string()],
                ..Default::default()
            }
        ),
        (
            "",
            Title {
                level: 4,
                priority: Some('A'),
                keyword: Some("TASK"),
                tags: vec!["tag", "a2%"],
                raw: "COMMENT Title"
            },
            "COMMENT Title"
        )
    );
}

// #[test]
// fn is_commented() {
//     assert!(Title::parse("* COMMENT Title", &CONFIG)
//         .1
//         .is_commented());
//     assert!(!Title::parse("* Title", &CONFIG).1.is_commented());
//     assert!(!Title::parse("* C0MMENT Title", &CONFIG)
//         .1
//         .is_commented());
//     assert!(!Title::parse("* comment Title", &CONFIG)
//         .1
//         .is_commented());
// }

#[test]
fn is_archived() {
    assert!(Title::parse("* Title :ARCHIVE:", &CONFIG).1.is_archived());
    assert!(Title::parse("* Title :t:ARCHIVE:", &CONFIG).1.is_archived());
    assert!(Title::parse("* Title :ARCHIVE:t:", &CONFIG).1.is_archived());
    assert!(!Title::parse("* Title", &CONFIG).1.is_archived());
    assert!(!Title::parse("* Title :ARCHIVED:", &CONFIG).1.is_archived());
    assert!(!Title::parse("* Title :ARCHIVES:", &CONFIG).1.is_archived());
    assert!(!Title::parse("* Title :archive:", &CONFIG).1.is_archived());
}
