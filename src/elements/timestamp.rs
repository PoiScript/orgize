use std::borrow::Cow;
use std::convert::TryFrom;
use std::fmt::{self, Write};

use nom::{
    branch::{alt, permutation},
    bytes::complete::{tag, take, take_until, take_while1, take_while_m_n},
    character::complete::{char, digit1, space0, space1},
    combinator::{map, map_res, opt, value, verify},
    sequence::{delimited, preceded, separated_pair},
    IResult,
};

use crate::parsers::{helper_for_parse_element, Parse};

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

#[cfg_attr(feature = "ser", derive(serde::Serialize))]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum TimeUnit {
    Hour,
    Day,
    Week,
    Month,
    Year,
}

#[cfg_attr(feature = "ser", derive(serde::Serialize))]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Repeater {
    pub mark: RepeaterMark,
    pub value: usize,
    pub unit: TimeUnit,
}

#[cfg_attr(feature = "ser", derive(serde::Serialize))]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Delay {
    pub mark: DelayMark,
    pub value: usize,
    pub unit: TimeUnit,
}

#[cfg_attr(feature = "ser", derive(serde::Serialize))]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum RepeaterMark {
    Cumulate,
    CatchUp,
    Restart,
}

#[cfg_attr(feature = "ser", derive(serde::Serialize))]
#[derive(Clone, Copy, Debug, PartialEq)]
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

impl TryFrom<&str> for RepeaterMark {
    type Error = ();
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        helper_for_parse_element(parse_repeater_mark(s)).ok_or(())
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

impl TryFrom<&str> for DelayMark {
    type Error = ();
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        helper_for_parse_element(parse_delay_mark(s)).ok_or(())
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

impl TryFrom<&str> for TimeUnit {
    type Error = ();
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        helper_for_parse_element(parse_time_unit(s)).ok_or(())
    }
}

impl fmt::Display for Repeater {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}{}", self.mark, self.value, self.unit)
    }
}

impl TryFrom<&str> for Repeater {
    type Error = ();
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        helper_for_parse_element(parse_repeater(s)).ok_or(())
    }
}

impl fmt::Display for Delay {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}{}", self.mark, self.value, self.unit)
    }
}

impl TryFrom<&str> for Delay {
    type Error = ();
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        helper_for_parse_element(parse_delay(s)).ok_or(())
    }
}

impl fmt::Display for Datetime<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:04}-{:02}-{:02}", self.year, self.month, self.day,)?;
        if !self.dayname.is_empty() {
            write!(f, " {}", self.dayname)?;
        }
        if let (Some(hour), Some(minute)) = (self.hour, self.minute) {
            write!(f, " {:02}:{:02}", hour, minute)?;
        }
        Ok(())
    }
}

impl<'a> Parse<'a> for Datetime<'a> {
    fn parse(s: &'a str) -> Option<Datetime<'a>> {
        helper_for_parse_element(parse_datetime(s))
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
        start_repeater: Option<Repeater>,
        #[cfg_attr(feature = "ser", serde(skip_serializing_if = "Option::is_none"))]
        end_repeater: Option<Repeater>,
        #[cfg_attr(feature = "ser", serde(skip_serializing_if = "Option::is_none"))]
        start_delay: Option<Delay>,
        #[cfg_attr(feature = "ser", serde(skip_serializing_if = "Option::is_none"))]
        end_delay: Option<Delay>,
    },
    InactiveRange {
        start: Datetime<'a>,
        end: Datetime<'a>,
        #[cfg_attr(feature = "ser", serde(skip_serializing_if = "Option::is_none"))]
        start_repeater: Option<Repeater>,
        #[cfg_attr(feature = "ser", serde(skip_serializing_if = "Option::is_none"))]
        end_repeater: Option<Repeater>,
        #[cfg_attr(feature = "ser", serde(skip_serializing_if = "Option::is_none"))]
        start_delay: Option<Delay>,
        #[cfg_attr(feature = "ser", serde(skip_serializing_if = "Option::is_none"))]
        end_delay: Option<Delay>,
    },
    Diary {
        value: Cow<'a, str>,
    },
}

