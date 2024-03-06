use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, space0},
    combinator::{map, opt, recognize},
    sequence::tuple,
    IResult,
};

use super::{
    combinator::{
        blank_lines, colon_token, double_arrow_token, eol_or_eof, GreenElement, NodeBuilder,
    },
    input::Input,
    timestamp::{timestamp_active_node, timestamp_inactive_node},
    SyntaxKind,
};

#[cfg_attr(
    feature = "tracing",
    tracing::instrument(level = "debug", skip(input), fields(input = input.s))
)]
pub fn clock_node(input: Input) -> IResult<Input, GreenElement, ()> {
    let mut parser = map(
        tuple((
            space0,
            tag("CLOCK:"),
            space0,
            alt((timestamp_inactive_node, timestamp_active_node)),
            opt(tuple((
                space0,
                double_arrow_token,
                space0,
                recognize(tuple((digit1, colon_token, digit1))),
            ))),
            space0,
            eol_or_eof,
            blank_lines,
        )),
        |(ws, clock, ws_, timestamp, duration, ws__, nl, post_blank)| {
            let mut b = NodeBuilder::new();

            b.ws(ws);
            b.text(clock);
            b.ws(ws_);
            b.push(timestamp);
            if let Some((ws, double_arrow, ws_, time)) = duration {
                b.ws(ws);
                b.push(double_arrow);
                b.ws(ws_);
                b.text(time);
            }
            b.ws(ws__);
            b.nl(nl);
            b.children.extend(post_blank);
            b.finish(SyntaxKind::CLOCK)
        },
    );
    crate::lossless_parser!(parser, input)
}

#[test]
fn parse() {
    use crate::ast::Clock;
    use crate::tests::to_ast;

    let to_clock = to_ast::<Clock>(clock_node);

    insta::assert_debug_snapshot!(
      to_clock("CLOCK: [2003-09-16 Tue 09:39]").syntax,
      @r###"
    CLOCK@0..29
      TEXT@0..6 "CLOCK:"
      WHITESPACE@6..7 " "
      TIMESTAMP_INACTIVE@7..29
        L_BRACKET@7..8 "["
        TIMESTAMP_YEAR@8..12 "2003"
        MINUS@12..13 "-"
        TIMESTAMP_MONTH@13..15 "09"
        MINUS@15..16 "-"
        TIMESTAMP_DAY@16..18 "16"
        WHITESPACE@18..19 " "
        TIMESTAMP_DAYNAME@19..22 "Tue"
        WHITESPACE@22..23 " "
        TIMESTAMP_HOUR@23..25 "09"
        COLON@25..26 ":"
        TIMESTAMP_MINUTE@26..28 "39"
        R_BRACKET@28..29 "]"
    "###
    );

    insta::assert_debug_snapshot!(
      to_clock("CLOCK: [2003-09-16 Tue 09:39]--[2003-09-16 Tue 10:39] =>  1:00\n\n").syntax,
      @r###"
    CLOCK@0..64
      TEXT@0..6 "CLOCK:"
      WHITESPACE@6..7 " "
      TIMESTAMP_INACTIVE@7..53
        L_BRACKET@7..8 "["
        TIMESTAMP_YEAR@8..12 "2003"
        MINUS@12..13 "-"
        TIMESTAMP_MONTH@13..15 "09"
        MINUS@15..16 "-"
        TIMESTAMP_DAY@16..18 "16"
        WHITESPACE@18..19 " "
        TIMESTAMP_DAYNAME@19..22 "Tue"
        WHITESPACE@22..23 " "
        TIMESTAMP_HOUR@23..25 "09"
        COLON@25..26 ":"
        TIMESTAMP_MINUTE@26..28 "39"
        R_BRACKET@28..29 "]"
        MINUS2@29..31 "--"
        L_BRACKET@31..32 "["
        TIMESTAMP_YEAR@32..36 "2003"
        MINUS@36..37 "-"
        TIMESTAMP_MONTH@37..39 "09"
        MINUS@39..40 "-"
        TIMESTAMP_DAY@40..42 "16"
        WHITESPACE@42..43 " "
        TIMESTAMP_DAYNAME@43..46 "Tue"
        WHITESPACE@46..47 " "
        TIMESTAMP_HOUR@47..49 "10"
        COLON@49..50 ":"
        TIMESTAMP_MINUTE@50..52 "39"
        R_BRACKET@52..53 "]"
      WHITESPACE@53..54 " "
      DOUBLE_ARROW@54..56 "=>"
      WHITESPACE@56..58 "  "
      TEXT@58..62 "1:00"
      NEW_LINE@62..63 "\n"
      BLANK_LINE@63..64 "\n"
    "###
    );
}
