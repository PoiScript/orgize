//! Headline Title

use std::borrow::Cow;
use std::collections::HashMap;

use memchr::memrchr2;
use nom::{
    bytes::complete::{tag, take_until, take_while},
    character::complete::{anychar, space1},
    combinator::{map, opt, verify},
    error::{make_error, ErrorKind},
    multi::fold_many0,
    sequence::{delimited, preceded},
    Err, IResult,
};

use crate::{
    config::ParseConfig,
    elements::{drawer::parse_drawer_without_blank, Planning, Timestamp},
    parse::combinators::{blank_lines_count, line, one_word},
};

/// Title Element
#[cfg_attr(test, derive(PartialEq))]
#[cfg_attr(feature = "ser", derive(serde::Serialize))]
#[derive(Debug, Clone)]
pub struct Title<'a> {
    /// Headline level, number of stars
    pub level: usize,
    /// Headline priority cookie
    #[cfg_attr(feature = "ser", serde(skip_serializing_if = "Option::is_none"))]
    pub priority: Option<char>,
    /// Headline title tags
    #[cfg_attr(feature = "ser", serde(skip_serializing_if = "Vec::is_empty"))]
    pub tags: Vec<Cow<'a, str>>,
    /// Headline todo keyword
    #[cfg_attr(feature = "ser", serde(skip_serializing_if = "Option::is_none"))]
    pub keyword: Option<Cow<'a, str>>,
    /// Raw headline's text, without the stars and the tags
    pub raw: Cow<'a, str>,
    /// Planning element associated to this headline
    #[cfg_attr(feature = "ser", serde(skip_serializing_if = "Option::is_none"))]
    pub planning: Option<Box<Planning<'a>>>,
    /// Property drawer associated to this headline
    #[cfg_attr(feature = "ser", serde(skip_serializing_if = "HashMap::is_empty"))]
    pub properties: HashMap<Cow<'a, str>, Cow<'a, str>>,
    /// Numbers of blank lines between last title's line and next non-blank line
    /// or buffer's end
    pub post_blank: usize,
}

impl Title<'_> {
    pub(crate) fn parse<'a>(
        input: &'a str,
        config: &ParseConfig,
    ) -> Option<(&'a str, (Title<'a>, &'a str))> {
        parse_title(input, config).ok()
    }

    // TODO: fn is_quoted(&self) -> bool { }
    // TODO: fn is_footnote_section(&self) -> bool { }

    /// Returns this headline's closed timestamp, or `None` if not set.
    pub fn closed(&self) -> Option<&Timestamp> {
        self.planning.as_ref().and_then(|p| p.closed.as_ref())
    }

    /// Returns this headline's scheduled timestamp, or `None` if not set.
    pub fn scheduled(&self) -> Option<&Timestamp> {
        self.planning.as_ref().and_then(|p| p.scheduled.as_ref())
    }

    /// Returns this headline's deadline timestamp, or `None` if not set.
    pub fn deadline(&self) -> Option<&Timestamp> {
        self.planning.as_ref().and_then(|p| p.deadline.as_ref())
    }

    /// Returns `true` if this headline is archived
    pub fn is_archived(&self) -> bool {
        self.tags.iter().any(|tag| tag == "ARCHIVE")
    }

    /// Returns `true` if this headline is commented
    pub fn is_commented(&self) -> bool {
        self.raw.starts_with("COMMENT")
            && (self.raw.len() == 7 || self.raw[7..].starts_with(char::is_whitespace))
    }

    pub fn into_owned(self) -> Title<'static> {
        Title {
            level: self.level,
            priority: self.priority,
            tags: self
                .tags
                .into_iter()
                .map(|s| s.into_owned().into())
                .collect(),
            keyword: self.keyword.map(Into::into).map(Cow::Owned),
            raw: self.raw.into_owned().into(),
            planning: self.planning.map(|p| Box::new(p.into_owned())),
            properties: self
                .properties
                .into_iter()
                .map(|(k, v)| (k.into_owned().into(), v.into_owned().into()))
                .collect(),
            post_blank: self.post_blank,
        }
    }
}

impl Default for Title<'_> {
    fn default() -> Title<'static> {
        Title {
            level: 1,
            priority: None,
            tags: Vec::new(),
            keyword: None,
            raw: Cow::Borrowed(""),
            planning: None,
            properties: HashMap::new(),
            post_blank: 0,
        }
    }
}

