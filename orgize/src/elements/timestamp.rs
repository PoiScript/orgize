use std::borrow::Cow;

use nom::{
    bytes::complete::{tag, take, take_till, take_while, take_while_m_n},
    character::complete::{space0, space1},
    combinator::{map, map_res, opt},
    error::ParseError,
    sequence::preceded,
    IResult,
};

/// Orgize Datetime Struct
#[cfg_attr(test, derive(PartialEq))]
#[cfg_attr(feature = "ser", derive(serde::Serialize))]
#[derive(Debug, Clone)]
pub struct Datetime<'a> {
    pub year: u16,
    pub month: u8,
    pub day: u8,
    pub dayname: Cow<'a, str>,
    #[cfg_attr(feature = "ser", serde(skip_serializing_if = "Option::is_none"))]
    pub hour: Option<u8>,
    #[cfg_attr(feature = "ser", serde(skip_serializing_if = "Option::is_none"))]
    pub minute: Option<u8>,
}

impl Datetime<'_> {
    pub fn into_owned(self) -> Datetime<'static> {
        Datetime {
            year: self.year,
            month: self.month,
            day: self.day,
            dayname: self.dayname.into_owned().into(),
            hour: self.hour,
            minute: self.minute,
        }
    }
}

#[cfg(feature = "chrono")]
mod chrono {
    use super::Datetime;
    use chrono::*;

    impl Into<NaiveDate> for Datetime<'_> {
        fn into(self) -> NaiveDate {
            (&self).into()
        }
    }

    impl Into<NaiveTime> for Datetime<'_> {
        fn into(self) -> NaiveTime {
            (&self).into()
        }
    }

    impl Into<NaiveDateTime> for Datetime<'_> {
        fn into(self) -> NaiveDateTime {
            (&self).into()
        }
    }

    impl Into<DateTime<Utc>> for Datetime<'_> {
        fn into(self) -> DateTime<Utc> {
            (&self).into()
        }
    }

    impl Into<NaiveDate> for &Datetime<'_> {
        fn into(self) -> NaiveDate {
            NaiveDate::from_ymd(self.year.into(), self.month.into(), self.day.into())
        }
    }

    impl Into<NaiveTime> for &Datetime<'_> {
        fn into(self) -> NaiveTime {
            NaiveTime::from_hms(
                self.hour.unwrap_or_default().into(),
                self.minute.unwrap_or_default().into(),
                0,
            )
        }
    }

    impl Into<NaiveDateTime> for &Datetime<'_> {
        fn into(self) -> NaiveDateTime {
            NaiveDateTime::new(self.into(), self.into())
        }
    }

    impl Into<DateTime<Utc>> for &Datetime<'_> {
        fn into(self) -> DateTime<Utc> {
            DateTime::from_utc(self.into(), Utc)
        }
    }
}

/// Timestamp Object
#[cfg_attr(test, derive(PartialEq))]
#[cfg_attr(feature = "ser", derive(serde::Serialize))]
#[cfg_attr(feature = "ser", serde(rename_all = "kebab-case"))]
#[cfg_attr(feature = "ser", serde(tag = "timestamp_type"))]
#[derive(Debug)]
pub enum Timestamp<'a> {
    Active {
        start: Datetime<'a>,
        #[cfg_attr(feature = "ser", serde(skip_serializing_if = "Option::is_none"))]
        repeater: Option<Cow<'a, str>>,
        #[cfg_attr(feature = "ser", serde(skip_serializing_if = "Option::is_none"))]
        delay: Option<Cow<'a, str>>,
    },
    Inactive {
        start: Datetime<'a>,
        #[cfg_attr(feature = "ser", serde(skip_serializing_if = "Option::is_none"))]
        repeater: Option<Cow<'a, str>>,
        #[cfg_attr(feature = "ser", serde(skip_serializing_if = "Option::is_none"))]
        delay: Option<Cow<'a, str>>,
    },
    ActiveRange {
        start: Datetime<'a>,
        end: Datetime<'a>,
        #[cfg_attr(feature = "ser", serde(skip_serializing_if = "Option::is_none"))]
        repeater: Option<Cow<'a, str>>,
        #[cfg_attr(feature = "ser", serde(skip_serializing_if = "Option::is_none"))]
        delay: Option<Cow<'a, str>>,
    },
    InactiveRange {
        start: Datetime<'a>,
        end: Datetime<'a>,
        #[cfg_attr(feature = "ser", serde(skip_serializing_if = "Option::is_none"))]
        repeater: Option<Cow<'a, str>>,
        #[cfg_attr(feature = "ser", serde(skip_serializing_if = "Option::is_none"))]
        delay: Option<Cow<'a, str>>,
    },
    Diary {
        value: Cow<'a, str>,
    },
}

