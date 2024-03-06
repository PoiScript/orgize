use memchr::{memchr2, memchr2_iter, Memchr2};
use nom::{bytes::complete::tag, IResult, InputTake, Slice};
use rowan::{GreenNode, GreenToken, Language, NodeOrToken};
use std::iter::once;

use super::{input::Input, OrgLanguage, SyntaxKind, SyntaxKind::*};

pub type GreenElement = NodeOrToken<GreenNode, GreenToken>;

#[inline]
pub fn token(kind: SyntaxKind, input: &str) -> GreenElement {
    GreenElement::Token(GreenToken::new(OrgLanguage::kind_to_raw(kind), input))
}

#[inline]
pub fn node<I>(kind: SyntaxKind, children: I) -> GreenElement
where
    I: IntoIterator<Item = GreenElement>,
    I::IntoIter: ExactSizeIterator,
{
    GreenElement::Node(GreenNode::new(OrgLanguage::kind_to_raw(kind), children))
}

macro_rules! token_parser {
    ($name:ident, $token:literal, $kind:ident) => {
        #[doc = "Recognizes `"]
        #[doc = $token]
        #[doc = "` and returns GreenToken"]
        pub fn $name(input: Input) -> IResult<Input, GreenElement, ()> {
            let (i, o) = tag($token)(input)?;
            Ok((i, token($kind, o.as_str())))
        }
    };
}

token_parser!(l_bracket_token, "[", L_BRACKET);
token_parser!(r_bracket_token, "]", R_BRACKET);
token_parser!(l_bracket2_token, "[[", L_BRACKET2);
token_parser!(r_bracket2_token, "]]", R_BRACKET2);
token_parser!(l_parens_token, "(", L_PARENS);
token_parser!(r_parens_token, ")", R_PARENS);
token_parser!(l_angle_token, "<", L_ANGLE);
token_parser!(r_angle_token, ">", R_ANGLE);
token_parser!(l_curly_token, "{", L_CURLY);
token_parser!(r_curly_token, "}", R_CURLY);
token_parser!(l_curly3_token, "{{{", L_CURLY3);
token_parser!(r_curly3_token, "}}}", R_CURLY3);
token_parser!(l_angle2_token, "<<", L_ANGLE2);
token_parser!(r_angle2_token, ">>", R_ANGLE2);
token_parser!(l_angle3_token, "<<<", L_ANGLE3);
token_parser!(r_angle3_token, ">>>", R_ANGLE3);
token_parser!(at_token, "@", AT);
token_parser!(at2_token, "@@", AT2);
token_parser!(minus2_token, "--", MINUS2);
// token_parser!(percent_token, "%", PERCENT);
token_parser!(percent2_token, "%%", PERCENT2);
// token_parser!(slash_token, "/", SLASH);
token_parser!(backslash_token, "\\", BACKSLASH);
token_parser!(underscore_token, "_", UNDERSCORE);
// token_parser!(star_token, "*", STAR);
token_parser!(plus_token, "+", PLUS);
token_parser!(minus_token, "-", MINUS);
token_parser!(colon_token, ":", COLON);
token_parser!(colon2_token, "::", COLON2);
token_parser!(pipe_token, "|", PIPE);
token_parser!(dollar_token, "$", DOLLAR);
token_parser!(dollar2_token, "$$", DOLLAR2);
// token_parser!(equal_token, "=", EQUAL);
// token_parser!(tilde_token, "~", TILDE);
token_parser!(hash_plus_token, "#+", HASH_PLUS);
token_parser!(caret_token, "^", CARET);
token_parser!(hash_token, "#", HASH);
token_parser!(double_arrow_token, "=>", DOUBLE_ARROW);

macro_rules! lossless_parser {
    ($parser:expr, $input:expr) => {{
        let i_ = $input;
        let (i, o) = $parser($input)?;
        cfg_if::cfg_if! {
            if #[cfg(feature = "tracing")] {
                tracing::trace!(consumed = o.to_string());
            }
        }
        debug_assert_eq!(
            &i_.as_str()[0..(i_.len() - i.len())],
            &o.to_string(),
            stringify!("parser must be lossless")
        );
        Ok((i, o))
    }};
}