#[inline]
fn parse_title<'a>(
    input: &'a str,
    config: &ParseConfig,
) -> IResult<&'a str, (Title<'a>, &'a str), ()> {
    let (input, level) = map(take_while(|c: char| c == '*'), |s: &str| s.len())(input)?;

    debug_assert!(level > 0);

    let (input, keyword) = opt(preceded(
        space1,
        verify(one_word, |s: &str| {
            config.todo_keywords.0.iter().any(|x| x == s)
                || config.todo_keywords.1.iter().any(|x| x == s)
        }),
    ))(input)?;

    let (input, priority) = opt(preceded(
        space1,
        delimited(
            tag("[#"),
            verify(anychar, |c: &char| c.is_ascii_uppercase()),
            tag("]"),
        ),
    ))(input)?;
    let (input, tail) = line(input)?;
    let tail = tail.trim();

    // tags can be separated by space or \t
    let (raw, tags) = memrchr2(b' ', b'\t', tail.as_bytes())
        .map(|i| (tail[0..i].trim(), &tail[i + 1..]))
        .filter(|(_, x)| is_tag_line(x))
        .unwrap_or((tail, ""));

    let tags = tags
        .split(':')
        .filter(|s| !s.is_empty())
        .map(Into::into)
        .collect();

    let (input, planning) = Planning::parse(input)
        .map(|(input, planning)| (input, Some(Box::new(planning))))
        .unwrap_or((input, None));

    let (input, properties) = opt(parse_properties_drawer)(input)?;
    let (input, post_blank) = blank_lines_count(input)?;

    Ok((
        input,
        (
            Title {
                properties: properties.unwrap_or_default(),
                level,
                keyword: keyword.map(Into::into),
                priority,
                tags,
                raw: raw.into(),
                planning,
                post_blank,
            },
            raw,
        ),
    ))
}

fn is_tag_line(input: &str) -> bool {
    input.len() > 2
        && input.starts_with(':')
        && input.ends_with(':')
        && input.chars().all(|ch| {
            ch.is_alphanumeric() || ch == '_' || ch == '@' || ch == '#' || ch == '%' || ch == ':'
        })
}

#[inline]
fn parse_properties_drawer(input: &str) -> IResult<&str, HashMap<Cow<'_, str>, Cow<'_, str>>, ()> {
    let (input, (drawer, content)) = parse_drawer_without_blank(input.trim_start())?;
    if drawer.name != "PROPERTIES" {
        return Err(Err::Error(make_error(input, ErrorKind::Tag)));
    }
    let (_, map) = fold_many0(
        parse_node_property,
        HashMap::new(),
        |mut acc: HashMap<_, _>, (name, value)| {
            acc.insert(name.into(), value.into());
            acc
        },
    )(content)?;
    Ok((input, map))
}

#[inline]
fn parse_node_property(input: &str) -> IResult<&str, (&str, &str), ()> {
    let (input, _) = blank_lines_count(input)?;
    let input = input.trim_start();
    let (input, name) = map(delimited(tag(":"), take_until(":"), tag(":")), |s: &str| {
        s.trim_end_matches('+')
    })(input)?;
    let (input, value) = line(input)?;
    Ok((input, (name, value.trim())))
}

