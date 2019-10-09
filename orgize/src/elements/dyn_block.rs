use std::borrow::Cow;

use nom::{
    bytes::complete::tag_no_case,
    character::complete::{alpha1, space1},
    error::ParseError,
    IResult,
};

use crate::parsers::{line, take_lines_while};

/// Dynamic Block Element
#[cfg_attr(test, derive(PartialEq))]
#[cfg_attr(feature = "ser", derive(serde::Serialize))]
#[derive(Debug)]
pub struct DynBlock<'a> {
    /// Block name
    pub block_name: Cow<'a, str>,
    /// Block argument
    #[cfg_attr(feature = "ser", serde(skip_serializing_if = "Option::is_none"))]
    pub arguments: Option<Cow<'a, str>>,
}

impl DynBlock<'_> {
    pub(crate) fn parse(input: &str) -> Option<(&str, (DynBlock, &str))> {
        parse_dyn_block::<()>(input).ok()
    }

    pub fn into_owned(self) -> DynBlock<'static> {
        DynBlock {
            block_name: self.block_name.into_owned().into(),
            arguments: self.arguments.map(Into::into).map(Cow::Owned),
        }
    }
}

#[inline]
fn parse_dyn_block<'a, E: ParseError<&'a str>>(
    input: &'a str,
) -> IResult<&str, (DynBlock, &str), E> {
    let (input, _) = tag_no_case("#+BEGIN:")(input)?;
    let (input, _) = space1(input)?;
    let (input, name) = alpha1(input)?;
    let (input, args) = line(input)?;
    let (input, contents) =
        take_lines_while(|line| !line.trim().eq_ignore_ascii_case("#+END:"))(input);
    let (input, _) = line(input)?;

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
                },
                "CONTENTS\n"
            )
        ))
    );
}