impl fmt::Display for Timestamp<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fn write_parts(
            f: &mut fmt::Formatter,
            repeater: &Option<Repeater>,
            delay: &Option<Delay>,
        ) -> fmt::Result {
            if let Some(repeater) = repeater {
                write!(f, " {}", repeater)?;
            }
            if let Some(delay) = delay {
                write!(f, " {}", delay)?;
            }
            Ok(())
        }

        fn write_range(
            f: &mut fmt::Formatter,
            open: char,
            close: char,
            start: &Datetime,
            end: &Datetime,
            start_repeater: &Option<Repeater>,
            start_delay: &Option<Delay>,
            end_repeater: &Option<Repeater>,
            end_delay: &Option<Delay>,
        ) -> fmt::Result {
            if start.year == end.year
                && start.month == end.month
                && start.day == end.day
                && start.dayname == end.dayname
                && start_repeater == end_repeater
                && start_delay == end_delay
                && start.hour.is_some()
                && start.minute.is_some()
                && end.hour.is_some()
                && end.minute.is_some()
            {
                write!(
                    f,
                    "{}{}-{:02}:{:02}",
                    open,
                    start,
                    end.hour.unwrap(),
                    end.minute.unwrap()
                )?;
                write_parts(f, start_repeater, start_delay)?;
            } else {
                write!(f, "{}{}", open, start)?;
                write_parts(f, start_repeater, start_delay)?;
                write!(f, "{}--{}{}", close, open, end)?;
                write_parts(f, end_repeater, end_delay)?;
            }
            f.write_char(close)
        }

        match self {
            Timestamp::Active {
                start,
                repeater,
                delay,
            } => {
                write!(f, "<{}", start)?;
                write_parts(f, repeater, delay)?;
                f.write_char('>')?;
            }
            Timestamp::Inactive {
                start,
                repeater,
                delay,
            } => {
                write!(f, "[{}", start)?;
                write_parts(f, repeater, delay)?;
                f.write_char(']')?;
            }
            Timestamp::ActiveRange {
                start,
                end,
                start_repeater,
                start_delay,
                end_repeater,
                end_delay,
            } => {
                write_range(
                    f,
                    '<',
                    '>',
                    start,
                    end,
                    start_repeater,
                    start_delay,
                    end_repeater,
                    end_delay,
                )?;
            }
            Timestamp::InactiveRange {
                start,
                end,
                start_repeater,
                start_delay,
                end_repeater,
                end_delay,
            } => {
                write_range(
                    f,
                    '[',
                    ']',
                    start,
                    end,
                    start_repeater,
                    start_delay,
                    end_repeater,
                    end_delay,
                )?;
            }
            Timestamp::Diary { value } => write!(f, "<%%({})>", value)?,
        }
        Ok(())
    }
}

impl Timestamp<'_> {
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
                start_repeater,
                end_repeater,
                start_delay,
                end_delay,
            } => Timestamp::ActiveRange {
                start: start.into_owned(),
                end: end.into_owned(),
                start_repeater,
                end_repeater,
                start_delay,
                end_delay,
            },
            Timestamp::InactiveRange {
                start,
                end,
                start_repeater,
                end_repeater,
                start_delay,
                end_delay,
            } => Timestamp::InactiveRange {
                start: start.into_owned(),
                end: end.into_owned(),
                start_repeater,
                end_repeater,
                start_delay,
                end_delay,
            },
            Timestamp::Diary { value } => Timestamp::Diary {
                value: value.into_owned().into(),
            },
        }
    }
}

impl<'a> Parse<'a> for Timestamp<'a> {
    fn parse(s: &'a str) -> Option<Timestamp<'a>> {
        helper_for_parse_element(parse_timestamp(s))
    }
}

fn parse_time(input: &str) -> IResult<&str, (u8, u8), ()> {
    let (input, hour) = map_res(take_while_m_n(1, 2, |c: char| c.is_ascii_digit()), |num| {
        u8::from_str_radix(num, 10)
    })(input)?;
    let (input, _) = tag(":")(input)?;
    let (input, minute) = map_res(take(2usize), |num| u8::from_str_radix(num, 10))(input)?;
    Ok((input, (hour, minute)))
}

fn parse_repeater_mark(input: &str) -> IResult<&str, RepeaterMark, ()> {
    alt((
        value(RepeaterMark::CatchUp, tag("++")),
        value(RepeaterMark::Cumulate, tag("+")),
        value(RepeaterMark::Restart, tag(".+")),
    ))(input)
}

