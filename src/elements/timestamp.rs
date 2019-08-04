use nom::{
    bytes::complete::{tag, take, take_till, take_while, take_while_m_n},
    character::complete::{space0, space1},
    combinator::{map, map_res, opt},
    IResult,
};

/// Datetime
///
/// # Syntax
///
/// ```text
/// YYYY-MM-DD DAYNAME
/// ```
///
#[cfg_attr(test, derive(PartialEq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Debug, Clone)]
pub struct Datetime<'a> {
    pub year: u16,
    pub month: u8,
    pub day: u8,
    pub dayname: &'a str,
    pub hour: Option<u8>,
    pub minute: Option<u8>,
}

fn parse_time(input: &str) -> IResult<&str, (u8, u8)> {
    let (input, hour) = map_res(take_while_m_n(1, 2, |c: char| c.is_ascii_digit()), |num| {
        u8::from_str_radix(num, 10)
    })(input)?;
    let (input, _) = tag(":")(input)?;
    let (input, minute) = map_res(take(2usize), |num| u8::from_str_radix(num, 10))(input)?;
    Ok((input, (hour, minute)))
}

fn parse_datetime(input: &str) -> IResult<&str, Datetime<'_>> {
    let parse_u8 = |num| u8::from_str_radix(num, 10);

    let (input, year) = map_res(take(4usize), |num| u16::from_str_radix(num, 10))(input)?;
    let (input, _) = tag("-")(input)?;
    let (input, month) = map_res(take(2usize), parse_u8)(input)?;
    let (input, _) = tag("-")(input)?;
    let (input, day) = map_res(take(2usize), parse_u8)(input)?;
    let (input, _) = space1(input)?;
    let (input, dayname) = take_while(|c: char| {
        !c.is_ascii_whitespace()
            && !c.is_ascii_digit()
            && c != '+'
            && c != '-'
            && c != ']'
            && c != '>'
    })(input)?;
    let (input, (hour, minute)) = map(
        opt(|input| {
            let (input, _) = space1(input)?;
            parse_time(input)
        }),
        |time| (time.map(|t| t.0), time.map(|t| t.1)),
    )(input)?;

    Ok((
        input,
        Datetime {
            year,
            month,
            day,
            dayname,
            hour,
            minute,
        },
    ))
}

#[cfg(feature = "chrono")]
mod chrono {
    use super::Datetime;
    use chrono::*;

    impl Into<NaiveDate> for Datetime<'_> {
        fn into(self) -> NaiveDate {
            NaiveDate::from_ymd(self.year.into(), self.month.into(), self.day.into())
        }
    }

    impl Into<NaiveTime> for Datetime<'_> {
        fn into(self) -> NaiveTime {
            NaiveTime::from_hms(
                self.hour.unwrap_or_default().into(),
                self.minute.unwrap_or_default().into(),
                0,
            )
        }
    }

    impl Into<NaiveDateTime> for Datetime<'_> {
        fn into(self) -> NaiveDateTime {
            NaiveDate::from_ymd(self.year.into(), self.month.into(), self.day.into()).and_hms(
                self.hour.unwrap_or_default().into(),
                self.minute.unwrap_or_default().into(),
                0,
            )
        }
    }
}

#[cfg_attr(test, derive(PartialEq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(tag = "type", rename_all = "snake_case"))]
#[derive(Debug)]
pub enum Timestamp<'a> {
    Active {
        start: Datetime<'a>,
        repeater: Option<&'a str>,
        delay: Option<&'a str>,
    },
    Inactive {
        start: Datetime<'a>,
        repeater: Option<&'a str>,
        delay: Option<&'a str>,
    },
    ActiveRange {
        start: Datetime<'a>,
        end: Datetime<'a>,
        repeater: Option<&'a str>,
        delay: Option<&'a str>,
    },
    InactiveRange {
        start: Datetime<'a>,
        end: Datetime<'a>,
        repeater: Option<&'a str>,
        delay: Option<&'a str>,
    },
    Diary {
        value: &'a str,
    },
}

