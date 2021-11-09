use nom::{
    bytes::complete::{take, take_till, take_while},
    character::complete::{space0, space1},
    combinator::{map, opt, verify},
    sequence::tuple,
    IResult,
};

use super::{
    combinator::{
        colon_token, debug_assert_lossless, l_angle_token, l_bracket_token, l_parens_token,
        minus2_token, minus_token, node, percent2_token, r_angle_token, r_bracket_token,
        r_parens_token, GreenElement, NodeBuilder,
    },
    input::Input,
    SyntaxKind::*,
};

pub fn timestamp_diary_node(input: Input) -> IResult<Input, GreenElement, ()> {
    debug_assert_lossless(map(
        tuple((
            l_angle_token,
            percent2_token,
            l_parens_token,
            take_till(|c| c == ')' || c == '>' || c == '\n'),
            r_parens_token,
            r_angle_token,
        )),
        |(l_angle, percent2, l_paren, value, r_paren, r_angle)| {
            node(
                TIMESTAMP_DIARY,
                [
                    l_angle,
                    percent2,
                    l_paren,
                    value.text_token(),
                    r_paren,
                    r_angle,
                ],
            )
        },
    ))(input)
}

fn is_digit_str(s: &Input) -> bool {
    s.as_str().bytes().all(|u| u.is_ascii_digit())
}

fn date(i: Input) -> IResult<Input, [GreenElement; 7], ()> {
    map(
        tuple((
            verify(take(4usize), is_digit_str),
            minus_token,
            verify(take(2usize), is_digit_str),
            minus_token,
            verify(take(2usize), is_digit_str),
            space1,
            take_while(|c: char| {
                !c.is_ascii_whitespace()
                    && !c.is_ascii_digit()
                    && c != '+'
                    && c != '-'
                    && c != ']'
                    && c != '>'
            }),
        )),
        |(year, minus, month, minus_, day, ws, dayname)| {
            [
                year.token(TIMESTAMP_YEAR),
                minus,
                month.token(TIMESTAMP_MONTH),
                minus_,
                day.token(TIMESTAMP_DAY),
                ws.ws_token(),
                dayname.token(TIMESTAMP_DAYNAME),
            ]
        },
    )(i)
}

fn time(i: Input) -> IResult<Input, [GreenElement; 3], ()> {
    map(
        tuple((
            verify(take(2usize), is_digit_str),
            colon_token,
            verify(take(2usize), is_digit_str),
        )),
        |(hour, colon, minute)| {
            [
                hour.token(TIMESTAMP_HOUR),
                colon,
                minute.token(TIMESTAMP_MINUTE),
            ]
        },
    )(i)
}

fn timestamp_active_node_base(input: Input) -> IResult<Input, GreenElement, ()> {
    let (input, l_angle) = l_angle_token(input)?;
    let (input, start_date) = date(input)?;
    let (input, start_time) = opt(tuple((space1, time)))(input)?;

    let mut b = NodeBuilder::new();
    b.push(l_angle);
    b.children.extend(start_date);

    if input.as_str().starts_with('-') {
        let (ws, start_time) = match start_time {
            Some(start_time) => start_time,
            None => return Err(nom::Err::Error(())),
        };

        let (input, minus) = minus_token(input)?;
        let (input, end_time) = time(input)?;
        let (input, space) = space0(input)?;
        // TODO: delay-or-repeater
        let (input, r_angle) = r_angle_token(input)?;

        b.ws(ws);
        b.children.extend(start_time);
        b.push(minus);
        b.children.extend(end_time);
        b.ws(space);
        b.push(r_angle);

        return Ok((input, b.finish(TIMESTAMP_ACTIVE)));
    }

    let (input, space) = space0(input)?;
    let (input, r_angle) = r_angle_token(input)?;

    if let Some((ws, start_time)) = start_time {
        b.ws(ws);
        b.children.extend(start_time);
    }

    b.ws(space);
    b.push(r_angle);

    if input.as_str().starts_with("--<") {
        let (input, minus2) = minus2_token(input)?;
        let (input, l_angle) = l_angle_token(input)?;
        let (input, end_date) = date(input)?;
        let (input, end_time) = opt(tuple((space1, time)))(input)?;
        let (input, space_) = space0(input)?;
        // TODO: delay-or-repeater
        let (input, r_angle) = r_angle_token(input)?;

        b.children.extend([minus2, l_angle]);
        b.children.extend(end_date);

        if let Some((ws, end_time)) = end_time {
            b.ws(ws);
            b.children.extend(end_time);
        }

        b.ws(space_);
        b.push(r_angle);

        Ok((input, b.finish(TIMESTAMP_ACTIVE)))
    } else {
        Ok((input, b.finish(TIMESTAMP_ACTIVE)))
    }
}

