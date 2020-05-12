use std::borrow::Cow;
use std::fmt::{self, Write};

use nom::{
    branch::{alt, permutation},
    bytes::complete::{tag, take, take_until, take_while1, take_while_m_n},
    character::complete::{digit1, one_of, space0, space1},
    combinator::{map, map_res, opt, verify},
    sequence::{delimited, preceded, separated_pair},
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
            self.year, self.month, self.day, self.dayname,
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

fn parse_time(input: &str) -> IResult<&str, (u8, u8), ()> {
    let (input, hour) = map_res(take_while_m_n(1, 2, |c: char| c.is_ascii_digit()), |num| {
        u8::from_str_radix(num, 10)
    })(input)?;
    let (input, _) = tag(":")(input)?;
    let (input, minute) = map_res(take(2usize), |num| u8::from_str_radix(num, 10))(input)?;
    Ok((input, (hour, minute)))
}

fn parse_repeater_mark(input: &str) -> IResult<&str, RepeaterMark, ()> {
    let (input, mark) = alt((tag("++"), tag("+"), tag(".+")))(input)?;
    Ok((
        input,
        match mark {
            "++" => RepeaterMark::CatchUp,
            "+" => RepeaterMark::Cumulate,
            ".+" => RepeaterMark::Restart,
            _ => unreachable!(),
        },
    ))
}

fn parse_delay_mark(input: &str) -> IResult<&str, DelayMark, ()> {
    let (input, mark) = alt((tag("--"), tag("-")))(input)?;
    Ok((
        input,
        match mark {
            "--" => DelayMark::First,
            "-" => DelayMark::All,
            _ => unreachable!(),
        },
    ))
}

fn parse_time_unit(input: &str) -> IResult<&str, TimeUnit, ()> {
    let (input, unit) = one_of("hdwmy")(input)?;
    Ok((
        input,
        match unit {
            'h' => TimeUnit::Hour,
            'd' => TimeUnit::Day,
            'w' => TimeUnit::Week,
            'm' => TimeUnit::Month,
            'y' => TimeUnit::Year,
            _ => unreachable!(),
        },
    ))
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
    let (input, (hour, minute)) = map(opt(preceded(space1, parse_time)), |time| {
        (time.map(|t| t.0), time.map(|t| t.1))
    })(input)?;
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
            |parts| Timestamp::Inactive {
                start: parts.datetime,
                delay: parts.delay,
                repeater: parts.repeater,
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
    assert_eq!(
        parse_timestamp("<2019-03-26 Fri 03:33>"),
        Ok((
            "",
            Timestamp::Active {
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
            },
        ))
    );
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
    assert_eq!(
        parse_timestamp("<2003-09-16 Tue 09:39-10:39 +1w --2d>"),
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
                start_repeater: repeater,
                end_repeater: repeater,
                start_delay: delay,
                end_delay: delay,
            },
        ))
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