fn parse_delay_mark(input: &str) -> IResult<&str, DelayMark, ()> {
    alt((
        value(DelayMark::First, tag("--")),
        value(DelayMark::All, tag("-")),
    ))(input)
}

fn parse_time_unit(input: &str) -> IResult<&str, TimeUnit, ()> {
    alt((
        value(TimeUnit::Hour, char('h')),
        value(TimeUnit::Day, char('d')),
        value(TimeUnit::Week, char('w')),
        value(TimeUnit::Month, char('m')),
        value(TimeUnit::Year, char('y')),
    ))(input)
}

fn parse_interval(input: &str) -> IResult<&str, (usize, TimeUnit), ()> {
    let (input, value) = map_res(digit1, |num| usize::from_str_radix(num, 10))(input)?;
    let (input, unit) = parse_time_unit(input)?;
    Ok((input, (value, unit)))
}

fn parse_repeater(input: &str) -> IResult<&str, Repeater, ()> {
    let (input, mark) = parse_repeater_mark(input)?;
    let (input, (value, unit)) = parse_interval(input)?;
    Ok((input, Repeater { mark, value, unit }))
}

fn parse_delay(input: &str) -> IResult<&str, Delay, ()> {
    let (input, mark) = parse_delay_mark(input)?;
    let (input, (value, unit)) = parse_interval(input)?;
    Ok((input, Delay { mark, value, unit }))
}

fn parse_repeater_and_delay(input: &str) -> IResult<&str, (Option<Repeater>, Option<Delay>), ()> {
    let (input, (repeater1, delay, repeater2)) = permutation((
        opt(preceded(space1, parse_repeater)),
        opt(preceded(space1, parse_delay)),
        opt(preceded(space1, parse_repeater)),
    ))(input)?;
    Ok((input, (repeater1.or(repeater2), delay)))
}

fn parse_dayname(input: &str) -> IResult<&str, &str, ()> {
    let (input, dayname) = verify(
        take_while1(|c: char| !c.is_whitespace() && c != '>' && c != ']'),
        |dayname: &str| {
            !dayname
                .chars()
                .any(|c| c.is_ascii_digit() || c == '+' || c == '-')
        },
    )(input)?;
    Ok((input, dayname))
}

fn parse_datetime<'a>(input: &'a str) -> IResult<&str, Datetime<'a>, ()> {
    let parse_u8 = |num| u8::from_str_radix(num, 10);

    let (input, year) = map_res(take(4usize), |num| u16::from_str_radix(num, 10))(input)?;
    let (input, _) = tag("-")(input)?;
    let (input, month) =
        map_res(take_while_m_n(1, 2, |c: char| c.is_ascii_digit()), parse_u8)(input)?;
    let (input, _) = tag("-")(input)?;
    let (input, day) =
        map_res(take_while_m_n(1, 2, |c: char| c.is_ascii_digit()), parse_u8)(input)?;
    let (input, dayname) = opt(preceded(space1, parse_dayname))(input)?;
    let (input, time) = opt(preceded(space1, parse_time))(input)?;
    let (hour, minute) = match time {
        Some((hour, minute)) => (Some(hour), Some(minute)),
        None => (None, None),
    };
    Ok((
        input,
        Datetime {
            year,
            month,
            day,
            dayname: dayname.unwrap_or_default().into(),
            hour,
            minute,
        },
    ))
}

#[cfg_attr(test, derive(PartialEq, Debug, Clone))]
struct TimestampParts<'a> {
    datetime: Datetime<'a>,
    end_time: Option<(u8, u8)>,
    repeater: Option<Repeater>,
    delay: Option<Delay>,
}

fn parse_timestamp_parts(input: &str) -> IResult<&str, TimestampParts, ()> {
    let (input, datetime) = parse_datetime(input)?;
    let (input, end_time) = opt(preceded(tag("-"), parse_time))(input)?;
    let (input, (repeater, delay)) = parse_repeater_and_delay(input)?;

    // Org timestamps allow terminal space before the > or ].
    let (input, _) = space0(input)?;
    Ok((
        input,
        TimestampParts {
            datetime,
            end_time,
            repeater,
            delay,
        },
    ))
}

