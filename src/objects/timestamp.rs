use memchr::memchr;

#[cfg_attr(test, derive(PartialEq))]
#[derive(Debug)]
pub struct Datetime {
    pub date: (u16, u8, u8),
    pub time: Option<(u8, u8)>,
}

#[cfg_attr(test, derive(PartialEq))]
#[derive(Debug)]
pub enum RepeaterType {
    Cumulate,
    CatchUp,
    Restart,
}

#[cfg_attr(test, derive(PartialEq))]
#[derive(Debug)]
pub enum DelayType {
    All,
    First,
}

#[cfg_attr(test, derive(PartialEq))]
#[derive(Debug)]
pub enum TimeUnit {
    Hour,
    Day,
    Week,
    Month,
    Year,
}

#[cfg_attr(test, derive(PartialEq))]
#[derive(Debug)]
pub struct Repeater {
    pub ty: RepeaterType,
    pub value: usize,
    pub unit: TimeUnit,
}

#[cfg_attr(test, derive(PartialEq))]
#[derive(Debug)]
pub struct Delay {
    pub ty: DelayType,
    pub value: usize,
    pub unit: TimeUnit,
}

#[cfg_attr(test, derive(PartialEq))]
#[derive(Debug)]
pub enum Timestamp<'a> {
    Active {
        start: Datetime,
        repeater: Option<Repeater>,
        delay: Option<Delay>,
    },
    Inactive {
        start: Datetime,
        repeater: Option<Repeater>,
        delay: Option<Delay>,
    },
    ActiveRange {
        start: Datetime,
        end: Datetime,
        repeater: Option<Repeater>,
        delay: Option<Delay>,
    },
    InactiveRange {
        start: Datetime,
        end: Datetime,
        repeater: Option<Repeater>,
        delay: Option<Delay>,
    },
    Diary(&'a str),
}