pub(crate) use lossless_parser;

/// Takes all blank lines
pub fn blank_lines(input: Input) -> IResult<Input, Vec<GreenElement>, ()> {
    if input.is_empty() {
        return Ok((input, vec![]));
    }

    let mut lines = vec![];
    let mut start = 0;
    let bytes = input.as_bytes();

    for index in line_ends_iter(input.as_str()) {
        if start != index && bytes[start..index].iter().all(|b| b.is_ascii_whitespace()) {
            lines.push(token(BLANK_LINE, &input.as_str()[start..index]));
            start = index;
        } else {
            break;
        }
    }

    Ok((input.slice(start..), lines))
}

#[test]
fn test_blank_lines() {
    use crate::config::ParseConfig;
    let config = &ParseConfig::default();
    let (input, output) = blank_lines(("", config).into()).unwrap();
    assert_eq!(input.as_str(), "");
    assert_eq!(output, vec![]);

    let (input, output) = blank_lines(("\n", config).into()).unwrap();
    assert_eq!(input.as_str(), "");
    assert_eq!(output.len(), 1);
    assert_eq!(output[0].to_string(), "\n");

    let (input, output) = blank_lines(("    t", config).into()).unwrap();
    assert_eq!(input.as_str(), "    t");
    assert_eq!(output, vec![]);

    let (input, output) = blank_lines(("  \r\n\n\t\t\r\n  \n  ", config).into()).unwrap();
    assert_eq!(input.as_str(), "");
    assert_eq!(output.len(), 5);
    assert_eq!(output[0].to_string(), "  \r\n");
    assert_eq!(output[1].to_string(), "\n");
    assert_eq!(output[2].to_string(), "\t\t\r\n");
    assert_eq!(output[3].to_string(), "  \n");
    assert_eq!(output[4].to_string(), "  ");

    let (input, output) =
        blank_lines(("\r\n\n\t\t\r\n  \n\r   \r   t\n  ", config).into()).unwrap();
    assert_eq!(input.as_str(), "   t\n  ");
    assert_eq!(output.len(), 6);
    assert_eq!(output[0].to_string(), "\r\n");
    assert_eq!(output[1].to_string(), "\n");
    assert_eq!(output[2].to_string(), "\t\t\r\n");
    assert_eq!(output[3].to_string(), "  \n");
    assert_eq!(output[4].to_string(), "\r");
    assert_eq!(output[5].to_string(), "   \r");
}

/// Returns 1. anything before trailing whitespace, 2. whitespace itself, 3. line feeding
pub fn trim_line_end(input: Input) -> IResult<Input, (Input, Input, Input), ()> {
    let bytes = input.as_bytes();

    let (input, contents, nl) = match memchr2(b'\r', b'\n', bytes) {
        Some(i) if bytes[i] == b'\r' && matches!(bytes.get(i + 1), Some(b'\n')) => (
            input.slice(i + 2..),
            input.slice(0..i),
            input.slice(i..i + 2),
        ),
        Some(i) => (
            input.slice(i + 1..),
            input.slice(0..i),
            input.slice(i..i + 1),
        ),
        _ => (input.of(""), input, input.of("")),
    };

    let (contents, ws) = match contents.bytes().rposition(|u| !u.is_ascii_whitespace()) {
        Some(i) => (contents.slice(0..i + 1), contents.slice(i + 1..)),
        None => (contents.of(""), contents),
    };

    Ok((input, (contents, ws, nl)))
}

