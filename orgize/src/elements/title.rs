//! Headline Title

use std::borrow::Cow;
use std::collections::HashMap;

use memchr::memrchr;
use nom::{
    bytes::complete::{tag, take_until, take_while},
    character::complete::{anychar, space1},
    combinator::{map, map_parser, opt, verify},
    error::ErrorKind,
    error_position,
    multi::fold_many0,
    sequence::{delimited, preceded},
    Err, IResult,
};

use crate::config::ParseConfig;
use crate::elements::{Drawer, Planning, Timestamp};
use crate::parsers::{line, skip_empty_lines, take_one_word};

/// Title Elemenet
#[cfg_attr(test, derive(PartialEq))]
#[cfg_attr(feature = "ser", derive(serde::Serialize))]
#[derive(Debug)]
pub struct Title<'a> {
    /// Headline level, number of stars
    pub level: usize,
    /// Headline priority cookie
    #[cfg_attr(feature = "ser", serde(skip_serializing_if = "Option::is_none"))]
    pub priority: Option<char>,
    /// Headline title tags, including the sparated colons
    #[cfg_attr(feature = "ser", serde(skip_serializing_if = "Vec::is_empty"))]
    pub tags: Vec<Cow<'a, str>>,
    /// Headline title keyword
    #[cfg_attr(feature = "ser", serde(skip_serializing_if = "Option::is_none"))]
    pub keyword: Option<Cow<'a, str>>,
    /// Raw headline's text, without the stars and the tags
    pub raw: Cow<'a, str>,
    /// Planning elemenet associated to this headline
    #[cfg_attr(feature = "ser", serde(skip_serializing_if = "Option::is_none"))]
    pub planning: Option<Box<Planning<'a>>>,
    /// Property drawer associated to this headline
    #[cfg_attr(feature = "ser", serde(skip_serializing_if = "HashMap::is_empty"))]
    pub properties: HashMap<Cow<'a, str>, Cow<'a, str>>,
}

impl Title<'_> {
    #[inline]
    pub(crate) fn parse<'a>(
        input: &'a str,
        config: &ParseConfig,
    ) -> IResult<&'a str, (Title<'a>, &'a str)> {
        let (input, level) = map(take_while(|c: char| c == '*'), |s: &str| s.len())(input)?;

        debug_assert!(level > 0);

        let (input, keyword) = opt(preceded(
            space1,
            verify(take_one_word, |s: &str| {
                config.todo_keywords.iter().any(|x| x == s)
                    || config.done_keywords.iter().any(|x| x == s)
            }),
        ))(input)?;

        let (input, priority) = opt(preceded(
            space1,
            map_parser(
                take_one_word,
                delimited(
                    tag("[#"),
                    verify(anychar, |c: &char| c.is_ascii_uppercase()),
                    tag("]"),
                ),
            ),
        ))(input)?;
        let (input, tail) = line(input)?;
        let tail = tail.trim();
        let (raw, tags) = memrchr(b' ', tail.as_bytes())
            .map(|i| (tail[0..i].trim(), &tail[i + 1..]))
            .filter(|(_, x)| x.len() > 2 && x.starts_with(':') && x.ends_with(':'))
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
                },
                raw,
            ),
        ))
    }

    // TODO: fn is_archived(&self) -> bool { }
    // TODO: fn is_commented(&self) -> bool { }
    // TODO: fn is_quoted(&self) -> bool { }
    // TODO: fn is_footnote_section(&self) -> bool { }

    /// Returns this headline's closed timestamp, or `None` if not set.
    pub fn closed(&self) -> Option<&Timestamp> {
        self.planning
            .as_ref()
            .and_then(|planning| planning.closed.as_ref())
    }

    /// Returns this headline's scheduled timestamp, or `None` if not set.
    pub fn scheduled(&self) -> Option<&Timestamp> {
        self.planning
            .as_ref()
            .and_then(|planning| planning.scheduled.as_ref())
    }

    /// Returns this headline's deadline timestamp, or `None` if not set.
    pub fn deadline(&self) -> Option<&Timestamp> {
        self.planning
            .as_ref()
            .and_then(|planning| planning.deadline.as_ref())
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
        }
    }
}

