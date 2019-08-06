use std::borrow::Cow;

use nom::{
    bytes::complete::tag,
    character::complete::{char, digit1, space0},
    combinator::recognize,
    sequence::separated_pair,
    IResult,
};

use crate::elements::{Datetime, Element, Timestamp};
use crate::parsers::eol;

/// clock elements
///
/// there are two types of clock: *closed* clock and *running* clock.
#[cfg_attr(test, derive(PartialEq))]
#[cfg_attr(feature = "ser", derive(serde::Serialize))]
#[cfg_attr(feature = "ser", serde(untagged))]
#[derive(Debug)]
pub enum Clock<'a> {
    /// closed Clock
    Closed {
        start: Datetime<'a>,
        end: Datetime<'a>,
        #[cfg_attr(feature = "ser", serde(skip_serializing_if = "Option::is_none"))]
        repeater: Option<Cow<'a, str>>,
        #[cfg_attr(feature = "ser", serde(skip_serializing_if = "Option::is_none"))]
        delay: Option<Cow<'a, str>>,
        duration: Cow<'a, str>,
    },
    /// running Clock
    Running {
        start: Datetime<'a>,
        #[cfg_attr(feature = "ser", serde(skip_serializing_if = "Option::is_none"))]
        repeater: Option<Cow<'a, str>>,
        #[cfg_attr(feature = "ser", serde(skip_serializing_if = "Option::is_none"))]
        delay: Option<Cow<'a, str>>,
    },
}

impl Clock<'_> {
    pub(crate) fn parse(input: &str) -> IResult<&str, Element<'_>> {
        let (input, _) = tag("CLOCK:")(input)?;
        let (input, _) = space0(input)?;
        let (input, timestamp) = Timestamp::parse_inactive(input)?;

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
                let (input, duration) =
                    recognize(separated_pair(digit1, char(':'), digit1))(input)?;
                let (input, _) = eol(input)?;
                Ok((
                    input,
                    Element::Clock(Clock::Closed {
                        start,
                        end,
                        repeater,
                        delay,
                        duration: duration.into(),
                    }),
                ))
            }
            Timestamp::Inactive {
                start,
                repeater,
                delay,
            } => {
                let (input, _) = eol(input)?;
                Ok((
                    input,
                    Element::Clock(Clock::Running {
                        start,
                        repeater,
                        delay,
                    }),
                ))
            }
            _ => unreachable!(
                "`Timestamp::parse_inactive` only returns `Timestamp::InactiveRange` or `Timestamp::Inactive`."
            ),
        }
    }

    /// returns `true` if the clock is running
    pub fn is_running(&self) -> bool {
        match self {
            Clock::Closed { .. } => false,
            Clock::Running { .. } => true,
        }
    }

    /// returns `true` if the clock is closed
    pub fn is_closed(&self) -> bool {
        match self {
            Clock::Closed { .. } => true,
            Clock::Running { .. } => false,
        }
    }

    /// returns `Some` if the clock is closed, `None` if running
    pub fn duration(&self) -> Option<&str> {
        match self {
            Clock::Closed { duration, .. } => Some(duration),
            Clock::Running { .. } => None,
        }
    }

    /// constructs a new timestamp object from the clock
    pub fn value(&self) -> Timestamp<'_> {
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
            } => Timestamp::Inactive {
                start: start.clone(),
                repeater: repeater.clone(),
                delay: delay.clone(),
            },
        }
    }
}

#[test]
fn parse() {
    assert_eq!(
        Clock::parse("CLOCK: [2003-09-16 Tue 09:39]"),
        Ok((
            "",
            Element::Clock(Clock::Running {
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
            })
        ))
    );
    assert_eq!(
        Clock::parse("CLOCK: [2003-09-16 Tue 09:39]--[2003-09-16 Tue 10:39] =>  1:00"),
        Ok((
            "",
            Element::Clock(Clock::Closed {
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
            })
        ))
    );
}
