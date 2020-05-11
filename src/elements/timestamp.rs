use std::borrow::Cow;
use std::fmt::{self, Write};

use nom::{
    bytes::complete::{tag, take, take_till, take_while, take_while_m_n},
    character::complete::{space0, space1},
    combinator::{map, map_res, opt},
    sequence::preceded,
    IResult,
};

/// Datetime Struct
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

#[cfg_attr(test, derive(PartialEq))]
#[cfg_attr(feature = "ser", derive(serde::Serialize))]
#[derive(Debug, Copy, Clone)]
pub enum TimeUnit {
    Hour,
    Day,
    Week,
    Month,
    Year,
}

#[cfg_attr(test, derive(PartialEq))]
#[cfg_attr(feature = "ser", derive(serde::Serialize))]
#[derive(Debug, Copy, Clone)]
pub struct Repeater {
    pub mark: RepeaterMark,
    pub value: usize,
    pub unit: TimeUnit,
}

#[cfg_attr(test, derive(PartialEq))]
#[cfg_attr(feature = "ser", derive(serde::Serialize))]
#[derive(Debug, Copy, Clone)]
pub struct Delay {
    pub mark: DelayMark,
    pub value: usize,
    pub unit: TimeUnit,
}

#[cfg_attr(test, derive(PartialEq))]
#[cfg_attr(feature = "ser", derive(serde::Serialize))]
#[derive(Debug, Copy, Clone)]
pub enum RepeaterMark {
    Cumulate,
    CatchUp,
    Restart,
}

#[cfg_attr(test, derive(PartialEq))]
#[cfg_attr(feature = "ser", derive(serde::Serialize))]
#[derive(Clone, Copy, Debug)]
pub enum DelayMark {
    All,
    First,
}

impl AsRef<str> for RepeaterMark {
    fn as_ref(&self) -> &str {
        match self {
            RepeaterMark::CatchUp => "++",
            RepeaterMark::Cumulate => "+",
            RepeaterMark::Restart => ".+",
        }
    }
}

impl fmt::Display for RepeaterMark {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.as_ref())
    }
}

impl AsRef<str> for DelayMark {
    fn as_ref(&self) -> &str {
        match self {
            DelayMark::All => "-",
            DelayMark::First => "--",
        }
    }
}

impl fmt::Display for DelayMark {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.as_ref())
    }
}

impl AsRef<str> for TimeUnit {
    fn as_ref(&self) -> &str {
        match self {
            TimeUnit::Hour => "h",
            TimeUnit::Day => "d",
            TimeUnit::Week => "w",
            TimeUnit::Month => "m",
            TimeUnit::Year => "y",
        }
    }
}

impl Into<char> for TimeUnit {
    fn into(self) -> char {
        self.as_ref().chars().next().unwrap()
    }
}

impl fmt::Display for TimeUnit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_char((*self).into())
    }
}

impl fmt::Display for Repeater {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}{}", self.mark, self.value, self.unit)
    }
}

impl fmt::Display for Delay {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}{}", self.mark, self.value, self.unit)
    }
}

impl fmt::Display for Datetime<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{:04}-{:02}-{:02} {}",
            self.year, self.month, self.day, self.dayname
        )?;
        if let (Some(hour), Some(minute)) = (self.hour, self.minute) {
            write!(f, " {:02}:{:02}", hour, minute)?;
        }
        Ok(())
    }
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
#[derive(Debug, Clone)]
pub enum Timestamp<'a> {
    Active {
        start: Datetime<'a>,
        #[cfg_attr(feature = "ser", serde(skip_serializing_if = "Option::is_none"))]
        repeater: Option<Repeater>,
        #[cfg_attr(feature = "ser", serde(skip_serializing_if = "Option::is_none"))]
        delay: Option<Delay>,
    },
    Inactive {
        start: Datetime<'a>,
        #[cfg_attr(feature = "ser", serde(skip_serializing_if = "Option::is_none"))]
        repeater: Option<Repeater>,
        #[cfg_attr(feature = "ser", serde(skip_serializing_if = "Option::is_none"))]
        delay: Option<Delay>,
    },
    ActiveRange {
        start: Datetime<'a>,
        end: Datetime<'a>,
        #[cfg_attr(feature = "ser", serde(skip_serializing_if = "Option::is_none"))]
        repeater: Option<Repeater>,
        #[cfg_attr(feature = "ser", serde(skip_serializing_if = "Option::is_none"))]
        delay: Option<Delay>,
    },
    InactiveRange {
        start: Datetime<'a>,
        end: Datetime<'a>,
        #[cfg_attr(feature = "ser", serde(skip_serializing_if = "Option::is_none"))]
        repeater: Option<Repeater>,
        #[cfg_attr(feature = "ser", serde(skip_serializing_if = "Option::is_none"))]
        delay: Option<Delay>,
    },
    Diary {
        value: Cow<'a, str>,
    },
}

