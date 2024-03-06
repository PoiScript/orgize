use nom::{
    branch::alt,
    bytes::complete::{tag, take_till, take_while1, take_while_m_n},
    character::complete::{digit1, space0, space1},
    combinator::{iterator, map, opt},
    sequence::tuple,
    IResult,
};

use super::{
    combinator::{
        colon_token, l_angle_token, l_bracket_token, l_parens_token, minus2_token, minus_token,
        node, percent2_token, r_angle_token, r_bracket_token, r_parens_token, GreenElement,
        NodeBuilder,
    },
    input::Input,
    SyntaxKind::*,
};

#[cfg_attr(
    feature = "tracing",
    tracing::instrument(level = "debug", skip(input), fields(input = input.s))
)]
pub fn timestamp_diary_node(input: Input) -> IResult<Input, GreenElement, ()> {
    let mut parser = map(
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
    );
    crate::lossless_parser!(parser, input)
}

fn date(i: Input) -> IResult<Input, [GreenElement; 5], ()> {
    map(
        tuple((
            take_while_m_n(4, 4, |c: char| c.is_ascii_digit()),
            minus_token,
            take_while_m_n(2, 2, |c: char| c.is_ascii_digit()),
            minus_token,
            take_while_m_n(2, 2, |c: char| c.is_ascii_digit()),
        )),
        |(year, minus, month, minus_, day)| {
            [
                year.token(TIMESTAMP_YEAR),
                minus,
                month.token(TIMESTAMP_MONTH),
                minus_,
                day.token(TIMESTAMP_DAY),
            ]
        },
    )(i)
}

fn dayname(i: Input) -> IResult<Input, GreenElement, ()> {
    map(
        take_while1(|c: char| {
            !c.is_ascii_whitespace()
                && !c.is_ascii_digit()
                && c != '+'
                && c != '-'
                && c != ']'
                && c != '>'
                && c != '.'
        }),
        |i: Input| i.token(TIMESTAMP_DAYNAME),
    )(i)
}

fn time(i: Input) -> IResult<Input, [GreenElement; 3], ()> {
    map(
        tuple((
            take_while_m_n(2, 2, |c: char| c.is_ascii_digit()),
            colon_token,
            take_while_m_n(2, 2, |c: char| c.is_ascii_digit()),
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

fn repeater_or_delay(
    input: Input,
) -> IResult<Input, (GreenElement, GreenElement, GreenElement), ()> {
    let (input, mark) = alt((
        map(alt((tag("++"), tag("+"), tag(".+"))), |i: Input| {
            i.token(TIMESTAMP_REPEATER_MARK)
        }),
        map(alt((tag("--"), tag("-"))), |i: Input| {
            i.token(TIMESTAMP_DELAY_MARK)
        }),
    ))(input)?;
    let (input, value) = digit1(input)?;
    let (input, unit) = alt((tag("h"), tag("d"), tag("w"), tag("m"), tag("y")))(input)?;

    Ok((
        input,
        (
            mark,
            value.token(TIMESTAMP_VALUE),
            unit.token(TIMESTAMP_UNIT),
        ),
    ))
}

fn timestamp_node_base(
    input: Input,
    l_parser: impl Fn(Input) -> IResult<Input, GreenElement, ()>,
    r_parser: impl Fn(Input) -> IResult<Input, GreenElement, ()>,
) -> IResult<Input, Vec<GreenElement>, ()> {
    let (input, l_angle) = l_parser(input)?;
    let (input, start_date) = date(input)?;
    let (input, start_dayname) = opt(tuple((space1, dayname)))(input)?;
    let (input, start_time) = opt(tuple((space1, time)))(input)?;

    let mut b = NodeBuilder::new();
    b.push(l_angle);
    b.children.extend(start_date);

    if let Some((ws, dayname)) = start_dayname {
        b.push(ws.ws_token());
        b.push(dayname);
    }

    if input.as_str().starts_with('-') {
        let (ws, start_time) = match start_time {
            Some(start_time) => start_time,
            None => return Err(nom::Err::Error(())),
        };

        let (input, minus) = minus_token(input)?;
        let (input, end_time) = time(input)?;

        b.ws(ws);
        b.children.extend(start_time);
        b.push(minus);
        b.children.extend(end_time);

        let mut iter = iterator(input, tuple((space1, repeater_or_delay)));
        for (ws, (mark, value, unit)) in &mut iter {
            b.children.extend([ws.ws_token(), mark, value, unit]);
        }
        let (input, _) = iter.finish()?;

        let (input, space) = space0(input)?;
        let (input, r_angle) = r_parser(input)?;

        b.ws(space);
        b.push(r_angle);

        return Ok((input, b.children));
    }

    if let Some((ws, start_time)) = start_time {
        b.ws(ws);
        b.children.extend(start_time);
    }

    let mut iter = iterator(input, tuple((space1, repeater_or_delay)));
    for (ws, (mark, value, unit)) in &mut iter {
        b.children.extend([ws.ws_token(), mark, value, unit]);
    }
    let (input, _) = iter.finish()?;

    let (input, space) = space0(input)?;
    let (input, r_angle) = r_parser(input)?;

    b.ws(space);
    b.push(r_angle);

    if input.as_str().starts_with("--") {
        let (input, minus2) = minus2_token(input)?;
        let (input, l_angle) = l_parser(input)?;
        let (input, end_date) = date(input)?;
        let (input, end_dayname) = opt(tuple((space1, dayname)))(input)?;
        let (input, end_time) = opt(tuple((space1, time)))(input)?;

        b.children.extend([minus2, l_angle]);
        b.children.extend(end_date);
        if let Some((ws, dayname)) = end_dayname {
            b.push(ws.ws_token());
            b.push(dayname);
        }
        if let Some((ws, end_time)) = end_time {
            b.ws(ws);
            b.children.extend(end_time);
        }
        let mut iter = iterator(input, tuple((space1, repeater_or_delay)));
        for (ws, (mark, value, unit)) in &mut iter {
            b.children.extend([ws.ws_token(), mark, value, unit]);
        }
        let (input, _) = iter.finish()?;

        let (input, space_) = space0(input)?;
        let (input, r_angle) = r_parser(input)?;

        b.ws(space_);
        b.push(r_angle);

        Ok((input, b.children))
    } else {
        Ok((input, b.children))
    }
}

#[cfg_attr(
    feature = "tracing",
    tracing::instrument(level = "debug", skip(input), fields(input = input.s))
)]
pub fn timestamp_active_node(input: Input) -> IResult<Input, GreenElement, ()> {
    fn parser(input: Input) -> IResult<Input, GreenElement, ()> {
        let (input, children) = timestamp_node_base(input, l_angle_token, r_angle_token)?;
        Ok((input, node(TIMESTAMP_ACTIVE, children)))
    }
    crate::lossless_parser!(parser, input)
}

#[cfg_attr(
    feature = "tracing",
    tracing::instrument(level = "debug", skip(input), fields(input = input.s))
)]
pub fn timestamp_inactive_node(input: Input) -> IResult<Input, GreenElement, ()> {
    fn parser(input: Input) -> IResult<Input, GreenElement, ()> {
        let (input, children) = timestamp_node_base(input, l_bracket_token, r_bracket_token)?;
        Ok((input, node(TIMESTAMP_INACTIVE, children)))
    }
    crate::lossless_parser!(parser, input)
}

