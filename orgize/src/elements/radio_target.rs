use nom::{
    bytes::complete::{tag, take_while},
    combinator::verify,
    error::ParseError,
    sequence::delimited,
    IResult,
};

// TODO: text-markup, entities, latex-fragments, subscript and superscript

#[inline]
pub fn parse_radio_target(input: &str) -> Option<(&str, &str)> {
    parse_radio_target_internal::<()>(input).ok()
}

#[inline]
fn parse_radio_target_internal<'a, E: ParseError<&'a str>>(
    input: &'a str,
) -> IResult<&'a str, &'a str, E> {
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
    use nom::error::VerboseError;

    assert_eq!(
        parse_radio_target_internal::<VerboseError<&str>>("<<<target>>>"),
        Ok(("", "target"))
    );
    assert_eq!(
        parse_radio_target_internal::<VerboseError<&str>>("<<<tar get>>>"),
        Ok(("", "tar get"))
    );
    assert!(parse_radio_target_internal::<VerboseError<&str>>("<<<target >>>").is_err());
    assert!(parse_radio_target_internal::<VerboseError<&str>>("<<< target>>>").is_err());
    assert!(parse_radio_target_internal::<VerboseError<&str>>("<<<ta<get>>>").is_err());
    assert!(parse_radio_target_internal::<VerboseError<&str>>("<<<ta>get>>>").is_err());
    assert!(parse_radio_target_internal::<VerboseError<&str>>("<<<ta\nget>>>").is_err());
    assert!(parse_radio_target_internal::<VerboseError<&str>>("<<<target>>").is_err());
}
