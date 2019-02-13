//! Headline

use memchr::memchr2;

#[cfg_attr(test, derive(PartialEq))]
#[derive(Debug)]
pub struct Headline<'a> {
    /// headline level, number of stars
    pub level: usize,
    /// priority cookie
    pub priority: Option<char>,
    /// headline tags, including the sparated colons
    pub tags: Option<&'a str>,
    /// headline title
    pub title: &'a str,
    /// headline keyword
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
        let pos = memchr2(b' ', b'\n', src.as_bytes()).unwrap_or_else(|| src.len());
        let word = &src[0..pos];
        if word.as_bytes().iter().all(|&c| c.is_ascii_uppercase()) && word != "COMMENT" {
            Some((word, pos))
        } else {
            None
        }
    }

    #[inline]
    fn parse_tags(src: &'a str) -> (Option<&'a str>, usize) {
        if let Some(last) = src.split_whitespace().last() {
            if last.len() > 2 && last.starts_with(':') && last.ends_with(':') {
                return (
                    Some(last),
                    memchr::memrchr(b':', src.as_bytes()).unwrap() - last.len(),
                );
            }
        }

        (None, src.len())
    }

    /// parsing the input string and returning the parsed headline
    /// and the content-begin and the end of headline container.
    ///
    /// ```rust
    /// use orgize::headline::Headline;
    ///
    /// let (hdl, _, _) = Headline::parse("* DONE [#A] COMMENT Title :tag:a2%:");
    ///
    /// assert_eq!(hdl.level, 1);
    /// assert_eq!(hdl.priority, Some('A'));
    /// assert_eq!(hdl.tags, Some(":tag:a2%:"));
    /// assert_eq!(hdl.title, "COMMENT Title");
    /// assert_eq!(hdl.keyword, Some("DONE"));
    /// ```
    pub fn parse(src: &'a str) -> (Headline<'a>, usize, usize) {
        let level = memchr2(b'\n', b' ', src.as_bytes()).unwrap_or_else(|| src.len());

        debug_assert!(level > 0);
        debug_assert!(src.as_bytes()[0..level].iter().all(|&c| c == b'*'));

        let (eol, end) = memchr::memchr(b'\n', src.as_bytes())
            .map(|i| (i, Headline::find_level(&src[i..], level) + i))
            .unwrap_or_else(|| (src.len(), src.len()));

        let mut title_start = skip_space!(src, level);

        let keyword = Headline::parse_keyword(&src[title_start..eol]).map(|(k, l)| {
            title_start += l;
            k
        });

        title_start = skip_space!(src, title_start);

        let priority = Headline::parse_priority(&src[title_start..eol]).map(|p| {
            title_start += 4;
            p
        });

        title_start = skip_space!(src, title_start);

        let (tags, title_off) = Headline::parse_tags(&src[title_start..eol]);

        (
            Headline {
                level,
                keyword,
                priority,
                title: &src[title_start..title_start + title_off],
                tags,
            },
            eol,
            end,
        )
    }

    pub fn find_level(src: &str, level: usize) -> usize {
        use jetscii::ByteSubstring;

        let bytes = src.as_bytes();
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

        src.len()
    }

    /// checks if this headline is "commented"
    pub fn is_commented(&self) -> bool {
        self.title.starts_with("COMMENT ")
    }

    /// checks if this headline is "archived"
    pub fn is_archived(&self) -> bool {
        self.tags
            .map(|tags| tags[1..].split_terminator(':').any(|t| t == "ARCHIVE"))
            .unwrap_or(false)
    }
}

#[test]
fn parse() {
    assert_eq!(
        Headline::parse("**** TODO [#A] COMMENT Title :tag:a2%:").0,
        Headline {
            level: 4,
            priority: Some('A'),
            keyword: Some("TODO"),
            title: "COMMENT Title",
            tags: Some(":tag:a2%:"),
        },
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

#[test]
fn find_level() {
    assert_eq!(
        Headline::find_level(
            r#"
** Title
* Title
** Title"#,
            1
        ),
        10
    );
}
