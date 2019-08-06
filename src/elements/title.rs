//! Headline Title

use memchr::memrchr;
use nom::{
    bytes::complete::{tag, take_until, take_while},
    character::complete::{anychar, space1},
    combinator::{map, map_parser, opt, verify},
    multi::fold_many0,
    sequence::delimited,
    sequence::preceded,
    IResult,
};
use std::collections::HashMap;

use crate::config::ParseConfig;
use crate::elements::{Drawer, Planning};
use crate::parsers::{skip_empty_lines, take_one_word, take_until_eol};

#[cfg_attr(test, derive(PartialEq))]
#[cfg_attr(feature = "ser", derive(serde::Serialize))]
#[derive(Debug)]
pub struct Title<'a> {
    /// headline level, number of stars
    pub level: usize,
    /// priority cookie
    #[cfg_attr(feature = "ser", serde(skip_serializing_if = "Option::is_none"))]
    pub priority: Option<char>,
    /// headline tags, including the sparated colons
    #[cfg_attr(feature = "ser", serde(skip_serializing_if = "Vec::is_empty"))]
    pub tags: Vec<&'a str>,
    /// headline keyword
    #[cfg_attr(feature = "ser", serde(skip_serializing_if = "Option::is_none"))]
    pub keyword: Option<&'a str>,
    pub raw: &'a str,
    #[cfg_attr(feature = "ser", serde(skip_serializing_if = "Option::is_none"))]
    pub planning: Option<Box<Planning<'a>>>,
    #[cfg_attr(feature = "ser", serde(skip_serializing_if = "HashMap::is_empty"))]
    pub properties: HashMap<&'a str, &'a str>,
}

impl Title<'_> {
    #[inline]
    pub(crate) fn parse<'a>(input: &'a str, config: &ParseConfig) -> IResult<&'a str, Title<'a>> {
        let (input, (level, keyword, priority, raw, tags)) = parse_headline(input, config)?;

        let (input, planning) = Planning::parse(input)
            .map(|(input, planning)| (input, Some(Box::new(planning))))
            .unwrap_or((input, None));

        let (input, properties) = opt(parse_properties_drawer)(input)?;

        Ok((
            input,
            Title {
                properties: properties.unwrap_or_default(),
                level,
                keyword,
                priority,
                tags,
                raw,
                planning,
            },
        ))
    }
}

fn parse_headline<'a>(
    input: &'a str,
    config: &ParseConfig,
) -> IResult<&'a str, (usize, Option<&'a str>, Option<char>, &'a str, Vec<&'a str>)> {
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
    let (input, tail) = take_until_eol(input)?;
    let (raw, tags) = memrchr(b' ', tail.as_bytes())
        .map(|i| (tail[0..i].trim(), &tail[i + 1..]))
        .filter(|(_, x)| x.len() > 2 && x.starts_with(':') && x.ends_with(':'))
        .unwrap_or((tail, ""));

    Ok((
        input,
        (
            level,
            keyword,
            priority,
            raw,
            tags.split(':').filter(|s| !s.is_empty()).collect(),
        ),
    ))
}

fn parse_properties_drawer(input: &str) -> IResult<&str, HashMap<&str, &str>> {
    let (input, (drawer, content)) = Drawer::parse(input)?;
    let _ = tag("PROPERTIES")(drawer.name)?;
    let (_, map) = fold_many0(
        parse_node_property,
        HashMap::new(),
        |mut acc: HashMap<_, _>, (name, value)| {
            acc.insert(name, value);
            acc
        },
    )(content)?;
    Ok((input, map))
}

fn parse_node_property(input: &str) -> IResult<&str, (&str, &str)> {
    let input = skip_empty_lines(input);
    let (input, name) = map(delimited(tag(":"), take_until(":"), tag(":")), |s: &str| {
        s.trim_end_matches('+')
    })(input)?;
    let (input, value) = take_until_eol(input)?;
    Ok((input, (name, value)))
}

impl Title<'_> {
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
fn parse_headline_() {
    assert_eq!(
        parse_headline("**** DONE [#A] COMMENT Title :tag:a2%:", &CONFIG),
        Ok((
            "",
            (
                4,
                Some("DONE"),
                Some('A'),
                "COMMENT Title",
                vec!["tag", "a2%"]
            )
        ))
    );
    assert_eq!(
        parse_headline("**** ToDO [#A] COMMENT Title", &CONFIG),
        Ok(("", (4, None, None, "ToDO [#A] COMMENT Title", vec![])))
    );
    assert_eq!(
        parse_headline("**** T0DO [#A] COMMENT Title", &CONFIG),
        Ok(("", (4, None, None, "T0DO [#A] COMMENT Title", vec![])))
    );
    assert_eq!(
        parse_headline("**** DONE [#1] COMMENT Title", &CONFIG),
        Ok(("", (4, Some("DONE"), None, "[#1] COMMENT Title", vec![],)))
    );
    assert_eq!(
        parse_headline("**** DONE [#a] COMMENT Title", &CONFIG),
        Ok(("", (4, Some("DONE"), None, "[#a] COMMENT Title", vec![],)))
    );
    assert_eq!(
        parse_headline("**** Title :tag:a2%", &CONFIG),
        Ok(("", (4, None, None, "Title :tag:a2%", vec![],)))
    );
    assert_eq!(
        parse_headline("**** Title tag:a2%:", &CONFIG),
        Ok(("", (4, None, None, "Title tag:a2%:", vec![],)))
    );

    assert_eq!(
        parse_headline(
            "**** DONE Title",
            &ParseConfig {
                done_keywords: vec![],
                ..Default::default()
            }
        ),
        Ok(("", (4, None, None, "DONE Title", vec![])))
    );
    assert_eq!(
        parse_headline(
            "**** TASK [#A] Title",
            &ParseConfig {
                todo_keywords: vec!["TASK".to_string()],
                ..Default::default()
            }
        ),
        Ok(("", (4, Some("TASK"), Some('A'), "Title", vec![],)))
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