impl fmt::Display for Timestamp<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Timestamp::Active { start, .. } => {
                write!(f, "<{}>", start)?;
            }
            Timestamp::Inactive { start, .. } => {
                write!(f, "[{}]", start)?;
            }
            Timestamp::ActiveRange { start, end, .. } => {
                write!(f, "<{}>--<{}>", start, end)?;
            }
            Timestamp::InactiveRange { start, end, .. } => {
                write!(f, "<{}>--<{}>", start, end)?;
            }
            Timestamp::Diary { value } => write!(f, "<%%({})>", value)?,
        }
        Ok(())
    }
}

impl Timestamp<'_> {
    pub(crate) fn parse_active(input: &str) -> Option<(&str, Timestamp)> {
        parse_active(input).ok()
    }

    pub(crate) fn parse_inactive(input: &str) -> Option<(&str, Timestamp)> {
        parse_inactive(input).ok()
    }

    pub(crate) fn parse_diary(input: &str) -> Option<(&str, Timestamp)> {
        parse_diary(input).ok()
    }

    pub fn into_owned(self) -> Timestamp<'static> {
        match self {
            Timestamp::Active {
                start,
                repeater,
                delay,
            } => Timestamp::Active {
                start: start.into_owned(),
                repeater,
                delay,
            },
            Timestamp::Inactive {
                start,
                repeater,
                delay,
            } => Timestamp::Inactive {
                start: start.into_owned(),
                repeater,
                delay,
            },
            Timestamp::ActiveRange {
                start,
                end,
                repeater,
                delay,
            } => Timestamp::ActiveRange {
                start: start.into_owned(),
                end: end.into_owned(),
                repeater,
                delay,
            },
            Timestamp::InactiveRange {
                start,
                end,
                repeater,
                delay,
            } => Timestamp::InactiveRange {
                start: start.into_owned(),
                end: end.into_owned(),
                repeater,
                delay,
            },
            Timestamp::Diary { value } => Timestamp::Diary {
                value: value.into_owned().into(),
            },
        }
    }
}

pub fn parse_active(input: &str) -> IResult<&str, Timestamp, ()> {
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

pub fn parse_inactive(input: &str) -> IResult<&str, Timestamp, ()> {
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

pub fn parse_diary(input: &str) -> IResult<&str, Timestamp, ()> {
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

fn parse_time(input: &str) -> IResult<&str, (u8, u8), ()> {
    let (input, hour) = map_res(take_while_m_n(1, 2, |c: char| c.is_ascii_digit()), |num| {
        u8::from_str_radix(num, 10)
    })(input)?;
    let (input, _) = tag(":")(input)?;
    let (input, minute) = map_res(take(2usize), |num| u8::from_str_radix(num, 10))(input)?;
    Ok((input, (hour, minute)))
}

fn parse_datetime(input: &str) -> IResult<&str, Datetime, ()> {
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

#[test]
fn parse() {
    assert_eq!(
        parse_inactive("[2003-09-16 Tue]"),
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
        parse_inactive("[2003-09-16 Tue 09:39]--[2003-09-16 Tue 10:39]"),
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
        parse_active("<2003-09-16 Tue 09:39-10:39>"),
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
