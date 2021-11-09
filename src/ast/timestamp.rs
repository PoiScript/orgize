use super::{filter_token, Timestamp};
use crate::syntax::SyntaxKind;

impl Timestamp {
    /// ```rust
    /// use orgize::{Org, ast::Timestamp};
    ///
    /// let ts = Org::parse("<2003-09-16 Tue 09:39-10:39>").first_node::<Timestamp>().unwrap();
    /// assert!(ts.is_active());
    /// let ts = Org::parse("<2003-09-16 Tue 09:39>--<2003-09-16 Tue 10:39>").first_node::<Timestamp>().unwrap();
    /// assert!(ts.is_active());
    /// let ts = Org::parse("<2003-09-16 Tue 09:39>").first_node::<Timestamp>().unwrap();
    /// assert!(ts.is_active());
    /// ```
    pub fn is_active(&self) -> bool {
        self.syntax.kind() == SyntaxKind::TIMESTAMP_ACTIVE
    }

    /// ```rust
    /// use orgize::{Org, ast::Timestamp};
    ///
    /// let ts = Org::parse("[2003-09-16 Tue 09:39-10:39]").first_node::<Timestamp>().unwrap();
    /// assert!(ts.is_inactive());
    /// let ts = Org::parse("[2003-09-16 Tue 09:39]--[2003-09-16 Tue 10:39]").first_node::<Timestamp>().unwrap();
    /// assert!(ts.is_inactive());
    /// let ts = Org::parse("[2003-09-16 Tue 09:39]").first_node::<Timestamp>().unwrap();
    /// assert!(ts.is_inactive());
    /// ```
    pub fn is_inactive(&self) -> bool {
        self.syntax.kind() == SyntaxKind::TIMESTAMP_INACTIVE
    }

    /// ```rust
    /// use orgize::{Org, ast::Timestamp};
    ///
    /// let ts = Org::parse("<%%(org-calendar-holiday)>").first_node::<Timestamp>().unwrap();
    /// assert!(ts.is_diary());
    /// ```
    pub fn is_diary(&self) -> bool {
        self.syntax.kind() == SyntaxKind::TIMESTAMP_DIARY
    }

    /// Returns `true` if this timestamp has a range
    ///
    /// ```rust
    /// use orgize::{Org, ast::Timestamp};
    ///
    /// let ts = Org::parse("[2003-09-16 Tue 09:39-10:39]").first_node::<Timestamp>().unwrap();
    /// assert!(ts.is_range());
    /// let ts = Org::parse("[2003-09-16 Tue 09:39]--[2003-09-16 Tue 10:39]").first_node::<Timestamp>().unwrap();
    /// assert!(ts.is_range());
    /// let ts = Org::parse("[2003-09-16 Tue 09:39]").first_node::<Timestamp>().unwrap();
    /// assert!(!ts.is_range());
    /// ```
    pub fn is_range(&self) -> bool {
        self.syntax
            .children_with_tokens()
            .filter_map(filter_token(SyntaxKind::MINUS))
            .count()
            > 2
    }

    /// Converts timestamp start to chrono NaiveDateTime
    ///
    /// ```rust
    /// use orgize::{Org, ast::Timestamp};
    /// use chrono::NaiveDateTime;
    ///
    /// let ts = Org::parse("[2003-09-16 Tue 09:39-10:39]").first_node::<Timestamp>().unwrap();
    /// assert_eq!(ts.start_to_chrono().unwrap(), "2003-09-16T09:39:00".parse::<NaiveDateTime>().unwrap());
    /// ```
    #[cfg(feature = "chrono")]
    pub fn start_to_chrono(&self) -> Option<chrono::NaiveDateTime> {
        Some(chrono::NaiveDateTime::new(
            chrono::NaiveDate::from_ymd_opt(
                self.year_start()?.text().parse().ok()?,
                self.month_start()?.text().parse().ok()?,
                self.day_start()?.text().parse().ok()?,
            )?,
            chrono::NaiveTime::from_hms_opt(
                self.hour_start()?.text().parse().ok()?,
                self.minute_start()?.text().parse().ok()?,
                0,
            )?,
        ))
    }

    /// Converts timestamp end to chrono NaiveDateTime
    ///
    /// ```rust
    /// use orgize::{Org, ast::Timestamp};
    /// use chrono::NaiveDateTime;
    ///
    /// let ts = Org::parse("[2003-09-16 Tue 09:39-10:39]").first_node::<Timestamp>().unwrap();
    /// assert_eq!(ts.end_to_chrono().unwrap(), "2003-09-16T10:39:00".parse::<NaiveDateTime>().unwrap());
    /// ```
    #[cfg(feature = "chrono")]
    pub fn end_to_chrono(&self) -> Option<chrono::NaiveDateTime> {
        Some(chrono::NaiveDateTime::new(
            chrono::NaiveDate::from_ymd_opt(
                self.year_end()?.text().parse().ok()?,
                self.month_end()?.text().parse().ok()?,
                self.day_end()?.text().parse().ok()?,
            )?,
            chrono::NaiveTime::from_hms_opt(
                self.hour_end()?.text().parse().ok()?,
                self.minute_end()?.text().parse().ok()?,
                0,
            )?,
        ))
    }
}
