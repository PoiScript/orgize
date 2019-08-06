#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq))]
#[cfg_attr(feature = "ser", derive(serde::Serialize))]
#[cfg_attr(feature = "ser", serde(tag = "table_type"))]
pub enum Table<'a> {
    #[cfg_attr(feature = "ser", serde(rename = "org"))]
    Org { tblfm: Option<&'a str> },
    #[cfg_attr(feature = "ser", serde(rename = "table.el"))]
    TableEl { value: &'a str },
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
