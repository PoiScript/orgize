use memchr::memchr;

use crate::elements::{timestamp::parse_timestamp, Timestamp};

/// Planning element
#[cfg_attr(test, derive(PartialEq))]
#[cfg_attr(feature = "ser", derive(serde::Serialize))]
#[derive(Debug, Clone)]
pub struct Planning<'a> {
    /// Timestamp associated to deadline keyword
    #[cfg_attr(feature = "ser", serde(skip_serializing_if = "Option::is_none"))]
    pub deadline: Option<Timestamp<'a>>,
    /// Timestamp associated to scheduled keyword
    #[cfg_attr(feature = "ser", serde(skip_serializing_if = "Option::is_none"))]
    pub scheduled: Option<Timestamp<'a>>,
    /// Timestamp associated to closed keyword
    #[cfg_attr(feature = "ser", serde(skip_serializing_if = "Option::is_none"))]
    pub closed: Option<Timestamp<'a>>,
}

impl Planning<'_> {
    #[inline]
    pub(crate) fn parse(text: &str) -> Option<(&str, Planning)> {
        let (mut deadline, mut scheduled, mut closed) = (None, None, None);
        let (mut tail, off) = memchr(b'\n', text.as_bytes())
            .map(|i| (text[..i].trim(), i + 1))
            .unwrap_or_else(|| (text.trim(), text.len()));

        while let Some(i) = memchr(b' ', tail.as_bytes()) {
            let next = &tail[i + 1..].trim_start();

            macro_rules! set_timestamp {
                ($timestamp:expr) => {{
                    let (new_tail, timestamp) = parse_timestamp(next).ok()?;
                    $timestamp = Some(timestamp);
                    tail = new_tail.trim_start();
                }};
            }

            match &tail[..i] {
                "DEADLINE:" if deadline.is_none() => set_timestamp!(deadline),
                "SCHEDULED:" if scheduled.is_none() => set_timestamp!(scheduled),
                "CLOSED:" if closed.is_none() => set_timestamp!(closed),
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

    pub fn into_owned(self) -> Planning<'static> {
        Planning {
            deadline: self.deadline.map(|x| x.into_owned()),
            scheduled: self.scheduled.map(|x| x.into_owned()),
            closed: self.closed.map(|x| x.into_owned()),
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
                scheduled: Some(Timestamp::Active {
                    start: Datetime {
                        year: 2019,
                        month: 4,
                        day: 8,
                        dayname: "Mon".into(),
                        hour: None,
                        minute: None
                    },
                    repeater: None,
                    delay: None
                }),
                deadline: None,
                closed: None,
            }
        ))
    )
}
