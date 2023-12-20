use nom::{
    branch::alt, bytes::complete::tag, character::complete::space0, combinator::iterator,
    sequence::tuple, IResult,
};

use super::{
    combinator::{eol_or_eof, GreenElement, NodeBuilder},
    input::Input,
    timestamp::{timestamp_active_node, timestamp_inactive_node},
    SyntaxKind::*,
};

pub fn planning_node(input: Input) -> IResult<Input, GreenElement, ()> {
    debug_assert!(!input.is_empty());
    crate::lossless_parser!(planning_node_base, input)
}

fn planning_node_base(input: Input) -> IResult<Input, GreenElement, ()> {
    let mut b = NodeBuilder::new();

    let mut it = iterator(
        input,
        tuple((
            space0,
            alt((tag("DEADLINE:"), tag("SCHEDULED:"), tag("CLOSED:"))),
            space0,
            alt((timestamp_active_node, timestamp_inactive_node)),
        )),
    );

    let start_len = b.len();

    it.for_each(|(ws, text, ws_, timestamp)| {
        let mut b_ = NodeBuilder::new();
        b_.ws(ws);
        b_.text(text);
        b_.ws(ws_);
        b_.push(timestamp);
        b.push(b_.finish(match text.as_str() {
            "DEADLINE:" => PLANNING_DEADLINE,
            "SCHEDULED:" => PLANNING_SCHEDULED,
            "CLOSED:" => PLANNING_CLOSED,
            _ => unreachable!(),
        }));
    });

    if b.len() == start_len {
        return Err(nom::Err::Error(()));
    }

    let (input, _) = it.finish()?;
    let (input, ws) = space0(input)?;
    let (input, nl) = eol_or_eof(input)?;

    b.ws(ws);
    b.nl(nl);

    Ok((input, b.finish(PLANNING)))
}

#[test]
fn prase() {
    use crate::{ast::Planning, tests::to_ast, ParseConfig};

    let to_planning = to_ast::<Planning>(planning_node);

    insta::assert_debug_snapshot!(
        to_planning("SCHEDULED: <2019-04-08 Mon>").syntax,
        @r###"
    PLANNING@0..27
      PLANNING_SCHEDULED@0..27
        TEXT@0..10 "SCHEDULED:"
        WHITESPACE@10..11 " "
        TIMESTAMP_ACTIVE@11..27
          L_ANGLE@11..12 "<"
          TIMESTAMP_YEAR@12..16 "2019"
          MINUS@16..17 "-"
          TIMESTAMP_MONTH@17..19 "04"
          MINUS@19..20 "-"
          TIMESTAMP_DAY@20..22 "08"
          WHITESPACE@22..23 " "
          TIMESTAMP_DAYNAME@23..26 "Mon"
          R_ANGLE@26..27 ">"
    "###
    );

    let config = &ParseConfig::default();

    assert!(planning_node(("   ", config).into()).is_err());
    assert!(planning_node((" SCHEDULED:   ", config).into()).is_err());
}
