use crate::objects::timestamp::{self, Timestamp};
use memchr::memchr;

#[cfg_attr(test, derive(PartialEq))]
#[derive(Debug)]
pub struct Planning<'a> {
    deadline: Option<Timestamp<'a>>,
    scheduled: Option<Timestamp<'a>>,
    closed: Option<Timestamp<'a>>,
}

impl<'a> Planning<'a> {
    pub(crate) fn parse(text: &'a str) -> Option<(Planning<'a>, usize)> {
        let (text, off) = memchr(b'\n', text.as_bytes())
            .map(|i| (text[..i].trim(), i + 1))
            .unwrap_or_else(|| (text.trim(), text.len()));

        let mut words = text.split_ascii_whitespace();
        let (mut deadline, mut scheduled, mut closed) = (None, None, None);

        while let Some(word) = words.next() {
            let next = words.next()?;

            macro_rules! set_timestamp {
                ($timestamp:expr) => {
                    if $timestamp.is_none() {
                        $timestamp = if next.starts_with('<') {
                            Some(
                                timestamp::parse_active(next)
                                    .or_else(|| timestamp::parse_diary(next))?
                                    .0,
                            )
                        } else if next.starts_with('[') {
                            Some(timestamp::parse_inactive(next)?.0)
                        } else {
                            return None;
                        }
                    } else {
                        return None;
                    }
                };
            }

            match word {
                "DEADLINE:" => set_timestamp!(deadline),
                "SCHEDULED:" => set_timestamp!(scheduled),
                "CLOSED:" => set_timestamp!(closed),
                _ => (),
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
