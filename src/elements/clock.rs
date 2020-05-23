use std::borrow::Cow;

use nom::{
    bytes::complete::tag,
    character::complete::{char, digit1, space0},
    combinator::recognize,
    sequence::separated_pair,
    IResult,
};

use crate::elements::timestamp::{parse_timestamp, Datetime, Delay, Repeater, Timestamp};
use crate::parse::combinators::{blank_lines_count, eol};

/// Clock Element
#[cfg_attr(test, derive(PartialEq))]
#[cfg_attr(feature = "ser", derive(serde::Serialize))]
#[cfg_attr(feature = "ser", serde(untagged))]
#[derive(Debug, Clone)]
pub enum Clock<'a> {
    /// Closed Clock
    Closed {
        /// Time start
        start: Datetime<'a>,
        /// Time end
        end: Datetime<'a>,
        #[cfg_attr(feature = "ser", serde(skip_serializing_if = "Option::is_none"))]
        start_repeater: Option<Repeater>,
        #[cfg_attr(feature = "ser", serde(skip_serializing_if = "Option::is_none"))]
        end_repeater: Option<Repeater>,
        #[cfg_attr(feature = "ser", serde(skip_serializing_if = "Option::is_none"))]
        start_delay: Option<Delay>,
        #[cfg_attr(feature = "ser", serde(skip_serializing_if = "Option::is_none"))]
        end_delay: Option<Delay>,
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
        repeater: Option<Repeater>,
        #[cfg_attr(feature = "ser", serde(skip_serializing_if = "Option::is_none"))]
        delay: Option<Delay>,
        /// Numbers of blank lines between the clock line and next non-blank
        /// line or buffer's end
        post_blank: usize,
    },
}

impl Clock<'_> {
    pub(crate) fn parse(input: &str) -> Option<(&str, Clock)> {
        parse_internal(input).ok()
    }

    pub fn into_onwed(self) -> Clock<'static> {
        match self {
            Clock::Closed {
                start,
                end,
                start_repeater,
                end_repeater,
                start_delay,
                end_delay,
                duration,
                post_blank,
            } => Clock::Closed {
                start: start.into_owned(),
                end: end.into_owned(),
                start_repeater,
                end_repeater,
                start_delay,
                end_delay,
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
                repeater,
                delay,
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
                start_repeater,
                end_repeater,
                start_delay,
                end_delay,
                ..
            } => Timestamp::InactiveRange {
                start: start.clone(),
                end: end.clone(),
                start_repeater: *start_repeater,
                end_repeater: *end_repeater,
                start_delay: *start_delay,
                end_delay: *end_delay,
            },
            Clock::Running {
                start,
                repeater,
                delay,
                ..
            } => Timestamp::Inactive {
                start: start.clone(),
                repeater: *repeater,
                delay: *delay,
            },
        }
    }
}

fn parse_internal(input: &str) -> IResult<&str, Clock, ()> {
    let (input, _) = space0(input)?;
    let (input, _) = tag("CLOCK:")(input)?;
    let (input, _) = space0(input)?;
    let (input, timestamp) = parse_timestamp(input)?;

    match timestamp {
        Timestamp::InactiveRange {
            start,
            end,
            start_repeater,
            end_repeater,
            start_delay,
            end_delay,
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
                    start_repeater,
                    end_repeater,
                    start_delay,
                    end_delay,
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
    assert_eq!(
        Clock::parse("CLOCK: [2003-09-16 Tue 09:39]"),
        Some((
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
        Clock::parse("CLOCK: [2003-09-16 Tue 09:39]--[2003-09-16 Tue 10:39] =>  1:00\n\n"),
        Some((
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
                start_repeater: None,
                end_repeater: None,
                start_delay: None,
                end_delay: None,
                duration: "1:00".into(),
                post_blank: 1,
            }
        ))
    );
}
