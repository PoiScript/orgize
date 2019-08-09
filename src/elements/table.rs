use std::borrow::Cow;

use nom::{
    combinator::{peek, verify},
    IResult,
};

use crate::parsers::{line, take_lines_while};

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq))]
#[cfg_attr(feature = "ser", derive(serde::Serialize))]
#[cfg_attr(feature = "ser", serde(tag = "table_type"))]
pub enum Table<'a> {
    #[cfg_attr(feature = "ser", serde(rename = "org"))]
    Org { tblfm: Option<Cow<'a, str>> },
    #[cfg_attr(feature = "ser", serde(rename = "table.el"))]
    TableEl { value: Cow<'a, str> },
}

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq))]
#[cfg_attr(feature = "ser", derive(serde::Serialize))]
#[cfg_attr(
    feature = "ser",
    serde(tag = "table_row_type", rename_all = "kebab-case")
)]
pub enum TableRow {
    Standard,
    Rule,
}

impl TableRow {
    pub(crate) fn parse(input: &str) -> Option<TableRow> {
        if input.starts_with("|-") {
            Some(TableRow::Rule)
        } else if input.starts_with('|') {
            Some(TableRow::Standard)
        } else {
            None
        }
    }
}

pub(crate) fn parse_table_el(input: &str) -> IResult<&str, &str> {
    let (input, _) = peek(verify(line, |s: &str| {
        let s = s.trim();
        s.starts_with("+-") && s.as_bytes().iter().all(|&c| c == b'+' || c == b'-')
    }))(input)?;

    take_lines_while(|line| line.starts_with('|') || line.starts_with('+'))(input)
}

#[test]
fn parse_table_el_() {
    assert_eq!(
        parse_table_el(
            r#"+---+
|   |
+---+

"#
        ),
        Ok((
            r#"
"#,
            r#"+---+
|   |
+---+
"#
        ))
    );
    assert!(parse_table_el("").is_err());
    assert!(parse_table_el("+----|---").is_err());
}
