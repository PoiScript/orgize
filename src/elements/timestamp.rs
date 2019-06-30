#[cfg(feature = "chrono")]
use chrono::*;
use nom::{
    bytes::complete::{tag, take, take_till, take_while, take_while_m_n},
    character::complete::{space0, space1},
    combinator::{map_res, opt},
    IResult,
};

/// Date
///
/// # Syntax
///
/// ```text
/// YYYY-MM-DD DAYNAME
/// ```
///
#[cfg_attr(test, derive(PartialEq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Debug, Clone, Copy)]
pub struct Date<'a> {
    pub year: u16,
    pub month: u8,
    pub day: u8,
    pub dayname: &'a str,
}

impl Date<'_> {
    fn parse(input: &str) -> IResult<&str, Date<'_>> {
        let (input, year) = map_res(take(4usize), |num| u16::from_str_radix(num, 10))(input)?;
        let (input, _) = tag("-")(input)?;
        let (input, month) = map_res(take(2usize), |num| u8::from_str_radix(num, 10))(input)?;
        let (input, _) = tag("-")(input)?;
        let (input, day) = map_res(take(2usize), |num| u8::from_str_radix(num, 10))(input)?;
        let (input, _) = space1(input)?;
        let (input, dayname) = take_while(|c: char| {
            !c.is_ascii_whitespace()
                && !c.is_ascii_digit()
                && c != '+'
                && c != '-'
                && c != ']'
                && c != '>'
        })(input)?;

        Ok((
            input,
            Date {
                year,
                month,
                day,
                dayname,
            },
        ))
    }
}

#[cfg_attr(test, derive(PartialEq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Debug, Clone, Copy)]
pub struct Time {
    pub hour: u8,
    pub minute: u8,
}

impl Time {
    fn parse(input: &str) -> IResult<&str, Time> {
        let (input, hour) = map_res(take_while_m_n(1, 2, |c: char| c.is_ascii_digit()), |num| {
            u8::from_str_radix(num, 10)
        })(input)?;
        let (input, _) = tag(":")(input)?;
        let (input, minute) = map_res(take(2usize), |num| u8::from_str_radix(num, 10))(input)?;

        Ok((input, Time { hour, minute }))
    }
}

#[cfg_attr(test, derive(PartialEq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Debug)]
pub enum Timestamp<'a> {
    Active {
        start_date: Date<'a>,
        start_time: Option<Time>,
        repeater: Option<&'a str>,
        delay: Option<&'a str>,
    },
    Inactive {
        start_date: Date<'a>,
        start_time: Option<Time>,
        repeater: Option<&'a str>,
        delay: Option<&'a str>,
    },
    ActiveRange {
        start_date: Date<'a>,
        start_time: Option<Time>,
        end_date: Date<'a>,
        end_time: Option<Time>,
        repeater: Option<&'a str>,
        delay: Option<&'a str>,
    },
    InactiveRange {
        start_date: Date<'a>,
        start_time: Option<Time>,
        end_date: Date<'a>,
        end_time: Option<Time>,
        repeater: Option<&'a str>,
        delay: Option<&'a str>,
    },
    Diary(&'a str),
}

impl Timestamp<'_> {
    pub(crate) fn parse_active(input: &str) -> IResult<&str, Timestamp<'_>> {
        let (input, _) = tag("<")(input)?;
        let (input, start_date) = Date::parse(input)?;
        let (input, _) = space0(input)?;
        let (input, start_time) = opt(Time::parse)(input)?;

        if input.starts_with('-') {
            let (input, end_time) = opt(Time::parse)(&input[1..])?;
            let (input, _) = space0(input)?;
            // TODO: delay-or-repeater
            let (input, _) = tag(">")(input)?;
            return Ok((
                input,
                Timestamp::ActiveRange {
                    start_date,
                    start_time,
                    end_date: start_date,
                    end_time,
                    repeater: None,
                    delay: None,
                },
            ));
        }

        let (input, _) = space0(input)?;
        // TODO: delay-or-repeater
        let (input, _) = tag(">")(input)?;

        if input.starts_with("--<") {
            let (input, end_date) = Date::parse(&input["--<".len()..])?;
            let (input, _) = space0(input)?;
            let (input, end_time) = opt(Time::parse)(input)?;
            let (input, _) = space0(input)?;
            // TODO: delay-or-repeater
            let (input, _) = tag(">")(input)?;
            Ok((
                input,
                Timestamp::ActiveRange {
                    start_date,
                    start_time,
                    end_date,
                    end_time,
                    repeater: None,
                    delay: None,
                },
            ))
        } else {
            Ok((
                input,
                Timestamp::Active {
                    start_date,
                    start_time,
                    repeater: None,
                    delay: None,
                },
            ))
        }
    }

    pub(crate) fn parse_inactive(input: &str) -> IResult<&str, Timestamp<'_>> {
        let (input, _) = tag("[")(input)?;
        let (input, start_date) = Date::parse(input)?;
        let (input, _) = space0(input)?;
        let (input, start_time) = opt(Time::parse)(input)?;

        if input.starts_with('-') {
            let (input, end_time) = opt(Time::parse)(&input[1..])?;
            let (input, _) = space0(input)?;
            // TODO: delay-or-repeater
            let (input, _) = tag("]")(input)?;
            return Ok((
                input,
                Timestamp::InactiveRange {
                    start_date,
                    start_time,
                    end_date: start_date,
                    end_time,
                    repeater: None,
                    delay: None,
                },
            ));
        }

        let (input, _) = space0(input)?;
        // TODO: delay-or-repeater
        let (input, _) = tag("]")(input)?;

        if input.starts_with("--[") {
            let (input, end_date) = Date::parse(&input["--[".len()..])?;
            let (input, _) = space0(input)?;
            let (input, end_time) = opt(Time::parse)(input)?;
            let (input, _) = space0(input)?;
            // TODO: delay-or-repeater
            let (input, _) = tag("]")(input)?;
            Ok((
                input,
                Timestamp::InactiveRange {
                    start_date,
                    start_time,
                    end_date,
                    end_time,
                    repeater: None,
                    delay: None,
                },
            ))
        } else {
            Ok((
                input,
                Timestamp::Inactive {
                    start_date,
                    start_time,
                    repeater: None,
                    delay: None,
                },
            ))
        }
    }

    pub(crate) fn parse_diary(input: &str) -> IResult<&str, Timestamp<'_>> {
        let (input, _) = tag("<%%(")(input)?;
        let (input, sexp) = take_till(|c| c == ')' || c == '>' || c == '\n')(input)?;
        let (input, _) = tag(")>")(input)?;

        Ok((input, Timestamp::Diary(sexp)))
    }
}

