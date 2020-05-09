//! Parsers combinators

use memchr::memchr;
use nom::{
    bytes::complete::take_while1,
    combinator::verify,
    error::{make_error, ErrorKind},
    Err, IResult,
};

// read until the first line_ending, if line_ending is not present, return the input directly
pub fn line(input: &str) -> IResult<&str, &str, ()> {
    if let Some(i) = memchr(b'\n', input.as_bytes()) {
        if i > 0 && input.as_bytes()[i - 1] == b'\r' {
            Ok((&input[i + 1..], &input[0..i - 1]))
        } else {
            Ok((&input[i + 1..], &input[0..i]))
        }
    } else {
        Ok(("", input))
    }
}

pub fn lines_till<F>(predicate: F) -> impl Fn(&str) -> IResult<&str, &str, ()>
where
    F: Fn(&str) -> bool,
{
    move |i| {
        let mut input = i;

        loop {
            // TODO: better error kind
            if input.is_empty() {
                return Err(Err::Error(make_error(input, ErrorKind::Many0)));
            }

            let (input_, line_) = line(input)?;

            debug_assert_ne!(input, input_);

            if predicate(line_) {
                let offset = i.len() - input.len();
                return Ok((input_, &i[0..offset]));
            }

            input = input_;
        }
    }
}

pub fn lines_while<F>(predicate: F) -> impl Fn(&str) -> IResult<&str, &str, ()>
where
    F: Fn(&str) -> bool,
{
    move |i| {
        let mut input = i;

        loop {
            // unlike lines_till, line_while won't return error
            if input.is_empty() {
                return Ok(("", i));
            }

            let (input_, line_) = line(input)?;

            debug_assert_ne!(input, input_);

            if !predicate(line_) {
                let offset = i.len() - input.len();
                return Ok((input, &i[0..offset]));
            }

            input = input_;
        }
    }
}

#[test]
fn test_lines_while() {
    assert_eq!(lines_while(|line| line == "foo")("foo"), Ok(("", "foo")));
    assert_eq!(lines_while(|line| line == "foo")("bar"), Ok(("bar", "")));
    assert_eq!(
        lines_while(|line| line == "foo")("foo\n\n"),
        Ok(("\n", "foo\n"))
    );
    assert_eq!(
        lines_while(|line| line.trim().is_empty())("\n\n\n"),
        Ok(("", "\n\n\n"))
    );
}

pub fn eol(input: &str) -> IResult<&str, &str, ()> {
    verify(line, |s: &str| {
        s.as_bytes().iter().all(u8::is_ascii_whitespace)
    })(input)
}

pub fn one_word(input: &str) -> IResult<&str, &str, ()> {
    take_while1(|c: char| !c.is_ascii_whitespace())(input)
}

pub fn blank_lines_count(input: &str) -> IResult<&str, usize, ()> {
    let mut count = 0;
    let mut input = input;

    loop {
        if input.is_empty() {
            return Ok(("", count));
        }

        let (input_, line_) = line(input)?;

        debug_assert_ne!(input, input_);

        if !line_.chars().all(char::is_whitespace) {
            return Ok((input, count));
        }

        count += 1;

        input = input_;
    }
}

#[test]
fn test_blank_lines_count() {
    assert_eq!(blank_lines_count("foo"), Ok(("foo", 0)));
    assert_eq!(blank_lines_count(" foo"), Ok((" foo", 0)));
    assert_eq!(blank_lines_count("  \t\nfoo\n"), Ok(("foo\n", 1)));
    assert_eq!(blank_lines_count("\n    \r\n\nfoo\n"), Ok(("foo\n", 3)));
    assert_eq!(
        blank_lines_count("\r\n   \n  \r\n   foo\n"),
        Ok(("   foo\n", 3))
    );
    assert_eq!(blank_lines_count("\r\n   \n  \r\n   \n"), Ok(("", 4)));
}
