use crate::objects::timestamp::{Datetime, Delay, Repeater, Timestamp};
use memchr::memchr;

/// clock elements
///
/// there are two types of clock: *closed* clock and *running* clock.
#[cfg_attr(test, derive(PartialEq))]
#[derive(Debug)]
pub enum Clock<'a> {
    /// closed Clock
    Closed {
        start: Datetime<'a>,
        end: Datetime<'a>,
        repeater: Option<Repeater>,
        delay: Option<Delay>,
        duration: &'a str,
    },
    /// running Clock
    Running {
        start: Datetime<'a>,
        repeater: Option<Repeater>,
        delay: Option<Delay>,
    },
}

impl<'a> Clock<'a> {
    pub(crate) fn parse(text: &'a str) -> Option<(Clock<'a>, usize)> {
        let (text, off) = memchr(b'\n', text.as_bytes())
            .map(|i| (text[..i].trim(), i + 1))
            .unwrap_or_else(|| (text.trim(), text.len()));

        let tail = memchr(b' ', text.as_bytes())
            .filter(|&i| &text[0..i] == "CLOCK:")
            .map(|i| text[i..].trim_start())?;

        if !tail.starts_with('[') {
            return None;
        }

        match Timestamp::parse_inactive(tail).map(|(t, off)| (t, tail[off..].trim_start())) {
            Some((
                Timestamp::InactiveRange {
                    start,
                    end,
                    repeater,
                    delay,
                },
                tail,
            )) => {
                if tail.starts_with("=>") {
                    let duration = &tail[3..].trim();
                    let colon = memchr(b':', duration.as_bytes())?;
                    if duration.as_bytes()[0..colon].iter().all(u8::is_ascii_digit)
                        && colon == duration.len() - 3
                        && duration.as_bytes()[colon + 1].is_ascii_digit()
                        && duration.as_bytes()[colon + 2].is_ascii_digit()
                    {
                        return Some((
                            Clock::Closed {
                                start,
                                end,
                                repeater,
                                delay,
                                duration,
                            },
                            off,
                        ));
                    }
                }
            }
            Some((
                Timestamp::Inactive {
                    start,
                    repeater,
                    delay,
                },
                tail,
            )) => {
                if tail.as_bytes().iter().all(u8::is_ascii_whitespace) {
                    return Some((
                        Clock::Running {
                            start,
                            repeater,
                            delay,
                        },
                        off,
                    ));
                }
            }
            _ => (),
        }

        None
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
    pub fn duration(&self) -> Option<&'a str> {
        match self {
            Clock::Closed { duration, .. } => Some(duration),
            Clock::Running { .. } => None,
        }
    }

    /// constructs a new timestamp object from the clock
    pub fn value(&self) -> Timestamp<'_> {
        match *self {
            Clock::Closed {
                start,
                end,
                repeater,
                delay,
                ..
            } => Timestamp::InactiveRange {
                start,
                end,
                repeater,
                delay,
            },
            Clock::Running {
                start,
                repeater,
                delay,
                ..
            } => Timestamp::Inactive {
                start,
                repeater,
                delay,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Clock;
    use crate::objects::timestamp::Datetime;

    #[test]
    fn parse() {
        assert_eq!(
            Clock::parse("CLOCK: [2003-09-16 Tue 09:39]"),
            Some((
                Clock::Running {
                    start: Datetime {
                        date: (2003, 9, 16),
                        time: Some((9, 39)),
                        dayname: "Tue"
                    },
                    repeater: None,
                    delay: None,
                },
                "CLOCK: [2003-09-16 Tue 09:39]".len()
            ))
        );
        assert_eq!(
            Clock::parse("CLOCK: [2003-09-16 Tue 09:39]--[2003-09-16 Tue 10:39] =>  1:00"),
            Some((
                Clock::Closed {
                    start: Datetime {
                        date: (2003, 9, 16),
                        time: Some((9, 39)),
                        dayname: "Tue"
                    },
                    end: Datetime {
                        date: (2003, 9, 16),
                        time: Some((10, 39)),
                        dayname: "Tue"
                    },
                    repeater: None,
                    delay: None,
                    duration: "1:00",
                },
                "CLOCK: [2003-09-16 Tue 09:39]--[2003-09-16 Tue 10:39] =>  1:00".len()
            ))
        );
    }
}
