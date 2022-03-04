use std::borrow::Cow;

use nom::{
    error::{make_error, ErrorKind},
    Err, IResult,
};

use crate::parse::combinators::{blank_lines_count, line, lines_while};

/// Table Element
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "ser", derive(serde::Serialize))]
#[cfg_attr(feature = "ser", serde(tag = "table_type"))]
pub enum Table<'a> {
    /// "org" type table
    #[cfg_attr(feature = "ser", serde(rename = "org"))]
    Org {
        #[cfg_attr(feature = "ser", serde(skip_serializing_if = "Option::is_none"))]
        tblfm: Option<Cow<'a, str>>,
        /// Numbers of blank lines between last table's line and next non-blank
        /// line or buffer's end
        post_blank: usize,
        has_header: bool,
    },
    /// "table.el" type table
    #[cfg_attr(feature = "ser", serde(rename = "table.el"))]
    TableEl {
        value: Cow<'a, str>,
        /// Numbers of blank lines between last table's line and next non-blank
        /// line or buffer's end
        post_blank: usize,
    },
}

impl Table<'_> {
    pub fn parse_table_el(input: &str) -> Option<(&str, Table)> {
        Self::parse_table_el_internal(input).ok()
    }

    fn parse_table_el_internal(input: &str) -> IResult<&str, Table, ()> {
        let (_, first_line) = line(input)?;

        let first_line = first_line.trim();

        // Table.el tables start at lines beginning with "+-" string and followed by plus or minus signs
        if !first_line.starts_with("+-")
            || first_line
                .as_bytes()
                .iter()
                .any(|&c| c != b'+' && c != b'-')
        {
            // TODO: better error kind
            return Err(Err::Error(make_error(input, ErrorKind::Many0)));
        }

        // Table.el tables end at the first line not starting with either a vertical line or a plus sign.
        let (input, content) = lines_while(|line| {
            let line = line.trim_start();
            line.starts_with('|') || line.starts_with('+')
        })(input)?;

        let (input, post_blank) = blank_lines_count(input)?;

        Ok((
            input,
            Table::TableEl {
                value: content.into(),
                post_blank,
            },
        ))
    }

    pub fn into_owned(self) -> Table<'static> {
        match self {
            Table::Org {
                tblfm,
                post_blank,
                has_header,
            } => Table::Org {
                tblfm: tblfm.map(Into::into).map(Cow::Owned),
                post_blank,
                has_header,
            },
            Table::TableEl { value, post_blank } => Table::TableEl {
                value: value.into_owned().into(),
                post_blank,
            },
        }
    }
}

/// Table Row Element
///
/// # Syntax
///
/// ```text
/// |   0 |   1 |   2 | <- TableRow::Body
/// |   0 |   1 |   2 | <- TableRow::Body
/// ```
///
/// ```text
/// |-----+-----+-----| <- ignores
/// |   0 |   1 |   2 | <- TableRow::Header
/// |   0 |   1 |   2 | <- TableRow::Header
/// |-----+-----+-----| <- TableRow::HeaderRule
/// |   0 |   1 |   2 | <- TableRow::Body
/// |-----+-----+-----| <- TableRow::BodyRule
/// |   0 |   1 |   2 | <- TableRow::Body
/// |-----+-----+-----| <- TableRow::BodyRule
/// |-----+-----+-----| <- TableRow::BodyRule
/// |   0 |   1 |   2 | <- TableRow::Body
/// |-----+-----+-----| <- ignores
/// ```
///
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "ser", derive(serde::Serialize))]
#[cfg_attr(feature = "ser", serde(tag = "table_row_type"))]
#[cfg_attr(feature = "ser", serde(rename_all = "kebab-case"))]
pub enum TableRow {
    /// This row is part of table header
    Header,
    /// This row is part of table body
    Body,
    /// This row is between table header and body
    HeaderRule,
    /// This row is between table body and next body
    BodyRule,
}

/// Table Cell Element
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "ser", derive(serde::Serialize))]
#[cfg_attr(feature = "ser", serde(tag = "table_cell_type"))]
#[cfg_attr(feature = "ser", serde(rename_all = "kebab-case"))]
pub enum TableCell {
    /// Header cell
    Header,
    /// Body cell, or standard cell
    Body,
}

#[test]
fn parse_table_el_() {
    assert_eq!(
        Table::parse_table_el(
            r#"  +---+
  |   |
  +---+

"#
        ),
        Some((
            "",
            Table::TableEl {
                value: r#"  +---+
  |   |
  +---+
"#
                .into(),
                post_blank: 1
            }
        ))
    );
    assert!(Table::parse_table_el("").is_none());
    assert!(Table::parse_table_el("+----|---").is_none());
}