pub(crate) fn parse_timestamp<'a>(input: &'a str) -> IResult<&str, Timestamp<'a>, ()> {
    alt((
        map(
            delimited(
                tag("<"),
                separated_pair(parse_timestamp_parts, tag(">--<"), parse_timestamp_parts),
                tag(">"),
            ),
            |(start, end)| Timestamp::ActiveRange {
                start: start.datetime,
                end: end.datetime,
                start_delay: start.delay,
                start_repeater: start.repeater,
                end_delay: end.delay,
                end_repeater: end.repeater,
            },
        ),
        map(
            delimited(
                tag("["),
                separated_pair(parse_timestamp_parts, tag("]--["), parse_timestamp_parts),
                tag("]"),
            ),
            |(start, end)| Timestamp::InactiveRange {
                start: start.datetime,
                end: end.datetime,
                start_delay: start.delay,
                start_repeater: start.repeater,
                end_delay: end.delay,
                end_repeater: end.repeater,
            },
        ),
        map(
            delimited(tag("<"), parse_timestamp_parts, tag(">")),
            |parts| match parts.end_time {
                Some((hour, minute)) => {
                    let mut end = parts.datetime.clone();
                    end.hour = Some(hour);
                    end.minute = Some(minute);
                    Timestamp::ActiveRange {
                        start: parts.datetime,
                        end,
                        start_repeater: parts.repeater,
                        end_repeater: parts.repeater,
                        start_delay: parts.delay,
                        end_delay: parts.delay,
                    }
                }
                None => Timestamp::Active {
                    start: parts.datetime,
                    delay: parts.delay,
                    repeater: parts.repeater,
                },
            },
        ),
        map(
            delimited(tag("["), parse_timestamp_parts, tag("]")),
            |parts| match parts.end_time {
                Some((hour, minute)) => {
                    let mut end = parts.datetime.clone();
                    end.hour = Some(hour);
                    end.minute = Some(minute);
                    Timestamp::InactiveRange {
                        start: parts.datetime,
                        end,
                        start_repeater: parts.repeater,
                        end_repeater: parts.repeater,
                        start_delay: parts.delay,
                        end_delay: parts.delay,
                    }
                }
                None => Timestamp::Inactive {
                    start: parts.datetime,
                    delay: parts.delay,
                    repeater: parts.repeater,
                },
            },
        ),
        map(
            delimited(tag("<%%("), take_until(")>"), tag(")>")),
            |diary: &str| Timestamp::Diary {
                value: diary.into(),
            },
        ),
    ))(input)
}

