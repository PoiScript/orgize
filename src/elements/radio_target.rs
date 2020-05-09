use nom::{
    bytes::complete::{tag, take_while},
    combinator::verify,
    sequence::delimited,
    IResult,
};

// TODO: text-markup, entities, latex-fragments, subscript and superscript

#[inline]
pub fn parse_radio_target(input: &str) -> Option<(&str, &str)> {
    parse_internal(input).ok()
}

#[inline]
fn parse_internal(input: &str) -> IResult<&str, &str, ()> {
    let (input, contents) = delimited(
        tag("<<<"),
        verify(
            take_while(|c: char| c != '<' && c != '\n' && c != '>'),
            |s: &str| s.starts_with(|c| c != ' ') && s.ends_with(|c| c != ' '),
        ),
        tag(">>>"),
    )(input)?;

    Ok((input, contents))
}

#[test]
fn parse() {
    assert_eq!(parse_radio_target("<<<target>>>"), Some(("", "target")));
    assert_eq!(parse_radio_target("<<<tar get>>>"), Some(("", "tar get")));

    assert!(parse_radio_target("<<<target >>>").is_none());
    assert!(parse_radio_target("<<< target>>>").is_none());
    assert!(parse_radio_target("<<<ta<get>>>").is_none());
    assert!(parse_radio_target("<<<ta>get>>>").is_none());
    assert!(parse_radio_target("<<<ta\nget>>>").is_none());
    assert!(parse_radio_target("<<<target>>").is_none());
}
