use memchr::memchr;
use std::str::FromStr;

#[cfg_attr(test, derive(PartialEq))]
#[derive(Debug, Clone, Copy)]
pub struct Datetime<'a> {
    pub(crate) date: &'a str,
    pub(crate) time: Option<&'a str>,
    pub(crate) dayname: &'a str,
}

impl<'a> Datetime<'a> {
    pub fn year(&self) -> u32 {
        u32::from_str(&self.date[0..4]).unwrap()
    }

    pub fn month(&self) -> u32 {
        u32::from_str(&self.date[5..7]).unwrap()
    }

    pub fn day(&self) -> u32 {
        u32::from_str(&self.date[8..10]).unwrap()
    }

    pub fn hour(&self) -> Option<u32> {
        self.time.map(|time| {
            if time.len() == 4 {
                u32::from_str(&time[0..1]).unwrap()
            } else {
                u32::from_str(&time[0..2]).unwrap()
            }
        })
    }

    pub fn minute(&self) -> Option<u32> {
        self.time.map(|time| {
            if time.len() == 4 {
                u32::from_str(&time[2..4]).unwrap()
            } else {
                u32::from_str(&time[3..5]).unwrap()
            }
        })
    }

    pub fn dayname(&self) -> &str {
        self.dayname
    }
}

#[cfg(feature = "chrono")]
mod chrono {
    use super::Datetime;
    use chrono::*;

    impl<'a> Datetime<'a> {
        pub fn naive_date(&self) -> NaiveDate {
            NaiveDate::from_ymd(self.year() as i32, self.month(), self.day())
        }

        pub fn naive_time(&self) -> NaiveTime {
            NaiveTime::from_hms(self.hour().unwrap_or(0), self.minute().unwrap_or(0), 0)
        }

        pub fn naive_date_time(&self) -> NaiveDateTime {
            NaiveDateTime::new(self.naive_date(), self.naive_time())
        }

        pub fn date_time<Tz: TimeZone>(&self, offset: Tz::Offset) -> DateTime<Tz> {
            DateTime::from_utc(self.naive_date_time(), offset)
        }

        pub fn date<Tz: TimeZone>(&self, offset: Tz::Offset) -> Date<Tz> {
            Date::from_utc(self.naive_date(), offset)
        }
    }
}

#[cfg_attr(test, derive(PartialEq))]
#[derive(Debug, Clone, Copy)]
pub enum RepeaterType {
    Cumulate,
    CatchUp,
    Restart,
}

#[cfg_attr(test, derive(PartialEq))]
#[derive(Debug, Copy, Clone)]
pub enum DelayType {
    All,
    First,
}

#[cfg_attr(test, derive(PartialEq))]
#[derive(Debug, Copy, Clone)]
pub enum TimeUnit {
    Hour,
    Day,
    Week,
    Month,
    Year,
}

#[cfg_attr(test, derive(PartialEq))]
#[derive(Debug, Copy, Clone)]
pub struct Repeater {
    pub ty: RepeaterType,
    pub value: usize,
    pub unit: TimeUnit,
}

#[cfg_attr(test, derive(PartialEq))]
#[derive(Debug, Copy, Clone)]
pub struct Delay {
    pub ty: DelayType,
    pub value: usize,
    pub unit: TimeUnit,
}

/// timestamp obejcts
#[cfg_attr(test, derive(PartialEq))]
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
    Diary(&'a str),
}

impl<'a> Timestamp<'a> {
    pub(crate) fn parse(text: &'a str) -> Option<(Timestamp<'a>, usize)> {
        if text.starts_with('<') {
            Timestamp::parse_active(text).or_else(|| Timestamp::parse_diary(text))
        } else if text.starts_with('[') {
            Timestamp::parse_inactive(text)
        } else {
            None
        }
    }

    pub(crate) fn parse_active(text: &'a str) -> Option<(Timestamp<'a>, usize)> {
        debug_assert!(text.starts_with('<'));

        let bytes = text.as_bytes();
        let mut off = memchr(b'>', bytes)?;
        let (start, mut end) = Self::parse_datetime(&text[1..off])?;

        if end.is_none()
            && off + "--<YYYY-MM-DD >".len() <= text.len()
            && text[off + 1..].starts_with("--<")
        {
            if let Some(new_off) = memchr(b'>', &bytes[off + 1..]) {
                if let Some((start, _)) = Self::parse_datetime(&text[off + 4..off + 1 + new_off]) {
                    end = Some(start);
                    off += new_off + 1;
                }
            }
        }

        Some((
            if let Some(end) = end {
                Timestamp::ActiveRange {
                    start,
                    end,
                    repeater: None,
                    delay: None,
                }
            } else {
                Timestamp::Active {
                    start,
                    repeater: None,
                    delay: None,
                }
            },
            off + 1,
        ))
    }