#[test]
fn parse() {
    let timestamp = Timestamp::Active {
        start: Datetime {
            year: 2019,
            month: 3,
            day: 26,
            dayname: "Fri".into(),
            hour: Some(3),
            minute: Some(33),
        },
        repeater: None,
        delay: None,
    };
    assert_eq!(
        parse_timestamp("<2019-03-26 Fri 03:33>"),
        Ok(("", timestamp.clone()))
    );
    assert_eq!(
        parse_timestamp("<2019-03-26  Fri   03:33   >"),
        Ok(("", timestamp))
    );
    // Org can't recognize leading space after the <.
    assert_eq!(parse_timestamp("< 2019-03-26  Fri   03:33   >").ok(), None,);

    assert_eq!(
        parse_timestamp("[2003-09-16 Tue]"),
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
        parse_timestamp("[2003-09-16 Tue 09:39]--[2003-09-16 Tue 10:39]"),
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
                start_repeater: None,
                end_repeater: None,
                start_delay: None,
                end_delay: None,
            },
        ))
    );

    // For consistency with org-element, when there are two end times specified
    // like this, we take the first time in each timestamp as the start/end
    // respectively. These are all invalid per the org spec, but org-element
    // handles them, and they seem like an case that could easily come up in
    // normal use.
    assert_eq!(
        parse_timestamp("<2003-09-16 Fri 03:14>--<2003-09-19 Mon 04:16-5:29>"),
        Ok((
            "",
            Timestamp::ActiveRange {
                start: Datetime {
                    year: 2003,
                    month: 9,
                    day: 16,
                    dayname: "Fri".into(),
                    hour: Some(3),
                    minute: Some(14),
                },
                end: Datetime {
                    year: 2003,
                    month: 9,
                    day: 19,
                    dayname: "Mon".into(),
                    hour: Some(4),
                    minute: Some(16),
                },
                start_repeater: None,
                end_repeater: None,
                start_delay: None,
                end_delay: None,
            },
        ))
    );
    assert_eq!(
        parse_timestamp("<2003-09-16 Fri 02:59-03:14>--<2003-09-16 Mon 5:29>"),
        Ok((
            "",
            Timestamp::ActiveRange {
                start: Datetime {
                    year: 2003,
                    month: 9,
                    day: 16,
                    dayname: "Fri".into(),
                    hour: Some(2),
                    minute: Some(59),
                },
                end: Datetime {
                    year: 2003,
                    month: 9,
                    day: 16,
                    dayname: "Mon".into(),
                    hour: Some(5),
                    minute: Some(29),
                },
                start_repeater: None,
                end_repeater: None,
                start_delay: None,
                end_delay: None,
            },
        ))
    );
    assert_eq!(
        parse_timestamp("<2003-09-16 Fri 02:59-03:14>--<2003-09-16 Mon 04:16-5:29>"),
        Ok((
            "",
            Timestamp::ActiveRange {
                start: Datetime {
                    year: 2003,
                    month: 9,
                    day: 16,
                    dayname: "Fri".into(),
                    hour: Some(2),
                    minute: Some(59),
                },
                end: Datetime {
                    year: 2003,
                    month: 9,
                    day: 16,
                    dayname: "Mon".into(),
                    hour: Some(4),
                    minute: Some(16),
                },
                start_repeater: None,
                end_repeater: None,
                start_delay: None,
                end_delay: None,
            },
        ))
    );

    assert_eq!(
        parse_timestamp("<2003-09-16 Fri>--<2003-09-19 Mon>"),
        Ok((
            "",
            Timestamp::ActiveRange {
                start: Datetime {
                    year: 2003,
                    month: 9,
                    day: 16,
                    dayname: "Fri".into(),
                    hour: None,
                    minute: None,
                },
                end: Datetime {
                    year: 2003,
                    month: 9,
                    day: 19,
                    dayname: "Mon".into(),
                    hour: None,
                    minute: None,
                },
                start_repeater: None,
                end_repeater: None,
                start_delay: None,
                end_delay: None,
            },
        ))
    );

    let repeater = Some(Repeater {
        mark: RepeaterMark::Cumulate,
        value: 1,
        unit: TimeUnit::Week,
    });
    let delay = Some(Delay {
        mark: DelayMark::First,
        value: 2,
        unit: TimeUnit::Day,
    });
    let timestamp = Timestamp::ActiveRange {
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
        start_repeater: repeater,
        end_repeater: repeater,
        start_delay: delay,
        end_delay: delay,
    };
    assert_eq!(
        parse_timestamp("<2003-09-16 Tue 09:39-10:39 +1w --2d>"),
        Ok(("", timestamp.clone()))
    );
    assert_eq!(
        parse_timestamp("<2003-09-16 Tue 09:39-10:39 --2d +1w>"),
        Ok(("", timestamp.clone()))
    );

    let repeater2 = Some(Repeater {
        mark: RepeaterMark::Restart,
        value: 1,
        unit: TimeUnit::Year,
    });
    let delay2 = Some(Delay {
        mark: DelayMark::All,
        value: 3,
        unit: TimeUnit::Hour,
    });
    assert_eq!(
        parse_timestamp("<2003-09-16 Tue 09:39 +1w --2d>--<2003-09-19 .+1y -3h>"),
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
                    day: 19,
                    dayname: Cow::default(),
                    hour: None,
                    minute: None,
                },
                start_repeater: repeater,
                end_repeater: repeater2,
                start_delay: delay,
                end_delay: delay2,
            },
        ))
    );

    assert_eq!(
        parse_timestamp("<%%(diary-date 2020 5 2)>"),
        Ok((
            "",
            Timestamp::Diary {
                value: "diary-date 2020 5 2".into()
            },
        ))
    );
}

