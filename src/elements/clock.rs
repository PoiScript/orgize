use std::borrow::Cow;

use nom::{
    bytes::complete::tag,
    character::complete::{char, digit1, space0},
    combinator::recognize,
    error::ParseError,
    sequence::separated_pair,
    IResult,
};

use crate::elements::timestamp::{parse_inactive, Datetime, Timestamp};
use crate::parse::combinators::{blank_lines_count, eol};

/// Clock Element
#[cfg_attr(test, derive(PartialEq))]
#[cfg_attr(feature = "ser", derive(serde::Serialize))]
#[cfg_attr(feature = "ser", serde(untagged))]
#[derive(Debug)]
pub enum Clock<'a> {
    /// Closed Clock
    Closed {
        /// Time start
        start: Datetime<'a>,
        /// Time end
        end: Datetime<'a>,
        #[cfg_attr(feature = "ser", serde(skip_serializing_if = "Option::is_none"))]
        repeater: Option<Cow<'a, str>>,
        #[cfg_attr(feature = "ser", serde(skip_serializing_if = "Option::is_none"))]
        delay: Option<Cow<'a, str>>,
        /// Clock duration
        duration: Cow<'a, str>,
        /// Numbers of blank lines between the clock line and next non-blank
        /// line or buffer's end
        post_blank: usize,
    },
    /// Running Clock
    Running {
        /// Time start
        start: Datetime<'a>,
        #[cfg_attr(feature = "ser", serde(skip_serializing_if = "Option::is_none"))]
        repeater: Option<Cow<'a, str>>,
        #[cfg_attr(feature = "ser", serde(skip_serializing_if = "Option::is_none"))]
        delay: Option<Cow<'a, str>>,
        /// Numbers of blank lines between the clock line and next non-blank
        /// line or buffer's end
        post_blank: usize,
    },
}

impl Clock<'_> {
    pub(crate) fn parse(input: &str) -> Option<(&str, Clock)> {
        parse_clock::<()>(input).ok()
    }

    pub fn into_onwed(self) -> Clock<'static> {
        match self {
            Clock::Closed {
                start,
                end,
                repeater,
                delay,
                duration,
                post_blank,
            } => Clock::Closed {
                start: start.into_owned(),
                end: end.into_owned(),
                repeater: repeater.map(Into::into).map(Cow::Owned),
                delay: delay.map(Into::into).map(Cow::Owned),
                duration: duration.into_owned().into(),
                post_blank,
            },
            Clock::Running {
                start,
                repeater,
                delay,
                post_blank,
            } => Clock::Running {
                start: start.into_owned(),
                repeater: repeater.map(Into::into).map(Cow::Owned),
                delay: delay.map(Into::into).map(Cow::Owned),
                post_blank,
            },
        }
    }

    /// Returns `true` if the clock is running.
    pub fn is_running(&self) -> bool {
        match self {
            Clock::Closed { .. } => false,
            Clock::Running { .. } => true,
        }
    }

    /// Returns `true` if the clock is closed.
    pub fn is_closed(&self) -> bool {
        match self {
            Clock::Closed { .. } => true,
            Clock::Running { .. } => false,
        }
    }

    /// Returns clock duration, or `None` if it's running.
    pub fn duration(&self) -> Option<&str> {
        match self {
            Clock::Closed { duration, .. } => Some(duration),
            Clock::Running { .. } => None,
        }
    }

    /// Constructs a timestamp from the clock.
    pub fn value(&self) -> Timestamp {
        match &*self {
            Clock::Closed {
                start,
                end,
                repeater,
                delay,
                ..
            } => Timestamp::InactiveRange {
                start: start.clone(),
                end: end.clone(),
                repeater: repeater.clone(),
                delay: delay.clone(),
            },
            Clock::Running {
                start,
                repeater,
                delay,
                ..
            } => Timestamp::Inactive {
                start: start.clone(),
                repeater: repeater.clone(),
                delay: delay.clone(),
            },
        }
    }
}

fn parse_clock<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&str, Clock, E> {
    let (input, _) = space0(input)?;
    let (input, _) = tag("CLOCK:")(input)?;
    let (input, _) = space0(input)?;
    let (input, timestamp) = parse_inactive(input)?;

    match timestamp {
        Timestamp::InactiveRange {
            start,
            end,
            repeater,
            delay,
        } => {
            let (input, _) = space0(input)?;
            let (input, _) = tag("=>")(input)?;
            let (input, _) = space0(input)?;
            let (input, duration) = recognize(separated_pair(digit1, char(':'), digit1))(input)?;
            let (input, _) = eol(input)?;
            let (input, blank) = blank_lines_count(input)?;
            Ok((
                input,
                Clock::Closed {
                    start,
                    end,
                    repeater,
                    delay,
                    duration: duration.into(),
                    post_blank: blank,
                },
            ))
        }
        Timestamp::Inactive {
            start,
            repeater,
            delay,
        } => {
            let (input, _) = eol(input)?;
            let (input, blank) = blank_lines_count(input)?;
            Ok((
                input,
                Clock::Running {
                    start,
                    repeater,
                    delay,
                    post_blank: blank,
                },
            ))
        }
        _ => unreachable!(
            "`parse_inactive` only returns `Timestamp::InactiveRange` or `Timestamp::Inactive`."
        ),
    }
}

#[test]
fn parse() {
    use nom::error::VerboseError;

    assert_eq!(
        parse_clock::<VerboseError<&str>>("CLOCK: [2003-09-16 Tue 09:39]"),
        Ok((
            "",
            Clock::Running {
                start: Datetime {
                    year: 2003,
                    month: 9,
                    day: 16,
                    dayname: "Tue".into(),
                    hour: Some(9),
                    minute: Some(39)
                },
                repeater: None,
                delay: None,
                post_blank: 0,
            }
        ))
    );
    assert_eq!(
        parse_clock::<VerboseError<&str>>(
            "CLOCK: [2003-09-16 Tue 09:39]--[2003-09-16 Tue 10:39] =>  1:00\n\n"
        ),
        Ok((
            "",
            Clock::Closed {
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
                    minute: Some(39)
                },
                repeater: None,
                delay: None,
                duration: "1:00".into(),
                post_blank: 1,
            }
        ))
    );
}