impl Timestamp<'_> {
    pub(crate) fn parse_active(input: &str) -> IResult<&str, Timestamp<'_>> {
        let (input, _) = tag("<")(input)?;
        let (input, start) = parse_datetime(input)?;

        if input.starts_with('-') {
            let (input, (hour, minute)) = parse_time(&input[1..])?;
            let (input, _) = space0(input)?;
            // TODO: delay-or-repeater
            let (input, _) = tag(">")(input)?;
            let mut end = start.clone();
            end.hour = Some(hour);
            end.minute = Some(minute);
            return Ok((
                input,
                Timestamp::ActiveRange {
                    start,
                    end,
                    repeater: None,
                    delay: None,
                },
            ));
        }

        let (input, _) = space0(input)?;
        // TODO: delay-or-repeater
        let (input, _) = tag(">")(input)?;

        if input.starts_with("--<") {
            let (input, end) = parse_datetime(&input["--<".len()..])?;
            let (input, _) = space0(input)?;
            // TODO: delay-or-repeater
            let (input, _) = tag(">")(input)?;
            Ok((
                input,
                Timestamp::ActiveRange {
                    start,
                    end,
                    repeater: None,
                    delay: None,
                },
            ))
        } else {
            Ok((
                input,
                Timestamp::Active {
                    start,
                    repeater: None,
                    delay: None,
                },
            ))
        }
    }

    pub(crate) fn parse_inactive(input: &str) -> IResult<&str, Timestamp<'_>> {
        let (input, _) = tag("[")(input)?;
        let (input, start) = parse_datetime(input)?;

        if input.starts_with('-') {
            let (input, (hour, minute)) = parse_time(&input[1..])?;
            let (input, _) = space0(input)?;
            // TODO: delay-or-repeater
            let (input, _) = tag("]")(input)?;
            let mut end = start.clone();
            end.hour = Some(hour);
            end.minute = Some(minute);
            return Ok((
                input,
                Timestamp::InactiveRange {
                    start,
                    end,
                    repeater: None,
                    delay: None,
                },
            ));
        }

        let (input, _) = space0(input)?;
        // TODO: delay-or-repeater
        let (input, _) = tag("]")(input)?;

        if input.starts_with("--[") {
            let (input, end) = parse_datetime(&input["--[".len()..])?;
            let (input, _) = space0(input)?;
            // TODO: delay-or-repeater
            let (input, _) = tag("]")(input)?;
            Ok((
                input,
                Timestamp::InactiveRange {
                    start,
                    end,
                    repeater: None,
                    delay: None,
                },
            ))
        } else {
            Ok((
                input,
                Timestamp::Inactive {
                    start,
                    repeater: None,
                    delay: None,
                },
            ))
        }
    }

    pub(crate) fn parse_diary(input: &str) -> IResult<&str, Timestamp<'_>> {
        let (input, _) = tag("<%%(")(input)?;
        let (input, value) = take_till(|c| c == ')' || c == '>' || c == '\n')(input)?;
        let (input, _) = tag(")>")(input)?;

        Ok((input, Timestamp::Diary { value }))
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
                start: Datetime {
                    year: 2003,
                    month: 9,
                    day: 16,
                    dayname: "Tue",
                    hour: None,
                    minute: None
                },
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
                start: Datetime {
                    year: 2003,
                    month: 9,
                    day: 16,
                    dayname: "Tue",
                    hour: Some(9),
                    minute: Some(39)
                },
                end: Datetime {
                    year: 2003,
                    month: 9,
                    day: 16,
                    dayname: "Tue",
                    hour: Some(10),
                    minute: Some(39),
                },
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
                start: Datetime {
                    year: 2003,
                    month: 9,
                    day: 16,
                    dayname: "Tue",
                    hour: Some(9),
                    minute: Some(39),
                },
                end: Datetime {
                    year: 2003,
                    month: 9,
                    day: 16,
                    dayname: "Tue",
                    hour: Some(10),
                    minute: Some(39),
                },
                repeater: None,
                delay: None
            },
        ))
    );
}
