// resued nom parsers

use memchr::{memchr, memchr_iter};
use nom::{
    branch::alt,
    bytes::complete::{tag, take_till},
    character::complete::space0,
    error::ErrorKind,
    error_position, Err, IResult,
};

pub(crate) fn eol(input: &str) -> IResult<&str, ()> {
    let (input, _) = space0(input)?;
    if input.is_empty() {
        Ok(("", ()))
    } else {
        let (input, _) = tag("\n")(input)?;
        Ok((input, ()))
    }
}

pub(crate) fn take_until_eol(input: &str) -> IResult<&str, &str> {
    if let Some(i) = memchr(b'\n', input.as_bytes()) {
        Ok((&input[i + 1..], input[0..i].trim()))
    } else {
        Ok(("", input.trim()))
    }
}

pub(crate) fn take_lines_till(
    predicate: impl Fn(&str) -> bool,
) -> impl Fn(&str) -> IResult<&str, &str> {
    move |input| {
        let mut start = 0;
        for i in memchr_iter(b'\n', input.as_bytes()) {
            if predicate(input[start..i].trim()) {
                return Ok((&input[i + 1..], &input[0..start]));
            }
            start = i + 1;
        }

        if predicate(input[start..].trim()) {
            Ok(("", &input[0..start]))
        } else {
            Err(Err::Error(error_position!(input, ErrorKind::TakeTill1)))
        }
    }
}

pub(crate) fn take_one_word(input: &str) -> IResult<&str, &str> {
    alt((take_till(|c: char| c == ' ' || c == '\t'), |input| {
        Ok(("", input))
    }))(input)
}

pub(crate) fn skip_empty_lines(contents: &str) -> &str {
    let mut i = 0;
    for pos in memchr_iter(b'\n', contents.as_bytes()) {
        if contents.as_bytes()[i..pos]
            .iter()
            .all(u8::is_ascii_whitespace)
        {
            i = pos + 1;
        } else {
            break;
        }
    }
    &contents[i..]
}

#[test]
fn test_skip_empty_lines() {
    assert_eq!(skip_empty_lines("foo"), "foo");
    assert_eq!(skip_empty_lines(" foo"), " foo");
    assert_eq!(skip_empty_lines(" \nfoo\n"), "foo\n");
    assert_eq!(skip_empty_lines(" \n\n\nfoo\n"), "foo\n");
    assert_eq!(skip_empty_lines(" \n  \n\nfoo\n"), "foo\n");
    assert_eq!(skip_empty_lines(" \n  \n\n   foo\n"), "   foo\n");
}