#[test]
fn test_trim_line_end() {
    use crate::config::ParseConfig;
    let config = &ParseConfig::default();
    let (input, output) = trim_line_end(("", config).into()).unwrap();
    assert_eq!(input.as_str(), "");
    assert_eq!(output.0.as_str(), "");
    assert_eq!(output.1.as_str(), "");
    assert_eq!(output.2.as_str(), "");

    let (input, output) = trim_line_end(("* hello, world :abc:", config).into()).unwrap();
    assert_eq!(input.as_str(), "");
    assert_eq!(output.0.as_str(), "* hello, world :abc:");
    assert_eq!(output.1.as_str(), "");
    assert_eq!(output.2.as_str(), "");

    let (input, output) =
        trim_line_end(("* hello, world :abc:  \r\nrest\n", config).into()).unwrap();
    assert_eq!(input.as_str(), "rest\n");
    assert_eq!(output.0.as_str(), "* hello, world :abc:");
    assert_eq!(output.1.as_str(), "  ");
    assert_eq!(output.2.as_str(), "\r\n");

    let (input, output) = trim_line_end((" \rr", config).into()).unwrap();
    assert_eq!(input.as_str(), "r");
    assert_eq!(output.0.as_str(), "");
    assert_eq!(output.1.as_str(), " ");
    assert_eq!(output.2.as_str(), "\r");
}

/// Recognizes a line ending \r, \n, \r\n or end of file
pub fn eol_or_eof(input: Input) -> IResult<Input, Input, ()> {
    let mut bytes = input.bytes();

    let count = match bytes.next() {
        Some(b'\n') => 1,
        Some(b'\r') => {
            if matches!(bytes.next(), Some(b'\n')) {
                2
            } else {
                1
            }
        }
        None => 0,
        _ => return Err(nom::Err::Error(())),
    };

    Ok(input.take_split(count))
}

struct LineStart<'a> {
    bytes: &'a [u8],
    iter: Memchr2<'a>,
}

impl<'a> LineStart<'a> {
    fn new(input: &'a str) -> Self {
        let bytes = input.as_bytes();
        LineStart {
            bytes,
            iter: memchr2_iter(b'\r', b'\n', bytes),
        }
    }
}

impl<'a> Iterator for LineStart<'a> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let i = self.iter.next()?;
        if self.bytes[i] == b'\r' && self.bytes.get(i + 1) == Some(&b'\n') {
            let ii = self.iter.next();
            debug_assert_eq!(i + 1, ii.unwrap());
            Some(i + 2)
        } else {
            Some(i + 1)
        }
    }
}

/// Returns an iterator of positions of line start, including zero
pub fn line_starts_iter(s: &str) -> impl Iterator<Item = usize> + '_ {
    once(0).chain(LineStart::new(s))
}

/// Returns an iterator of positions of line end, including eof
pub fn line_ends_iter(s: &str) -> impl Iterator<Item = usize> + '_ {
    LineStart::new(s).chain(once(s.len()))
}

pub struct NodeBuilder {
    pub children: Vec<GreenElement>,
}

impl NodeBuilder {
    pub fn new() -> NodeBuilder {
        NodeBuilder { children: vec![] }
    }

    pub fn ws(&mut self, i: Input) {
        if !i.is_empty() {
            debug_assert!(i.bytes().all(|c| c.is_ascii_whitespace()));
            self.children.push(i.ws_token())
        }
    }

    pub fn nl(&mut self, i: Input) {
        if !i.is_empty() {
            debug_assert!(
                i.s == "\n" || i.s == "\r\n" || i.s == "\r",
                "{:?} should be a new line",
                i.s
            );
            self.children.push(i.nl_token())
        }
    }

    pub fn text(&mut self, i: Input) {
        if !i.is_empty() {
            self.children.push(i.text_token())
        }
    }

    pub fn token(&mut self, kind: SyntaxKind, i: Input) {
        self.children.push(i.token(kind))
    }

    pub fn push(&mut self, elem: GreenElement) {
        self.children.push(elem)
    }

    pub fn push_opt(&mut self, elem: Option<GreenElement>) {
        if let Some(elem) = elem {
            self.children.push(elem)
        }
    }

    pub fn len(&self) -> usize {
        self.children.len()
    }

    pub fn finish(self, kind: SyntaxKind) -> GreenElement {
        GreenElement::Node(GreenNode::new(kind.into(), self.children))
    }
}