// TODO
// #[cfg_attr(test, derive(PartialEq))]
// #[cfg_attr(feature = "serde", derive(serde::Serialize))]
// #[derive(Debug, Copy, Clone)]
// pub enum RepeaterType {
//     Cumulate,
//     CatchUp,
//     Restart,
// }

// #[cfg_attr(test, derive(PartialEq))]
// #[cfg_attr(feature = "serde", derive(serde::Serialize))]
// #[derive(Debug, Copy, Clone)]
// pub enum DelayType {
//     All,
//     First,
// }

// #[cfg_attr(test, derive(PartialEq))]
// #[cfg_attr(feature = "serde", derive(serde::Serialize))]
// #[derive(Debug, Copy, Clone)]
// pub enum TimeUnit {
//     Hour,
//     Day,
//     Week,
//     Month,
//     Year,
// }

// #[cfg_attr(test, derive(PartialEq))]
// #[cfg_attr(feature = "serde", derive(serde::Serialize))]
// #[derive(Debug, Copy, Clone)]
// pub struct Repeater {
//     pub ty: RepeaterType,
//     pub value: usize,
//     pub unit: TimeUnit,
// }

// #[cfg_attr(test, derive(PartialEq))]
// #[cfg_attr(feature = "serde", derive(serde::Serialize))]
// #[derive(Debug, Copy, Clone)]
// pub struct Delay {
//     pub ty: DelayType,
//     pub value: usize,
//     pub unit: TimeUnit,
// }

#[test]
fn parse() {
    assert_eq!(
        Timestamp::parse_inactive("[2003-09-16 Tue]"),
        Ok((
            "",
            Timestamp::Inactive {
                start_date: Date {
                    year: 2003,
                    month: 9,
                    day: 16,
                    dayname: "Tue"
                },
                start_time: None,
                repeater: None,
                delay: None,
            },
        ))
    );
    assert_eq!(
        Timestamp::parse_inactive("[2003-09-16 Tue 09:39]--[2003-09-16 Tue 10:39]"),
        Ok((
            "",
            Timestamp::InactiveRange {
                start_date: Date {
                    year: 2003,
                    month: 9,
                    day: 16,
                    dayname: "Tue"
                },
                start_time: Some(Time {
                    hour: 9,
                    minute: 39
                }),
                end_date: Date {
                    year: 2003,
                    month: 9,
                    day: 16,
                    dayname: "Tue"
                },
                end_time: Some(Time {
                    hour: 10,
                    minute: 39
                }),
                repeater: None,
                delay: None
            },
        ))
    );
    assert_eq!(
        Timestamp::parse_active("<2003-09-16 Tue 09:39-10:39>"),
        Ok((
            "",
            Timestamp::ActiveRange {
                start_date: Date {
                    year: 2003,
                    month: 9,
                    day: 16,
                    dayname: "Tue"
                },
                start_time: Some(Time {
                    hour: 9,
                    minute: 39
                }),
                end_date: Date {
                    year: 2003,
                    month: 9,
                    day: 16,
                    dayname: "Tue"
                },
                end_time: Some(Time {
                    hour: 10,
                    minute: 39
                }),
                repeater: None,
                delay: None
            },
        ))
    );
}
