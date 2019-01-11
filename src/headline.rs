#[cfg_attr(test, derive(PartialEq))]
#[derive(Debug)]
pub struct Headline<'a> {
    pub level: usize,
    pub priority: Option<char>,
    pub tags: Option<&'a str>,
    pub title: &'a str,
    pub keyword: Option<&'a str>,
}

impl<'a> Headline<'a> {
    #[inline]
    fn parse_priority(src: &str) -> Option<char> {
        let bytes = src.as_bytes();
        if bytes.len() > 4
            && bytes[0] == b'['
            && bytes[1] == b'#'
            && bytes[2].is_ascii_uppercase()
            && bytes[3] == b']'
            && bytes[4] == b' '
        {
            Some(bytes[2] as char)
        } else {
            None
        }
    }

    #[inline]
    fn parse_keyword(src: &'a str) -> Option<(&'a str, usize)> {
        let mut pos = 0;
        while pos < src.len() {
            if src.as_bytes()[pos] == b' ' {
                break;
            } else if src.as_bytes()[pos].is_ascii_uppercase() {
                pos += 1;
            } else {
                return None;
            }
        }
        if pos == src.len() || src[0..pos] == *"COMMENT" {
            None
        } else {
            Some((&src[0..pos], pos))
        }
    }

    #[inline]
    fn parse_tags(src: &'a str) -> (Option<&'a str>, usize) {
        if let Some(last) = src.split_whitespace().last() {
            if last.starts_with(':') && last.ends_with(':') {
                (Some(last), src.rfind(':').unwrap() - last.len())
            } else {
                (None, src.len())
            }
        } else {
            (None, src.len())
        }
    }

    pub fn parse(src: &'a str) -> (Headline<'a>, usize, usize) {
        let mut level = 0;
        loop {
            if src.as_bytes()[level] == b'*' {
                level += 1;
            } else {
                break;
            }
        }

        let eol = eol!(src);
        let end = Headline::find_level(&src[eol..], level) + eol;

        let mut title_start = skip_space!(src, level);

        let keyword = match Headline::parse_keyword(&src[title_start..eol]) {
            Some((k, l)) => {
                title_start += l;
                Some(k)
            }
            None => None,
        };

        title_start = skip_space!(src, title_start);

        let priority = match Headline::parse_priority(&src[title_start..eol]) {
            Some(p) => {
                title_start += 4;
                Some(p)
            }
            None => None,
        };

        title_start = skip_space!(src, title_start);

        let (tags, title_off) = Headline::parse_tags(&src[title_start..eol]);

        // println!("{:?} {:?} {:?}", keyword, priority, tags);
        // println!("{:?} {}", title_start, title_off);

        (
            Headline::new(
                level,
                keyword,
                priority,
                &src[title_start..title_start + title_off],
                tags,
            ),
            eol,
            end,
        )
    }

    // TODO: optimize
    pub fn find_level(src: &str, level: usize) -> usize {
        let mut pos = 0;
        loop {
            if pos >= src.len() {
                return src.len();
            }

            if src.as_bytes()[pos] == b'*' && (pos == 0 || src.as_bytes()[pos - 1] == b'\n') {
                let pos_ = pos;
                loop {
                    if pos >= src.len() {
                        return src.len();
                    }
                    if src.as_bytes()[pos] == b'*' {
                        pos += 1;
                    } else if src.as_bytes()[pos] == b' ' && pos - pos_ <= level {
                        return pos_;
                    } else {
                        break;
                    }
                }
            }

            pos += 1
        }
    }

    pub fn is_commented(&self) -> bool {
        self.title.starts_with("COMMENT ")
    }

    pub fn is_archived(&self) -> bool {
        self.tags
            .map(|tags| tags[1..].split_terminator(':').any(|t| t == "ARCHIVE"))
            .unwrap_or(false)
    }

    pub fn new(
        level: usize,
        keyword: Option<&'a str>,
        priority: Option<char>,
        title: &'a str,
        tags: Option<&'a str>,
    ) -> Headline<'a> {
        Headline {
            level,
            keyword,
            priority,
            title,
            tags,
        }
    }
}

#[test]
fn parse() {
    assert_eq!(
        Headline::parse("**** TODO [#A] COMMENT Title :tag:a2%:").0,
        Headline::new(
            4,
            Some("TODO"),
            Some('A'),
            "COMMENT Title",
            Some(":tag:a2%:"),
        ),
    );
    assert_eq!(
        Headline::parse("**** ToDO [#A] COMMENT Title :tag:a2%:").0,
        Headline {
            level: 4,
            priority: None,
            tags: Some(":tag:a2%:"),
            title: "ToDO [#A] COMMENT Title",
            keyword: None,
        },
    );
    assert_eq!(
        Headline::parse("**** T0DO [#A] COMMENT Title :tag:a2%:").0,
        Headline {
            level: 4,
            priority: None,
            tags: Some(":tag:a2%:"),
            title: "T0DO [#A] COMMENT Title",
            keyword: None,
        },
    );
    assert_eq!(
        Headline::parse("**** TODO [#1] COMMENT Title :tag:a2%:").0,
        Headline {
            level: 4,
            priority: None,
            tags: Some(":tag:a2%:"),
            title: "[#1] COMMENT Title",
            keyword: Some("TODO")
        },
    );
    assert_eq!(
        Headline::parse("**** TODO [#a] COMMENT Title :tag:a2%:").0,
        Headline {
            level: 4,
            priority: None,
            tags: Some(":tag:a2%:"),
            title: "[#a] COMMENT Title",
            keyword: Some("TODO")
        },
    );
    assert_eq!(
        Headline::parse("**** TODO [#A] COMMENT Title :tag:a2%").0,
        Headline {
            level: 4,
            priority: Some('A'),
            tags: None,
            title: "COMMENT Title :tag:a2%",
            keyword: Some("TODO"),
        },
    );
    assert_eq!(
        Headline::parse("**** TODO [#A] COMMENT Title tag:a2%:").0,
        Headline {
            level: 4,
            priority: Some('A'),
            tags: None,
            title: "COMMENT Title tag:a2%:",
            keyword: Some("TODO"),
        },
    );
    assert_eq!(
        Headline::parse("**** COMMENT Title tag:a2%:").0,
        Headline {
            level: 4,
            priority: None,
            tags: None,
            title: "COMMENT Title tag:a2%:",
            keyword: None,
        },
    );
}

#[test]
fn is_commented() {
    assert!(Headline::parse("* COMMENT Title").0.is_commented());
    assert!(!Headline::parse("* Title").0.is_commented());
    assert!(!Headline::parse("* C0MMENT Title").0.is_commented());
    assert!(!Headline::parse("* comment Title").0.is_commented());
}

#[test]
fn is_archived() {
    assert!(Headline::parse("* Title :ARCHIVE:").0.is_archived());
    assert!(Headline::parse("* Title :tag:ARCHIVE:").0.is_archived());
    assert!(Headline::parse("* Title :ARCHIVE:tag:").0.is_archived());
    assert!(!Headline::parse("* Title").0.is_commented());
    assert!(!Headline::parse("* Title :ARCHIVED:").0.is_archived());
    assert!(!Headline::parse("* Title :ARCHIVES:").0.is_archived());
    assert!(!Headline::parse("* Title :archive:").0.is_archived());
}