fn timestamp_inactive_node_base(input: Input) -> IResult<Input, GreenElement, ()> {
    let (input, l_bracket) = l_bracket_token(input)?;
    let (input, start_date) = date(input)?;
    let (input, start_time) = opt(tuple((space1, time)))(input)?;

    let mut b = NodeBuilder::new();
    b.push(l_bracket);
    b.children.extend(start_date);

    if input.s.starts_with('-') {
        let (ws, start_time) = match start_time {
            Some(start_time) => start_time,
            None => return Err(nom::Err::Error(())),
        };

        let (input, minus) = minus_token(input)?;
        let (input, end_time) = time(input)?;
        let (input, space) = space0(input)?;
        // TODO: delay-or-repeater
        let (input, r_bracket) = r_bracket_token(input)?;

        b.ws(ws);
        b.children.extend(start_time);
        b.push(minus);
        b.children.extend(end_time);
        b.ws(space);
        b.push(r_bracket);

        return Ok((input, b.finish(TIMESTAMP_INACTIVE)));
    }

    let (input, space) = space0(input)?;
    let (input, r_bracket) = r_bracket_token(input)?;

    if let Some((ws, start_time)) = start_time {
        b.ws(ws);
        b.children.extend(start_time);
    }

    b.ws(space);
    b.push(r_bracket);

    if input.s.starts_with("--[") {
        let (input, minus2) = minus2_token(input)?;
        let (input, l_bracket) = l_bracket_token(input)?;
        let (input, end_date) = date(input)?;
        let (input, end_time) = opt(tuple((space1, time)))(input)?;
        let (input, space_) = space0(input)?;
        // TODO: delay-or-repeater
        let (input, r_bracket) = r_bracket_token(input)?;

        b.children.extend([minus2, l_bracket]);
        b.children.extend(end_date);

        if let Some((ws, end_time)) = end_time {
            b.ws(ws);
            b.children.extend(end_time);
        }

        b.ws(space_);
        b.push(r_bracket);

        Ok((input, b.finish(TIMESTAMP_INACTIVE)))
    } else {
        Ok((input, b.finish(TIMESTAMP_INACTIVE)))
    }
}

pub fn timestamp_active_node(input: Input) -> IResult<Input, GreenElement, ()> {
    debug_assert_lossless(timestamp_active_node_base)(input)
}
pub fn timestamp_inactive_node(input: Input) -> IResult<Input, GreenElement, ()> {
    debug_assert_lossless(timestamp_inactive_node_base)(input)
}

#[test]
fn parse() {
    use crate::{ast::Timestamp, tests::to_ast};

    let to_timestamp = to_ast::<Timestamp>(timestamp_inactive_node);

    let ts = to_timestamp("[2003-09-16 Tue]");
    assert!(!ts.is_range());
    insta::assert_debug_snapshot!(
        ts.syntax,
        @r###"
    TIMESTAMP_INACTIVE@0..16
      L_BRACKET@0..1 "["
      TIMESTAMP_YEAR@1..5 "2003"
      MINUS@5..6 "-"
      TIMESTAMP_MONTH@6..8 "09"
      MINUS@8..9 "-"
      TIMESTAMP_DAY@9..11 "16"
      WHITESPACE@11..12 " "
      TIMESTAMP_DAYNAME@12..15 "Tue"
      R_BRACKET@15..16 "]"
    "###
    );

    let ts = to_timestamp("[2003-09-16 Tue 09:39]--[2003-09-16 Tue 10:39]");
    assert!(ts.is_range());
    insta::assert_debug_snapshot!(
        ts.syntax,
        @r###"
    TIMESTAMP_INACTIVE@0..46
      L_BRACKET@0..1 "["
      TIMESTAMP_YEAR@1..5 "2003"
      MINUS@5..6 "-"
      TIMESTAMP_MONTH@6..8 "09"
      MINUS@8..9 "-"
      TIMESTAMP_DAY@9..11 "16"
      WHITESPACE@11..12 " "
      TIMESTAMP_DAYNAME@12..15 "Tue"
      WHITESPACE@15..16 " "
      TIMESTAMP_HOUR@16..18 "09"
      COLON@18..19 ":"
      TIMESTAMP_MINUTE@19..21 "39"
      R_BRACKET@21..22 "]"
      MINUS2@22..24 "--"
      L_BRACKET@24..25 "["
      TIMESTAMP_YEAR@25..29 "2003"
      MINUS@29..30 "-"
      TIMESTAMP_MONTH@30..32 "09"
      MINUS@32..33 "-"
      TIMESTAMP_DAY@33..35 "16"
      WHITESPACE@35..36 " "
      TIMESTAMP_DAYNAME@36..39 "Tue"
      WHITESPACE@39..40 " "
      TIMESTAMP_HOUR@40..42 "10"
      COLON@42..43 ":"
      TIMESTAMP_MINUTE@43..45 "39"
      R_BRACKET@45..46 "]"
    "###
    );

    let ts = to_timestamp("[2003-09-16 Tue 09:39-10:39]");
    assert!(ts.is_range());
    insta::assert_debug_snapshot!(
        ts.syntax,
        @r###"
    TIMESTAMP_INACTIVE@0..28
      L_BRACKET@0..1 "["
      TIMESTAMP_YEAR@1..5 "2003"
      MINUS@5..6 "-"
      TIMESTAMP_MONTH@6..8 "09"
      MINUS@8..9 "-"
      TIMESTAMP_DAY@9..11 "16"
      WHITESPACE@11..12 " "
      TIMESTAMP_DAYNAME@12..15 "Tue"
      WHITESPACE@15..16 " "
      TIMESTAMP_HOUR@16..18 "09"
      COLON@18..19 ":"
      TIMESTAMP_MINUTE@19..21 "39"
      MINUS@21..22 "-"
      TIMESTAMP_HOUR@22..24 "10"
      COLON@24..25 ":"
      TIMESTAMP_MINUTE@25..27 "39"
      R_BRACKET@27..28 "]"
    "###
    );
}