impl Timestamp<'_> {
    pub(crate) fn parse_active(input: &str) -> Option<(&str, Timestamp)> {
        parse_active::<()>(input).ok()
    }

    pub(crate) fn parse_inactive(input: &str) -> Option<(&str, Timestamp)> {
        parse_inactive::<()>(input).ok()
    }

    pub(crate) fn parse_diary(input: &str) -> Option<(&str, Timestamp)> {
        parse_diary::<()>(input).ok()
    }

    pub fn into_owned(self) -> Timestamp<'static> {
        match self {
            Timestamp::Active {
                start,
                repeater,
                delay,
            } => Timestamp::Active {
                start: start.into_owned(),
                repeater: repeater.map(Into::into).map(Cow::Owned),
                delay: delay.map(Into::into).map(Cow::Owned),
            },
            Timestamp::Inactive {
                start,
                repeater,
                delay,
            } => Timestamp::Inactive {
                start: start.into_owned(),
                repeater: repeater.map(Into::into).map(Cow::Owned),
                delay: delay.map(Into::into).map(Cow::Owned),
            },
            Timestamp::ActiveRange {
                start,
                end,
                repeater,
                delay,
            } => Timestamp::ActiveRange {
                start: start.into_owned(),
                end: end.into_owned(),
                repeater: repeater.map(Into::into).map(Cow::Owned),
                delay: delay.map(Into::into).map(Cow::Owned),
            },
            Timestamp::InactiveRange {
                start,
                end,
                repeater,
                delay,
            } => Timestamp::InactiveRange {
                start: start.into_owned(),
                end: end.into_owned(),
                repeater: repeater.map(Into::into).map(Cow::Owned),
                delay: delay.map(Into::into).map(Cow::Owned),
            },
            Timestamp::Diary { value } => Timestamp::Diary {
                value: value.into_owned().into(),
            },
        }
    }
}

pub fn parse_active<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&str, Timestamp, E> {
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

pub fn parse_inactive<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&str, Timestamp, E> {
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

pub fn parse_diary<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&str, Timestamp, E> {
    let (input, _) = tag("<%%(")(input)?;
    let (input, value) = take_till(|c| c == ')' || c == '>' || c == '\n')(input)?;
    let (input, _) = tag(")>")(input)?;

    Ok((
        input,
        Timestamp::Diary {
            value: value.into(),
        },
    ))
}

fn parse_time<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&str, (u8, u8), E> {
    let (input, hour) = map_res(take_while_m_n(1, 2, |c: char| c.is_ascii_digit()), |num| {
        u8::from_str_radix(num, 10)
    })(input)?;
    let (input, _) = tag(":")(input)?;
    let (input, minute) = map_res(take(2usize), |num| u8::from_str_radix(num, 10))(input)?;
    Ok((input, (hour, minute)))
}

fn parse_datetime<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&str, Datetime, E> {
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
    let (input, (hour, minute)) = map(opt(preceded(space1, parse_time)), |time| {
        (time.map(|t| t.0), time.map(|t| t.1))
    })(input)?;

    Ok((
        input,
        Datetime {
            year,
            month,
            day,
            dayname: dayname.into(),
            hour,
            minute,
        },
    ))
}

// TODO
// #[cfg_attr(test, derive(PartialEq))]
// #[cfg_attr(feature = "ser", derive(serde::Serialize))]
// #[derive(Debug, Copy, Clone)]
// pub enum RepeaterType {
//     Cumulate,
//     CatchUp,
//     Restart,
// }

// #[cfg_attr(test, derive(PartialEq))]
// #[cfg_attr(feature = "ser", derive(serde::Serialize))]
// #[derive(Debug, Copy, Clone)]
// pub enum DelayType {
//     All,
//     First,
// }

// #[cfg_attr(test, derive(PartialEq))]
// #[cfg_attr(feature = "ser", derive(serde::Serialize))]
// #[derive(Debug, Copy, Clone)]
// pub enum TimeUnit {
//     Hour,
//     Day,
//     Week,
//     Month,
//     Year,
// }

// #[cfg_attr(test, derive(PartialEq))]
// #[cfg_attr(feature = "ser", derive(serde::Serialize))]
// #[derive(Debug, Copy, Clone)]
// pub struct Repeater {
//     pub ty: RepeaterType,
//     pub value: usize,
//     pub unit: TimeUnit,
// }

// #[cfg_attr(test, derive(PartialEq))]
// #[cfg_attr(feature = "ser", derive(serde::Serialize))]
// #[derive(Debug, Copy, Clone)]
// pub struct Delay {
//     pub ty: DelayType,
//     pub value: usize,
//     pub unit: TimeUnit,
// }

#[test]
fn parse() {
    use nom::error::VerboseError;

    assert_eq!(
        parse_inactive::<VerboseError<&str>>("[2003-09-16 Tue]"),
        Ok((
            "",
            Timestamp::Inactive {
                start: Datetime {
                    year: 2003,
                    month: 9,
                    day: 16,
                    dayname: "Tue".into(),
                    hour: None,
                    minute: None
                },
                repeater: None,
                delay: None,
            },
        ))
    );
    assert_eq!(
        parse_inactive::<VerboseError<&str>>("[2003-09-16 Tue 09:39]--[2003-09-16 Tue 10:39]"),
        Ok((
            "",
            Timestamp::InactiveRange {
                start: Datetime {
                    year: 2003,
                    month: 9,
                    day: 16,
                    dayname: "Tue".into(),
                    hour: Some(9),
                    minute: Some(39)
                },
                end: Datetime {
                    year: 2003,
                    month: 9,
                    day: 16,
                    dayname: "Tue".into(),
                    hour: Some(10),
                    minute: Some(39),
                },
                repeater: None,
                delay: None
            },
        ))
    );
    assert_eq!(
        parse_active::<VerboseError<&str>>("<2003-09-16 Tue 09:39-10:39>"),
        Ok((
            "",
            Timestamp::ActiveRange {
                start: Datetime {
                    year: 2003,
                    month: 9,
                    day: 16,
                    dayname: "Tue".into(),
                    hour: Some(9),
                    minute: Some(39),
                },
                end: Datetime {
                    year: 2003,
                    month: 9,
                    day: 16,
                    dayname: "Tue".into(),
                    hour: Some(10),
                    minute: Some(39),
                },
                repeater: None,
                delay: None
            },
        ))
    );
}
