use std::borrow::Cow;

use nom::{
    combinator::{peek, verify},
    error::ParseError,
    IResult,
};

use crate::parsers::{line, take_lines_while};

/// Table Elemenet
#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq))]
#[cfg_attr(feature = "ser", derive(serde::Serialize))]
#[cfg_attr(feature = "ser", serde(tag = "table_type"))]
pub enum Table<'a> {
    /// "org" type table
    #[cfg_attr(feature = "ser", serde(rename = "org"))]
    Org { tblfm: Option<Cow<'a, str>> },
    /// "table.el" type table
    #[cfg_attr(feature = "ser", serde(rename = "table.el"))]
    TableEl { value: Cow<'a, str> },
}

impl Table<'_> {
    pub fn into_owned(self) -> Table<'static> {
        match self {
            Table::Org { tblfm } => Table::Org {
                tblfm: tblfm.map(Into::into).map(Cow::Owned),
            },
            Table::TableEl { value } => Table::TableEl {
                value: value.into_owned().into(),
            },
        }
    }
}

/// Table Row Elemenet
#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq))]
#[cfg_attr(feature = "ser", derive(serde::Serialize))]
#[cfg_attr(feature = "ser", serde(tag = "table_row_type"))]
#[cfg_attr(feature = "ser", serde(rename_all = "kebab-case"))]
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

pub fn parse_table_el(input: &str) -> Option<(&str, &str)> {
    parse_table_el_internal::<()>(input).ok()
}

fn parse_table_el_internal<'a, E: ParseError<&'a str>>(
    input: &'a str,
) -> IResult<&'a str, &'a str, E> {
    let (input, _) = peek(verify(line, |s: &str| {
        let s = s.trim();
        s.starts_with("+-") && s.as_bytes().iter().all(|&c| c == b'+' || c == b'-')
    }))(input)?;

    let (input, content) =
        take_lines_while(|line| line.starts_with('|') || line.starts_with('+'))(input);

    Ok((input, content))
}

#[test]
fn parse_table_el_() {
    use nom::error::VerboseError;

    assert_eq!(
        parse_table_el_internal::<VerboseError<&str>>(
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
    assert!(parse_table_el_internal::<VerboseError<&str>>("").is_err());
    assert!(parse_table_el_internal::<VerboseError<&str>>("+----|---").is_err());
}