#[test]
fn test_parse_datetime() {
    let mut datetime = Datetime {
        year: 2020,
        month: 3,
        day: 1,
        dayname: Cow::default(),
        hour: None,
        minute: None,
    };

    assert_eq!(parse_datetime("2020-03-01"), Ok(("", datetime.clone())));

    datetime.dayname = "Sun".into();
    assert_eq!(parse_datetime("2020-03-01 Sun"), Ok(("", datetime.clone())));

    datetime.dayname = "Zeepsday".into();
    assert_eq!(
        parse_datetime("2020-3-01 Zeepsday"),
        Ok(("", datetime.clone()))
    );

    datetime.dayname = "Sun".into();
    datetime.month = 7;
    assert_eq!(parse_datetime("2020-07-1 Sun"), Ok(("", datetime.clone())));

    datetime.dayname = "FRI".into();
    datetime.month = 1;
    assert_eq!(parse_datetime("2020-1-1 FRI"), Ok(("", datetime.clone())));

    datetime.dayname = Cow::default();
    datetime.month = 1;
    assert_eq!(parse_datetime("2020-1-1 "), Ok((" ", datetime.clone())));

    datetime.year = 0;
    assert_eq!(parse_datetime("0000-1-1"), Ok(("", datetime.clone())));

    datetime.year = 5;
    assert_eq!(parse_datetime("0005-1-1"), Ok(("", datetime.clone())));

    datetime.year = 9999;
    assert_eq!(parse_datetime("9999-1-1"), Ok(("", datetime.clone())));

    assert_eq!(parse_datetime("2-1-1 .+1d/2d").ok(), None,);

    assert_eq!(parse_datetime("-1-1").ok(), None,);
    assert_eq!(parse_datetime("1-1").ok(), None,);
    assert_eq!(parse_datetime("1").ok(), None,);
    assert_eq!(parse_datetime("").ok(), None,);
}

#[test]
fn test_parse_time() {
    assert_eq!(parse_time("").ok(), None);
    assert_eq!(parse_time(":").ok(), None);
    assert_eq!(parse_time("9").ok(), None);
    assert_eq!(parse_time("9-9").ok(), None);
    assert_eq!(parse_time("00:09-00:10"), Ok(("-00:10", (0, 9))));

    assert_eq!(parse_time("5:5").ok(), None);
    assert_eq!(parse_time(":5").ok(), None);
    assert_eq!(parse_time(":05").ok(), None);

    assert_eq!(parse_time(" 5:05").ok(), None);
    assert_eq!(parse_time("5:05"), Ok(("", (5, 5))));
    assert_eq!(parse_time("05:05"), Ok(("", (5, 5))));
    assert_eq!(parse_time("00:05\tbees"), Ok(("\tbees", (0, 5))));
    assert_eq!(parse_time("00:00"), Ok(("", (0, 0))));
    assert_eq!(parse_time("0:00"), Ok(("", (0, 0))));
    assert_eq!(parse_time("0:01"), Ok(("", (0, 1))));
}

#[test]
fn test_parse_interval() {
    assert_eq!(parse_interval("5d"), Ok(("", (5, TimeUnit::Day))));
    assert_eq!(parse_interval("0h"), Ok(("", (0, TimeUnit::Hour))));
    assert_eq!(parse_interval("1h"), Ok(("", (1, TimeUnit::Hour))));
    assert_eq!(parse_interval("2m"), Ok(("", (2, TimeUnit::Month))));
    assert_eq!(
        parse_interval("02m hello\nworld"),
        Ok((" hello\nworld", (2, TimeUnit::Month)))
    );
    assert_eq!(parse_interval("222y"), Ok(("", (222, TimeUnit::Year))));
    assert_eq!(parse_interval("").ok(), None);
    assert_eq!(parse_interval("5").ok(), None);
    assert_eq!(parse_interval("y").ok(), None);
    assert_eq!(parse_interval("y5").ok(), None);
}