    pub(crate) fn parse_inactive(text: &'a str) -> Option<(Timestamp<'a>, usize)> {
        debug_assert!(text.starts_with('['));

        let bytes = text.as_bytes();
        let mut off = memchr(b']', bytes)?;
        let (start, mut end) = Self::parse_datetime(&text[1..off])?;
        if end.is_none()
            && off + "--[YYYY-MM-DD ]".len() <= text.len()
            && text[off + 1..].starts_with("--[")
        {
            if let Some(new_off) = memchr(b']', &bytes[off + 1..]) {
                if let Some((start, _)) = Self::parse_datetime(&text[off + 4..off + 1 + new_off]) {
                    end = Some(start);
                    off += new_off + 1;
                }
            }
        }

        Some((
            if let Some(end) = end {
                Timestamp::InactiveRange {
                    start,
                    end,
                    repeater: None,
                    delay: None,
                }
            } else {
                Timestamp::Inactive {
                    start,
                    repeater: None,
                    delay: None,
                }
            },
            off + 1,
        ))
    }

    fn parse_datetime(text: &'a str) -> Option<(Datetime<'a>, Option<Datetime<'a>>)> {
        if text.is_empty()
            || !text.starts_with(|c: char| c.is_ascii_digit())
            || !text.ends_with(|c: char| c.is_ascii_alphanumeric())
        {
            return None;
        }

        let mut words = text.split_ascii_whitespace();

        let date = words.next().filter(|word| {
            let word = word.as_bytes();
            // YYYY-MM-DD
            word.len() == 10
                && word[0..4].iter().all(u8::is_ascii_digit)
                && word[4] == b'-'
                && word[5..7].iter().all(u8::is_ascii_digit)
                && word[7] == b'-'
                && word[8..10].iter().all(u8::is_ascii_digit)
        })?;

        let dayname = words.next().filter(|word| {
            word.as_bytes().iter().all(|&c| {
                !(c == b'+'
                    || c == b'-'
                    || c == b']'
                    || c == b'>'
                    || c.is_ascii_digit()
                    || c == b'\n')
            })
        })?;

        let (start, end) = if let Some(word) = words.next() {
            let time = word.as_bytes();

            if (time.len() == "H:MM".len()
                && time[0].is_ascii_digit()
                && time[1] == b':'
                && time[2..4].iter().all(u8::is_ascii_digit))
                || (time.len() == "HH:MM".len()
                    && time[0..2].iter().all(u8::is_ascii_digit)
                    && time[2] == b':'
                    && time[3..5].iter().all(u8::is_ascii_digit))
            {
                (
                    Datetime {
                        date,
                        dayname,
                        time: Some(word),
                    },
                    None,
                )
            } else if time.len() == "H:MM-H:MM".len()
                && time[0].is_ascii_digit()
                && time[1] == b':'
                && time[2..4].iter().all(u8::is_ascii_digit)
                && time[4] == b'-'
                && time[5].is_ascii_digit()
                && time[6] == b':'
                && time[7..9].iter().all(u8::is_ascii_digit)
            {
                (
                    Datetime {
                        date,
                        dayname,
                        time: Some(&word[0..4]),
                    },
                    Some(Datetime {
                        date,
                        dayname,
                        time: Some(&word[5..9]),
                    }),
                )
            } else if time.len() == "H:MM-HH:MM".len()
                && time[0].is_ascii_digit()
                && time[1] == b':'
                && time[2..4].iter().all(u8::is_ascii_digit)
                && time[4] == b'-'
                && time[5..7].iter().all(u8::is_ascii_digit)
                && time[7] == b':'
                && time[8..10].iter().all(u8::is_ascii_digit)
            {
                (
                    Datetime {
                        date,
                        dayname,
                        time: Some(&word[0..4]),
                    },
                    Some(Datetime {
                        date,
                        dayname,
                        time: Some(&word[5..10]),
                    }),
                )
            } else if time.len() == "HH:MM-H:MM".len()
                && time[0..2].iter().all(u8::is_ascii_digit)
                && time[2] == b':'
                && time[3..5].iter().all(u8::is_ascii_digit)
                && time[5] == b'-'
                && time[6].is_ascii_digit()
                && time[7] == b':'
                && time[8..10].iter().all(u8::is_ascii_digit)
            {
                (
                    Datetime {
                        date,
                        dayname,
                        time: Some(&word[0..5]),
                    },
                    Some(Datetime {
                        date,
                        dayname,
                        time: Some(&word[6..10]),
                    }),
                )
            } else if time.len() == "HH:MM-HH:MM".len()
                && time[0..2].iter().all(u8::is_ascii_digit)
                && time[2] == b':'
                && time[3..5].iter().all(u8::is_ascii_digit)
                && time[5] == b'-'
                && time[6..8].iter().all(u8::is_ascii_digit)
                && time[8] == b':'
                && time[9..11].iter().all(u8::is_ascii_digit)
            {
                (
                    Datetime {
                        date,
                        dayname,
                        time: Some(&word[0..5]),
                    },
                    Some(Datetime {
                        date,
                        dayname,
                        time: Some(&word[6..11]),
                    }),
                )
            } else {
                return None;
            }
        } else {
            (
                Datetime {
                    date,
                    dayname,
                    time: None,
                },
                None,
            )
        };

        // TODO: repeater and delay
        if words.next().is_some() {
            None
        } else {
            Some((start, end))
        }
    }

