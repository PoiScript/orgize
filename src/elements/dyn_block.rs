use std::borrow::Cow;

use nom::{
    bytes::complete::tag_no_case,
    character::complete::{alpha1, space0, space1},
    error::ParseError,
    IResult,
};

use crate::parsers::{blank_lines, line, take_lines_while};

/// Dynamic Block Element
#[derive(Debug, Default)]
#[cfg_attr(test, derive(PartialEq))]
#[cfg_attr(feature = "ser", derive(serde::Serialize))]
pub struct DynBlock<'a> {
    /// Block name
    pub block_name: Cow<'a, str>,
    /// Block argument
    #[cfg_attr(feature = "ser", serde(skip_serializing_if = "Option::is_none"))]
    pub arguments: Option<Cow<'a, str>>,
    /// Numbers of blank lines between first block's line and next non-blank
    /// line
    pub pre_blank: usize,
    /// Numbers of blank lines between last drawer's line and next non-blank
    /// line or buffer's end
    pub post_blank: usize,
}

impl DynBlock<'_> {
    pub(crate) fn parse(input: &str) -> Option<(&str, (DynBlock, &str))> {
        parse_dyn_block::<()>(input).ok()
    }

    pub fn into_owned(self) -> DynBlock<'static> {
        DynBlock {
            block_name: self.block_name.into_owned().into(),
            arguments: self.arguments.map(Into::into).map(Cow::Owned),
            pre_blank: self.pre_blank,
            post_blank: self.post_blank,
        }
    }
}

#[inline]
fn parse_dyn_block<'a, E: ParseError<&'a str>>(
    input: &'a str,
) -> IResult<&str, (DynBlock, &str), E> {
    let (input, _) = space0(input)?;
    let (input, _) = tag_no_case("#+BEGIN:")(input)?;
    let (input, _) = space1(input)?;
    let (input, name) = alpha1(input)?;
    let (input, args) = line(input)?;
    let (input, contents) =
        take_lines_while(|line| !line.trim().eq_ignore_ascii_case("#+END:"))(input);
    let (contents, pre_blank) = blank_lines(contents);
    let (input, _) = line(input)?;
    let (input, post_blank) = blank_lines(input);

    Ok((
        input,
        (
            DynBlock {
                block_name: name.into(),
                arguments: if args.trim().is_empty() {
                    None
                } else {
                    Some(args.trim().into())
                },
                pre_blank,
                post_blank,
            },
            contents,
        ),
    ))
}

#[test]
fn parse() {
    use nom::error::VerboseError;

    // TODO: testing
    assert_eq!(
        parse_dyn_block::<VerboseError<&str>>(
            r#"#+BEGIN: clocktable :scope file


CONTENTS
#+END:

"#
        ),
        Ok((
            "",
            (
                DynBlock {
                    block_name: "clocktable".into(),
                    arguments: Some(":scope file".into()),
                    pre_blank: 2,
                    post_blank: 1,
                },
                "CONTENTS\n"
            )
        ))
    );
}
