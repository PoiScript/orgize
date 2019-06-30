use crate::elements::{Date, Element, Time, Timestamp};
use memchr::memchr;

/// clock elements
///
/// there are two types of clock: *closed* clock and *running* clock.
#[cfg_attr(test, derive(PartialEq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Debug)]
pub enum Clock<'a> {
    /// closed Clock
    Closed {
        start_date: Date<'a>,
        start_time: Option<Time>,
        end_date: Date<'a>,
        end_time: Option<Time>,
        repeater: Option<&'a str>,
        delay: Option<&'a str>,
        duration: &'a str,
    },
    /// running Clock
    Running {
        start_date: Date<'a>,
        start_time: Option<Time>,
        repeater: Option<&'a str>,
        delay: Option<&'a str>,
    },
}

impl Clock<'_> {
    pub(crate) fn parse(text: &str) -> Option<(&str, Element<'_>)> {
        let (text, eol) = memchr(b'\n', text.as_bytes())
            .map(|i| (text[..i].trim(), i + 1))
            .unwrap_or_else(|| (text.trim(), text.len()));

        if !text.starts_with("CLOCK:") {
            return None;
        }

        let tail = &text["CLOCK:".len()..].trim_start();

        if !tail.starts_with('[') {
            return None;
        }

        let (tail, timestamp) = Timestamp::parse_inactive(tail).ok()?;

        let tail = tail.trim();

        match timestamp {
            Timestamp::InactiveRange {
                start_date,
                start_time,
                end_date,
                end_time,
                repeater,
                delay,
            } if tail.starts_with("=>") => {
                let duration = &tail[3..].trim();
                let colon = memchr(b':', duration.as_bytes())?;
                if duration.as_bytes()[0..colon].iter().all(u8::is_ascii_digit)
                    && colon == duration.len() - 3
                    && duration.as_bytes()[colon + 1].is_ascii_digit()
                    && duration.as_bytes()[colon + 2].is_ascii_digit()
                {
                    Some((
                        &text[eol..],
                        Element::Clock(Clock::Closed {
                            start_date,
                            start_time,
                            end_date,
                            end_time,
                            repeater,
                            delay,
                            duration,
                        }),
                    ))
                } else {
                    None
                }
            }
            Timestamp::Inactive {
                start_date,
                start_time,
                repeater,
                delay,
            } if tail.is_empty() => Some((
                &text[eol..],
                Element::Clock(Clock::Running {
                    start_date,
                    start_time,
                    repeater,
                    delay,
                }),
            )),
            _ => None,
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
        match *self {
            Clock::Closed {
                start_date,
                start_time,
                end_date,
                end_time,
                repeater,
                delay,
                ..
            } => Timestamp::InactiveRange {
                start_date,
                start_time,
                end_date,
                end_time,
                repeater,
                delay,
            },
            Clock::Running {
                start_date,
                start_time,
                repeater,
                delay,
            } => Timestamp::Inactive {
                start_date,
                start_time,
                repeater,
                delay,
            },
        }
    }
}

#[test]
fn parse() {
    assert_eq!(
        Clock::parse("CLOCK: [2003-09-16 Tue 09:39]"),
        Some((
            "",
            Element::Clock(Clock::Running {
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
                repeater: None,
                delay: None,
            })
        ))
    );
    assert_eq!(
        Clock::parse("CLOCK: [2003-09-16 Tue 09:39]--[2003-09-16 Tue 10:39] =>  1:00"),
        Some((
            "",
            Element::Clock(Clock::Closed {
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
                delay: None,
                duration: "1:00",
            })
        ))
    );
}