    pub(crate) fn parse_diary(text: &str) -> Option<(Timestamp<'_>, usize)> {
        debug_assert!(text.starts_with('<'));

        if text.len() <= "<%%()>".len() || &text[1..4] != "%%(" {
            return None;
        }

        let bytes = text.as_bytes();

        memchr(b'>', bytes)
            .filter(|i| {
                bytes[i - 1] == b')' && bytes["<%%(".len()..i - 1].iter().all(|&c| c != b'\n')
            })
            .map(|i| (Timestamp::Diary(&text["<%%(".len()..i - 1]), i))
    }
}

#[test]
fn parse_range() {
    use super::*;

    assert_eq!(
        Timestamp::parse_inactive("[2003-09-16 Tue]"),
        Some((
            Timestamp::Inactive {
                start: Datetime {
                    date: "2003-09-16",
                    time: None,
                    dayname: "Tue"
                },
                repeater: None,
                delay: None,
            },
            "[2003-09-16 Tue]".len()
        ))
    );
    assert_eq!(
        Timestamp::parse_inactive("[2003-09-16 Tue 09:39]--[2003-09-16 Tue 10:39]"),
        Some((
            Timestamp::InactiveRange {
                start: Datetime {
                    date: "2003-09-16",
                    time: Some("09:39"),
                    dayname: "Tue"
                },
                end: Datetime {
                    date: "2003-09-16",
                    time: Some("10:39"),
                    dayname: "Tue"
                },
                repeater: None,
                delay: None
            },
            "[2003-09-16 Tue 09:39]--[2003-09-16 Tue 10:39]".len()
        ))
    );
    assert_eq!(
        Timestamp::parse_active("<2003-09-16 Tue 09:39-10:39>"),
        Some((
            Timestamp::ActiveRange {
                start: Datetime {
                    date: "2003-09-16",
                    time: Some("09:39"),
                    dayname: "Tue"
                },
                end: Datetime {
                    date: "2003-09-16",
                    time: Some("10:39"),
                    dayname: "Tue"
                },
                repeater: None,
                delay: None
            },
            "<2003-09-16 Tue 09:39-10:39>".len()
        ))
    );
}

#[test]
fn parse_datetime() {
    use super::*;

    assert_eq!(
        Timestamp::parse_datetime("2003-09-16 Tue"),
        Some((
            Datetime {
                date: "2003-09-16",
                time: None,
                dayname: "Tue"
            },
            None
        ))
    );
    assert_eq!(
        Timestamp::parse_datetime("2003-09-16  Tue 9:39"),
        Some((
            Datetime {
                date: "2003-09-16",
                time: Some("9:39"),
                dayname: "Tue"
            },
            None
        ))
    );
    assert_eq!(
        Timestamp::parse_datetime("2003-09-16 Tue  09:39"),
        Some((
            Datetime {
                date: "2003-09-16",
                time: Some("09:39"),
                dayname: "Tue"
            },
            None
        ))
    );
    assert_eq!(
        Timestamp::parse_datetime("2003-09-16 Tue 9:39-10:39"),
        Some((
            Datetime {
                date: "2003-09-16",
                time: Some("9:39"),
                dayname: "Tue"
            },
            Some(Datetime {
                date: "2003-09-16",
                time: Some("10:39"),
                dayname: "Tue"
            }),
        ))
    );

    assert_eq!(Timestamp::parse_datetime("2003-9-16 Tue"), None);
    assert_eq!(Timestamp::parse_datetime("2003-09-16"), None);
    assert_eq!(Timestamp::parse_datetime("2003-09-16 09:39"), None);
    assert_eq!(Timestamp::parse_datetime("2003-09-16 Tue 0939"), None);
}
