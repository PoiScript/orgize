use crate::objects::timestamp::Timestamp;
use memchr::memchr;

#[cfg_attr(test, derive(PartialEq))]
#[derive(Debug)]
pub struct Planning<'a> {
    pub deadline: Option<Timestamp<'a>>,
    pub scheduled: Option<Timestamp<'a>>,
    pub closed: Option<Timestamp<'a>>,
}

impl<'a> Planning<'a> {
    pub(crate) fn parse(text: &'a str) -> Option<(Planning<'a>, usize)> {
        let (mut deadline, mut scheduled, mut closed) = (None, None, None);
        let (mut tail, off) = memchr(b'\n', text.as_bytes())
            .map(|i| (text[..i].trim(), i + 1))
            .unwrap_or_else(|| (text.trim(), text.len()));

        while let Some(i) = memchr(b' ', tail.as_bytes()) {
            let next = &tail[i + 1..].trim_start();

            macro_rules! set_timestamp {
                ($timestamp:expr) => {
                    if $timestamp.is_none() {
                        if next.starts_with('<') {
                            let (timestamp, off) = Timestamp::parse_active(next)
                                .or_else(|| Timestamp::parse_diary(next))?;
                            $timestamp = Some(timestamp);
                            tail = &next[off..].trim_start();
                        } else if next.starts_with('<') {
                            let (timestamp, off) = Timestamp::parse_active(next)?;
                            $timestamp = Some(timestamp);
                            tail = &next[off..].trim_start();
                        } else {
                            return None;
                        }
                    } else {
                        return None;
                    }
                };
            }

            match &tail[..i] {
                "DEADLINE:" => set_timestamp!(deadline),
                "SCHEDULED:" => set_timestamp!(scheduled),
                "CLOSED:" => set_timestamp!(closed),
                _ => return None,
            }
        }

        if deadline.is_none() && scheduled.is_none() && closed.is_none() {
            None
        } else {
            Some((
                Planning {
                    deadline,
                    scheduled,
                    closed,
                },
                off,
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn prase() {
        use super::Planning;
        use crate::objects::timestamp::{Datetime, Timestamp};

        assert_eq!(
            Planning::parse("SCHEDULED: <2019-04-08 Mon>\n"),
            Some((
                Planning {
                    scheduled: Some(Timestamp::Active {
                        start: Datetime {
                            date: (2019, 4, 8),
                            time: None
                        },
                        repeater: None,
                        delay: None
                    }),
                    closed: None,
                    deadline: None,
                },
                "SCHEDULED: <2019-04-08 Mon>\n".len()
            ))
        )
    }
}