fn parse_properties_drawer(input: &str) -> IResult<&str, HashMap<Cow<'_, str>, Cow<'_, str>>> {
    let (input, (drawer, content)) = Drawer::parse(input.trim_start())?;
    if drawer.name != "PROPERTIES" {
        return Err(Err::Error(error_position!(input, ErrorKind::Tag)));
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

fn parse_node_property(input: &str) -> IResult<&str, (&str, &str)> {
    let input = skip_empty_lines(input).trim_start();
    let (input, name) = map(delimited(tag(":"), take_until(":"), tag(":")), |s: &str| {
        s.trim_end_matches('+')
    })(input)?;
    let (input, value) = line(input)?;
    Ok((input, (name, value.trim())))
}

impl Title<'_> {
    /// checks if this headline is "archived"
    pub fn is_archived(&self) -> bool {
        self.tags.iter().any(|tag| tag == "ARCHIVE")
    }
}

#[test]
fn parse_title() {
    use crate::config::DEFAULT_CONFIG;

    assert_eq!(
        Title::parse("**** DONE [#A] COMMENT Title :tag:a2%:", &DEFAULT_CONFIG),
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
                    properties: HashMap::new()
                },
                "COMMENT Title"
            )
        ))
    );
    assert_eq!(
        Title::parse("**** ToDO [#A] COMMENT Title", &DEFAULT_CONFIG),
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
                    properties: HashMap::new()
                },
                "ToDO [#A] COMMENT Title"
            )
        ))
    );
    assert_eq!(
        Title::parse("**** T0DO [#A] COMMENT Title", &DEFAULT_CONFIG),
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
                    properties: HashMap::new()
                },
                "T0DO [#A] COMMENT Title"
            )
        ))
    );
    assert_eq!(
        Title::parse("**** DONE [#1] COMMENT Title", &DEFAULT_CONFIG),
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
                    properties: HashMap::new()
                },
                "[#1] COMMENT Title"
            )
        ))
    );
    assert_eq!(
        Title::parse("**** DONE [#a] COMMENT Title", &DEFAULT_CONFIG),
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
                    properties: HashMap::new()
                },
                "[#a] COMMENT Title"
            )
        ))
    );
    assert_eq!(
        Title::parse("**** Title :tag:a2%", &DEFAULT_CONFIG),
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
                    properties: HashMap::new()
                },
                "Title :tag:a2%"
            )
        ))
    );
    assert_eq!(
        Title::parse("**** Title tag:a2%:", &DEFAULT_CONFIG),
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
                    properties: HashMap::new()
                },
                "Title tag:a2%:"
            )
        ))
    );

    assert_eq!(
        Title::parse(
            "**** DONE Title",
            &ParseConfig {
                done_keywords: vec![],
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
                    properties: HashMap::new()
                },
                "DONE Title"
            )
        ))
    );
    assert_eq!(
        Title::parse(
            "**** TASK [#A] Title",
            &ParseConfig {
                todo_keywords: vec!["TASK".to_string()],
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
                    properties: HashMap::new()
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

// #[test]
// fn is_commented() {
//     assert!(Title::parse("* COMMENT Title", &DEFAULT_CONFIG)
//         .1
//         .is_commented());
//     assert!(!Title::parse("* Title", &DEFAULT_CONFIG).1.is_commented());
//     assert!(!Title::parse("* C0MMENT Title", &DEFAULT_CONFIG)
//         .1
//         .is_commented());
//     assert!(!Title::parse("* comment Title", &DEFAULT_CONFIG)
//         .1
//         .is_commented());
// }