#[test]
fn parse_title_() {
    use crate::config::DEFAULT_CONFIG;

    assert_eq!(
        parse_title("**** DONE [#A] COMMENT Title :tag:a2%:", &DEFAULT_CONFIG),
        Ok((
            "",
            (
                Title {
                    level: 4,
                    keyword: Some("DONE".into()),
                    priority: Some('A'),
                    raw: "COMMENT Title".into(),
                    tags: vec!["tag".into(), "a2%".into()],
                    planning: None,
                    properties: HashMap::new(),
                    post_blank: 0,
                },
                "COMMENT Title"
            )
        ))
    );
    assert_eq!(
        parse_title("**** ToDO [#A] COMMENT Title", &DEFAULT_CONFIG),
        Ok((
            "",
            (
                Title {
                    level: 4,
                    keyword: None,
                    priority: None,
                    raw: "ToDO [#A] COMMENT Title".into(),
                    tags: vec![],
                    planning: None,
                    properties: HashMap::new(),
                    post_blank: 0,
                },
                "ToDO [#A] COMMENT Title"
            )
        ))
    );
    assert_eq!(
        parse_title("**** T0DO [#A] COMMENT Title", &DEFAULT_CONFIG),
        Ok((
            "",
            (
                Title {
                    level: 4,
                    keyword: None,
                    priority: None,
                    raw: "T0DO [#A] COMMENT Title".into(),
                    tags: vec![],
                    planning: None,
                    properties: HashMap::new(),
                    post_blank: 0,
                },
                "T0DO [#A] COMMENT Title"
            )
        ))
    );
    assert_eq!(
        parse_title("**** DONE [#1] COMMENT Title", &DEFAULT_CONFIG),
        Ok((
            "",
            (
                Title {
                    level: 4,
                    keyword: Some("DONE".into()),
                    priority: None,
                    raw: "[#1] COMMENT Title".into(),
                    tags: vec![],
                    planning: None,
                    properties: HashMap::new(),
                    post_blank: 0,
                },
                "[#1] COMMENT Title"
            )
        ))
    );
    assert_eq!(
        parse_title("**** DONE [#a] COMMENT Title", &DEFAULT_CONFIG),
        Ok((
            "",
            (
                Title {
                    level: 4,
                    keyword: Some("DONE".into()),
                    priority: None,
                    raw: "[#a] COMMENT Title".into(),
                    tags: vec![],
                    planning: None,
                    properties: HashMap::new(),
                    post_blank: 0,
                },
                "[#a] COMMENT Title"
            )
        ))
    );

    // https://github.com/PoiScript/orgize/issues/20
    for raw in &["", "::", " ::", "hello", " hello"] {
        for keyword in &[Some(Cow::Borrowed("DONE")), None] {
            for raw_tags in &["", " :test:", "\t:a:b:"] {
                let mut s = String::from("** ");
                if let Some(keyword) = keyword {
                    s += keyword;
                    s.push(' ');
                }
                s += "[#B]";
                s += &raw;
                s += &raw_tags;

                // A title with an empty (or whitespace only) raw will have its
                // tags interpreted as the title, rather than as tags. This is
                // intended behavior, and consistent with org-element.
                let mut raw = raw.to_string();
                let tags: Vec<_> = if raw.trim().is_empty() {
                    raw += raw_tags;
                    Vec::new()
                } else {
                    raw_tags
                        .trim()
                        .split(':')
                        .filter_map(|t| if !t.is_empty() { Some(t.into()) } else { None })
                        .collect()
                };

                assert_eq!(
                    parse_title(&s, &DEFAULT_CONFIG),
                    Ok((
                        "",
                        (
                            Title {
                                level: 2,
                                keyword: keyword.clone(),
                                priority: Some('B'),
                                raw: Cow::Borrowed(raw.trim()),
                                tags,
                                planning: None,
                                properties: HashMap::new(),
                                post_blank: 0,
                            },
                            raw.trim(),
                        )
                    ))
                );
            }
        }
    }

    assert_eq!(
        parse_title("**** Title :tag:a2%", &DEFAULT_CONFIG),
        Ok((
            "",
            (
                Title {
                    level: 4,
                    keyword: None,
                    priority: None,
                    raw: "Title :tag:a2%".into(),
                    tags: vec![],
                    planning: None,
                    properties: HashMap::new(),
                    post_blank: 0,
                },
                "Title :tag:a2%"
            )
        ))
    );
    assert_eq!(
        parse_title("**** Title tag:a2%:", &DEFAULT_CONFIG),
        Ok((
            "",
            (
                Title {
                    level: 4,
                    keyword: None,
                    priority: None,
                    raw: "Title tag:a2%:".into(),
                    tags: vec![],
                    planning: None,
                    properties: HashMap::new(),
                    post_blank: 0,
                },
                "Title tag:a2%:"
            )
        ))
    );

    assert_eq!(
        parse_title(
            "**** DONE Title",
            &ParseConfig {
                todo_keywords: (vec![], vec![]),
                ..Default::default()
            }
        ),
        Ok((
            "",
            (
                Title {
                    level: 4,
                    keyword: None,
                    priority: None,
                    raw: "DONE Title".into(),
                    tags: vec![],
                    planning: None,
                    properties: HashMap::new(),
                    post_blank: 0,
                },
                "DONE Title"
            )
        ))
    );
    assert_eq!(
        parse_title(
            "**** TASK [#A] Title",
            &ParseConfig {
                todo_keywords: (vec!["TASK".to_string()], vec![]),
                ..Default::default()
            }
        ),
        Ok((
            "",
            (
                Title {
                    level: 4,
                    keyword: Some("TASK".into()),
                    priority: Some('A'),
                    raw: "Title".into(),
                    tags: vec![],
                    planning: None,
                    properties: HashMap::new(),
                    post_blank: 0,
                },
                "Title"
            )
        ))
    );
}

#[test]
fn parse_properties_drawer_() {
    assert_eq!(
        parse_properties_drawer("   :PROPERTIES:\n   :CUSTOM_ID: id\n   :END:"),
        Ok((
            "",
            vec![("CUSTOM_ID".into(), "id".into())]
                .into_iter()
                .collect::<HashMap<_, _>>()
        ))
    )
}