#[test]
fn parse() {
    use crate::{ast::Timestamp, tests::to_ast};

    let to_timestamp = to_ast::<Timestamp>(timestamp_inactive_node);

    to_timestamp("[2003-09-16]");
    to_timestamp("[2003-09-16 09:09]");
    to_timestamp("[2003-09-16 Tue]");
    to_timestamp("[2003-09-16 Tue 09:09]");
    to_timestamp("[2003-09-16]--[2003-09-16]");
    to_timestamp("[2003-09-16 09:09]--[2003-09-16 09:09]");
    to_timestamp("[2003-09-16]--[2003-09-16 09:09]");
    to_timestamp("[2003-09-16 Tue]--[2003-09-16 Tue]");
    to_timestamp("[2003-09-16 Tue 09:09]--[2003-09-16 Tue 09:09]");
    to_timestamp("[2003-09-16 Tue 09:09-09:09]");
    to_timestamp("[2003-09-16 09:09-09:09 ]");
    to_timestamp("[2003-09-16 09:09 +1w .+1d]");
    to_timestamp("[2003-09-16 09:09]--[2003-09-16  +1w .+1d --1d ]");
    to_timestamp("[2003-09-16 Tue 09:09 +1w]--[2003-09-16 .+1d --1d ]");
    to_timestamp("[2003-09-16 09:09-10:19 +1w --1d]");

    let ts = to_timestamp("[2003-09-16 Tue +1w]");
    assert!(!ts.is_range());
    insta::assert_debug_snapshot!(
        ts.syntax,
        @r###"
    TIMESTAMP_INACTIVE@0..20
      L_BRACKET@0..1 "["
      TIMESTAMP_YEAR@1..5 "2003"
      MINUS@5..6 "-"
      TIMESTAMP_MONTH@6..8 "09"
      MINUS@8..9 "-"
      TIMESTAMP_DAY@9..11 "16"
      WHITESPACE@11..12 " "
      TIMESTAMP_DAYNAME@12..15 "Tue"
      WHITESPACE@15..16 " "
      TIMESTAMP_REPEATER_MARK@16..17 "+"
      TIMESTAMP_VALUE@17..18 "1"
      TIMESTAMP_UNIT@18..19 "w"
      R_BRACKET@19..20 "]"
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
