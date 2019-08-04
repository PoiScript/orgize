use memchr::memchr;

use crate::elements::Timestamp;

/// palnning elements
#[cfg_attr(test, derive(PartialEq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Debug)]
pub struct Planning<'a> {
    /// the date when the task should be done
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub deadline: Option<Box<Timestamp<'a>>>,
    /// the date when you should start working on the task
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub scheduled: Option<Box<Timestamp<'a>>>,
    /// the date when the task is closed
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub closed: Option<Box<Timestamp<'a>>>,
}

impl Planning<'_> {
    #[inline]
    pub(crate) fn parse(text: &str) -> Option<(&str, Planning<'_>)> {
        let (mut deadline, mut scheduled, mut closed) = (None, None, None);
        let (mut tail, off) = memchr(b'\n', text.as_bytes())
            .map(|i| (text[..i].trim(), i + 1))
            .unwrap_or_else(|| (text.trim(), text.len()));

        while let Some(i) = memchr(b' ', tail.as_bytes()) {
            let next = &tail[i + 1..].trim_start();

            macro_rules! set_timestamp {
                ($timestamp:expr) => {
                    if $timestamp.is_none() {
                        let (new_tail, timestamp) = Timestamp::parse_active(next)
                            .or_else(|_| Timestamp::parse_inactive(next))
                            .ok()?;
                        $timestamp = Some(Box::new(timestamp));
                        tail = new_tail.trim_start();
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
                &text[off..],
                Planning {
                    deadline,
                    scheduled,
                    closed,
                },
            ))
        }
    }
}

#[test]
fn prase() {
    use crate::elements::Datetime;

    assert_eq!(
        Planning::parse("SCHEDULED: <2019-04-08 Mon>\n"),
        Some((
            "",
            Planning {
                scheduled: Some(Box::new(Timestamp::Active {
                    start: Datetime {
                        year: 2019,
                        month: 4,
                        day: 8,
                        dayname: "Mon",
                        hour: None,
                        minute: None
                    },
                    repeater: None,
                    delay: None
                })),
                deadline: None,
                closed: None,
            }
        ))
    )
}