#[test]
fn test_parse_repeater_and_delay() {
    // Note that parse_repeater_and_delay needs a leading space if non-empty,
    // due to where it sits in parsing timestamps.
    for nope in &[
        " ", "+1d", "--1w", " -1", " 1", " +", " ++", " .+", " -", " --", " +1", " ++1", " .+",
        " -1", " --1", " 1d", " 5w", " y",
    ] {
        assert_eq!(parse_repeater_and_delay(*nope), Ok((*nope, (None, None))));
    }

    let mut repeater = Repeater {
        mark: RepeaterMark::Cumulate,
        unit: TimeUnit::Day,
        value: 1,
    };
    assert_eq!(
        parse_repeater_and_delay(" +1d"),
        Ok(("", (Some(repeater), None)))
    );

    repeater.mark = RepeaterMark::Restart;
    assert_eq!(
        parse_repeater_and_delay(" .+1d"),
        Ok(("", (Some(repeater), None)))
    );

    repeater.mark = RepeaterMark::CatchUp;
    assert_eq!(
        parse_repeater_and_delay(" ++1d"),
        Ok(("", (Some(repeater), None)))
    );

    let mut delay = Delay {
        mark: DelayMark::First,
        unit: TimeUnit::Year,
        value: 7,
    };
    assert_eq!(
        parse_repeater_and_delay(" --7y"),
        Ok(("", (None, Some(delay))))
    );

    delay.mark = DelayMark::All;
    assert_eq!(
        parse_repeater_and_delay(" -7y"),
        Ok(("", (None, Some(delay))))
    );

    delay.unit = TimeUnit::Hour;
    assert_eq!(
        parse_repeater_and_delay(" -7h"),
        Ok(("", (None, Some(delay))))
    );

    delay.unit = TimeUnit::Month;
    assert_eq!(
        parse_repeater_and_delay(" -7m"),
        Ok(("", (None, Some(delay))))
    );

    delay.unit = TimeUnit::Week;
    assert_eq!(
        parse_repeater_and_delay(" -7w"),
        Ok(("", (None, Some(delay))))
    );
    assert_eq!(
        parse_repeater_and_delay(" -07w"),
        Ok(("", (None, Some(delay))))
    );

    assert_eq!(
        parse_repeater_and_delay(" -7w ++1d"),
        Ok(("", (Some(repeater), Some(delay))))
    );
    assert_eq!(
        parse_repeater_and_delay(" ++1d -7w"),
        Ok(("", (Some(repeater), Some(delay))))
    );
}

#[test]
fn test_parse_timestamp_parts() {
    let mut parts = TimestampParts {
        datetime: Datetime {
            year: 2020,
            month: 1,
            day: 1,
            dayname: "".into(),
            hour: None,
            minute: None,
        },
        end_time: None,
        repeater: None,
        delay: None,
    };

    assert_eq!(parse_timestamp_parts("2020-01-01"), Ok(("", parts.clone())));
    parts.datetime.hour = Some(1);
    parts.datetime.minute = Some(0);
    assert_eq!(
        parse_timestamp_parts("2020-01-01 01:00"),
        Ok(("", parts.clone()))
    );
    parts.end_time = Some((2, 0));
    assert_eq!(
        parse_timestamp_parts("2020-01-01 01:00-02:00"),
        Ok(("", parts.clone()))
    );
    parts.end_time = None;
    parts.datetime.hour = None;
    parts.datetime.minute = None;
    parts.datetime.dayname = "Fri".into();
    assert_eq!(
        parse_timestamp_parts("2020-01-01 Fri"),
        Ok(("", parts.clone()))
    );
    parts.datetime.hour = Some(1);
    parts.datetime.minute = Some(0);
    assert_eq!(
        parse_timestamp_parts("2020-01-01 Fri 01:00"),
        Ok(("", parts.clone()))
    );
    parts.end_time = Some((2, 0));
    assert_eq!(
        parse_timestamp_parts("2020-01-01 Fri 01:00-02:00"),
        Ok(("", parts.clone()))
    );
    parts.delay = Some(Delay {
        mark: DelayMark::All,
        value: 3,
        unit: TimeUnit::Day,
    });
    parts.datetime.dayname = "".into();
    parts.datetime.hour = None;
    parts.datetime.minute = None;
    parts.end_time = None;
    assert_eq!(
        parse_timestamp_parts("2020-01-01 -3d"),
        Ok(("", parts.clone()))
    );
    parts.datetime.hour = Some(1);
    parts.datetime.minute = Some(0);
    assert_eq!(
        parse_timestamp_parts("2020-01-01 01:00 -3d"),
        Ok(("", parts.clone()))
    );
    parts.end_time = Some((2, 0));
    assert_eq!(
        parse_timestamp_parts("2020-01-01 01:00-02:00 -3d"),
        Ok(("", parts.clone()))
    );
    parts.datetime.dayname = "Fri".into();
    parts.datetime.hour = None;
    parts.datetime.minute = None;
    parts.end_time = None;
    assert_eq!(
        parse_timestamp_parts("2020-01-01 Fri -3d"),
        Ok(("", parts.clone()))
    );
    parts.datetime.hour = Some(1);
    parts.datetime.minute = Some(0);
    assert_eq!(
        parse_timestamp_parts("2020-01-01 Fri 01:00 -3d"),
        Ok(("", parts.clone()))
    );
    parts.end_time = Some((2, 0));
    assert_eq!(
        parse_timestamp_parts("2020-01-01 Fri 01:00-02:00 -3d"),
        Ok(("", parts.clone()))
    );
    parts.datetime.hour = None;
    parts.datetime.minute = None;
    parts.end_time = None;
    assert_eq!(
        parse_timestamp_parts("2020-01-01 Fri -3d"),
        Ok(("", parts.clone()))
    );

    parts.repeater = Some(Repeater {
        mark: RepeaterMark::CatchUp,
        value: 2,
        unit: TimeUnit::Week,
    });
    parts.datetime.dayname = "Mon".into();
    parts.datetime.hour = Some(3);
    parts.datetime.minute = Some(14);
    assert_eq!(
        parse_timestamp_parts("2020-01-01 Mon 03:14 ++2w -3d hello"),
        Ok(("hello", parts.clone()))
    );
}