pub fn parse_active(text: &str) -> Option<(Timestamp<'_>, usize)> {
    debug_assert!(text.starts_with('<'));

    let bytes = text.as_bytes();
    let mut off = memchr(b'>', bytes)?;
    let (start, mut end) = parse_datetime(&bytes[1..off])?;
    if end.is_none()
        && off <= text.len() - 14 /* --<YYYY-MM-DD> */
        && text[off + 1..].starts_with("--<")
    {
        if let Some(new_off) = memchr(b'>', &bytes[off + 1..]) {
            if let Some((start, _)) = parse_datetime(&bytes[off + 4..off + 1 + new_off]) {
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

pub fn parse_inactive(text: &str) -> Option<(Timestamp<'_>, usize)> {
    debug_assert!(text.starts_with('['));

    let bytes = text.as_bytes();
    let mut off = memchr(b']', bytes)?;
    let (start, mut end) = parse_datetime(&bytes[1..off])?;
    if end.is_none()
        && off <= text.len() - 14 /* --[YYYY-MM-DD] */
        && text[off + 1..].starts_with("--[")
    {
        if let Some(new_off) = memchr(b']', &bytes[off + 1..]) {
            if let Some((start, _)) = parse_datetime(&bytes[off + 4..off + 1 + new_off]) {
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

fn parse_datetime(bytes: &[u8]) -> Option<(Datetime, Option<Datetime>)> {
    if !bytes[0].is_ascii_digit() || !bytes[bytes.len() - 1].is_ascii_alphanumeric() {
        return None;
    }

    // similar to str::split_ascii_whitespace, but for &[u8]
    let mut words = bytes
        .split(u8::is_ascii_whitespace)
        .filter(|s| !s.is_empty());

    let date = words
        .next()
        .filter(|word| {
            word.len() == 10 /* YYYY-MM-DD */
            && word[0..4].iter().all(u8::is_ascii_digit)
            && word[4] == b'-'
            && word[5..7].iter().all(u8::is_ascii_digit)
            && word[7] == b'-'
            && word[8..10].iter().all(u8::is_ascii_digit)
        })
        .map(|word| {
            (
                (u16::from(word[0]) - u16::from(b'0')) * 1000
                    + (u16::from(word[1]) - u16::from(b'0')) * 100
                    + (u16::from(word[2]) - u16::from(b'0')) * 10
                    + (u16::from(word[3]) - u16::from(b'0')),
                (word[5] - b'0') * 10 + (word[6] - b'0'),
                (word[8] - b'0') * 10 + (word[9] - b'0'),
            )
        })?;

    let _dayname = words.next().filter(|word| {
        word.iter().all(|&c| {
            !(c == b'+' || c == b'-' || c == b']' || c == b'>' || c.is_ascii_digit() || c == b'\n')
        })
    })?;

    let (start, end) = if let Some(word) = words.next() {
        macro_rules! datetime {
            ($a:expr, $b:expr, $c:expr) => {
                Datetime {
                    date,
                    time: Some((word[$a] - b'0', (word[$b] - b'0') * 10 + (word[$c] - b'0'))),
                }
            };
            ($a:expr, $b:expr, $c:expr, $d:expr) => {
                Datetime {
                    date,
                    time: Some((
                        (word[$a] - b'0') * 10 + (word[$b] - b'0'),
                        (word[$c] - b'0') * 10 + (word[$d] - b'0'),
                    )),
                }
            };
        }

        if word.len() == 4 // H:MM
            && word[0].is_ascii_digit()
            && word[1] == b':'
            && word[2..4].iter().all(u8::is_ascii_digit)
        {
            (datetime!(0, 2, 3), None)
        } else if word.len() == 5 // HH:MM
            && word[0..2].iter().all(u8::is_ascii_digit)
            && word[2] == b':'
            && word[3..5].iter().all(u8::is_ascii_digit)
        {
            (datetime!(0, 1, 3, 4), None)
        } else if word.len() == 9 // H:MM-H:MM
            && word[0].is_ascii_digit()
            && word[1] == b':'
            && word[2..4].iter().all(u8::is_ascii_digit)
            && word[4] == b'-'
            && word[5].is_ascii_digit()
            && word[6] == b':'
            && word[7..9].iter().all(u8::is_ascii_digit)
        {
            (datetime!(0, 2, 3), Some(datetime!(5, 7, 8)))
        } else if word.len() == 10 // H:MM-HH:MM
            && word[0].is_ascii_digit()
            && word[1] == b':'
            && word[2..4].iter().all(u8::is_ascii_digit)
            && word[4] == b'-'
            && word[5..7].iter().all(u8::is_ascii_digit)
            && word[7] == b':'
            && word[8..10].iter().all(u8::is_ascii_digit)
        {
            (datetime!(0, 2, 3), Some(datetime!(5, 6, 8, 9)))
        } else if word.len() == 10 // HH:MM-H:MM
            && word[0..2].iter().all(u8::is_ascii_digit)
            && word[2] == b':'
            && word[3..5].iter().all(u8::is_ascii_digit)
            && word[5] == b'-'
            && word[6].is_ascii_digit()
            && word[7] == b':'
            && word[8..10].iter().all(u8::is_ascii_digit)
        {
            (datetime!(0, 1, 3, 4), Some(datetime!(6, 8, 9)))
        } else if word.len() == 11 // HH:MM-HH:MM
            && word[0..2].iter().all(u8::is_ascii_digit)
            && word[2] == b':'
            && word[3..5].iter().all(u8::is_ascii_digit)
            && word[5] == b'-'
            && word[6..8].iter().all(u8::is_ascii_digit)
            && word[8] == b':'
            && word[9..11].iter().all(u8::is_ascii_digit)
        {
            (datetime!(0, 1, 3, 4), Some(datetime!(6, 7, 9, 10)))
        } else {
            return None;
        }
    } else {
        (Datetime { date, time: None }, None)
    };

    // TODO: repeater and delay
    if words.next().is_some() {
        None
    } else {
        Some((start, end))
    }
}

pub fn parse_diary(text: &str) -> Option<(Timestamp<'_>, usize)> {
    debug_assert!(text.starts_with('<'));

    if text.len() <= 6 /* <%%()> */ || &text[1..4] != "%%(" {
        return None;
    }

    let bytes = text.as_bytes();

    memchr(b'>', bytes)
        .filter(|i| bytes[i - 1] == b')' && bytes[4..i - 1].iter().all(|&c| c != b'\n'))
        .map(|i| (Timestamp::Diary(&text[4..i - 1]), i))
}

#[cfg(test)]
mod tests {
    #[test]
    fn parse_range() {
        use super::*;

        assert_eq!(
            parse_inactive("[2003-09-16 Tue]"),
            Some((
                Timestamp::Inactive {
                    start: Datetime {
                        date: (2003, 9, 16),
                        time: None
                    },
                    repeater: None,
                    delay: None,
                },
                "[2003-09-16 Tue]".len()
            ))
        );
        assert_eq!(
            parse_inactive("[2003-09-16 Tue 09:39]--[2003-09-16 Tue 10:39]"),
            Some((
                Timestamp::InactiveRange {
                    start: Datetime {
                        date: (2003, 9, 16),
                        time: Some((9, 39))
                    },
                    end: Datetime {
                        date: (2003, 9, 16),
                        time: Some((10, 39))
                    },
                    repeater: None,
                    delay: None
                },
                "[2003-09-16 Tue 09:39]--[2003-09-16 Tue 10:39]".len()
            ))
        );
        assert_eq!(
            parse_active("<2003-09-16 Tue 09:39-10:39>"),
            Some((
                Timestamp::ActiveRange {
                    start: Datetime {
                        date: (2003, 9, 16),
                        time: Some((9, 39))
                    },
                    end: Datetime {
                        date: (2003, 9, 16),
                        time: Some((10, 39))
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
            parse_datetime(b"2003-09-16 Tue"),
            Some((
                Datetime {
                    date: (2003, 9, 16),
                    time: None
                },
                None
            ))
        );
        assert_eq!(
            parse_datetime(b"2003-09-16  Tue 9:39"),
            Some((
                Datetime {
                    date: (2003, 9, 16),
                    time: Some((9, 39))
                },
                None
            ))
        );
        assert_eq!(
            parse_datetime(b"2003-09-16 Tue  09:39"),
            Some((
                Datetime {
                    date: (2003, 9, 16),
                    time: Some((9, 39))
                },
                None
            ))
        );
        assert_eq!(
            parse_datetime(b"2003-09-16 Tue 9:39-10:39"),
            Some((
                Datetime {
                    date: (2003, 9, 16),
                    time: Some((9, 39))
                },
                Some(Datetime {
                    date: (2003, 9, 16),
                    time: Some((10, 39))
                }),
            ))
        );

        assert_eq!(parse_datetime(b"2003-9-16 Tue"), None);
        assert_eq!(parse_datetime(b"2003-09-16"), None);
        assert_eq!(parse_datetime(b"2003-09-16 09:39"), None);
        assert_eq!(parse_datetime(b"2003-09-16 Tue 0939"), None);
    }
}
