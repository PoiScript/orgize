use memchr::memchr;
use nom::{
    bytes::complete::{tag, take_while1},
    sequence::delimited,
    IResult,
};

#[cfg_attr(test, derive(PartialEq))]
#[cfg_attr(feature = "ser", derive(serde::Serialize))]
#[derive(Debug)]
pub struct FnDef<'a> {
    pub label: &'a str,
}

fn parse_label(input: &str) -> IResult<&str, &str> {
    let (input, label) = delimited(
        tag("[fn:"),
        take_while1(|c: char| c.is_ascii_alphanumeric() || c == '-' || c == '_'),
        tag("]"),
    )(input)?;

    Ok((input, label))
}

impl FnDef<'_> {
    #[inline]
    pub(crate) fn parse(text: &str) -> Option<(&str, FnDef<'_>, &str)> {
        let (tail, label) = parse_label(text).ok()?;

        let end = memchr(b'\n', tail.as_bytes()).unwrap_or_else(|| tail.len());

        Some((&tail[end..], FnDef { label }, &tail[0..end]))
    }
}

#[test]
fn parse() {
    assert_eq!(
        FnDef::parse("[fn:1] https://orgmode.org"),
        Some(("", FnDef { label: "1" }, " https://orgmode.org"))
    );
    assert_eq!(
        FnDef::parse("[fn:word_1] https://orgmode.org"),
        Some(("", FnDef { label: "word_1" }, " https://orgmode.org"))
    );
    assert_eq!(
        FnDef::parse("[fn:WORD-1] https://orgmode.org"),
        Some(("", FnDef { label: "WORD-1" }, " https://orgmode.org"))
    );
    assert_eq!(
        FnDef::parse("[fn:WORD]"),
        Some(("", FnDef { label: "WORD" }, ""))
    );
    assert_eq!(FnDef::parse("[fn:] https://orgmode.org"), None);
    assert_eq!(FnDef::parse("[fn:wor d] https://orgmode.org"), None);
    assert_eq!(FnDef::parse("[fn:WORD https://orgmode.org"), None);
}