#[test]
fn test_format_repeater() {
    for s in &["+5d", "+1w", ".+9h", "++7m", "+9y", "++0d"] {
        assert_eq!(*s, Repeater::try_from(*s).unwrap().to_string());
    }

    assert_eq!("+9d", Repeater::try_from("+09d").unwrap().to_string());

    for s in &["++", "", "+", ".+", "++5", "+6", "-1d", "6d", "5w"] {
        assert_eq!(None, Repeater::try_from(*s).ok())
    }
}

#[test]
fn test_format_delay() {
    for s in &["-5d", "-1w", "--9h", "--7m", "--9y", "--0w"] {
        assert_eq!(*s, Delay::try_from(*s).unwrap().to_string());
    }

    assert_eq!("--9d", Delay::try_from("--09d").unwrap().to_string());

    for s in &["--", "", "-", "--5", "-6", ".+1d", "+1d", "6d", "5w"] {
        assert_eq!(None, Delay::try_from(*s).ok())
    }
}

#[test]
fn test_format_datetime() {
    for s in &[
        "2020-01-01",
        "2019-05-09 Zeepsday",
        "2025-09-09 Mon 03:05",
        "2025-09-09 03:05",
    ] {
        assert_eq!(*s, Datetime::parse(*s).unwrap().to_string());
    }
}

#[test]
fn test_format_timestamp() {
    for s in &[
        "<2017-02-23>",
        "[2017-02-23]",
        "<2016-05-29 Mon>",
        "[2016-05-29 Mon]",
        "<2020-03-05 .+1w -1d>",
        "[2020-03-05 .+1w -1d]",
        "<1990-01-01 Fri 03:55>",
        "[1990-01-01 Fri 03:55]",
        "<1991-01-01 Mon 03:55-04:00>",
        "[1991-01-01 Mon 03:55-04:00]",
        "<1991-01-01 Mon 03:55>--<1992-02-03 Tue 05:59>",
        "[1991-01-01 Mon 03:55]--[1992-02-03 Tue 05:59]",
        "[1991-01-01 Mon]--[1992-02-03 Tue 05:59]",
        "[1991-01-01 Mon 03:55]--[1992-02-03 Tue]",
        "<1991-01-01 Mon 03:55 +1d -1w>--<1992-02-03 Tue +2w -2d>",
    ] {
        assert_eq!(*s, Timestamp::parse(s).unwrap().to_string());
    }

    assert_eq!(
        "<1991-01-01 Mon 03:55-04:00>",
        Timestamp::parse("<1991-01-01 Mon 03:55>--<1991-01-01 Mon 04:00>")
            .unwrap()
            .to_string()
    );

    assert_eq!(
        "<1991-01-01 +1d -1w>",
        Timestamp::parse("<1991-01-01 -1w +1d>")
            .unwrap()
            .to_string()
    );

    assert_eq!(
        "<1991-01-01 05:05>",
        Timestamp::parse("<1991-01-01 5:05>").unwrap().to_string()
    );

    assert_eq!(
        "<1991-01-01 05:05-07:09>",
        Timestamp::parse("<1991-1-1 5:05-7:09>")
            .unwrap()
            .to_string()
    );
}
