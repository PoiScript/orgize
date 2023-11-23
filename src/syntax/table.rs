use nom::{
    bytes::complete::take_while,
    character::complete::{multispace0, space0},
    combinator::iterator,
    sequence::tuple,
    AsBytes, Err, IResult, InputTake, Slice,
};

use super::{
    combinator::{blank_lines, line_ends_iter, node, pipe_token, GreenElement, NodeBuilder},
    input::Input,
    object::standard_object_nodes,
    SyntaxKind::*,
};

fn org_table_node_base(input: Input) -> IResult<Input, GreenElement, ()> {
    let mut children = vec![];

    let mut start = 0;
    for i in line_ends_iter(input.as_str()) {
        let line = input.slice(start..i);
        let trimmed = line.as_str().trim_start();

        // Org tables end at the first line not starting with a vertical bar.
        if !trimmed.starts_with('|') {
            if start == 0 {
                return Err(nom::Err::Error(()));
            } else {
                break;
            }
        }

        if trimmed.starts_with("|-") {
            children.push(node(ORG_TABLE_RULE_ROW, [line.text_token()]));
        } else {
            children.push(table_standard_row_node(line)?);
        }

        start = i;
    }

    let (input, post_blank) = blank_lines(input.slice(start..))?;

    children.extend(post_blank);

    Ok((input, node(ORG_TABLE, children)))
}

fn table_standard_row_node(input: Input) -> Result<GreenElement, nom::Err<()>> {
    let mut b = NodeBuilder::new();

    let (input, ws) = space0(input)?;

    b.ws(ws);

    let mut it = iterator(
        input,
        tuple((pipe_token, multispace0, take_while(|c: char| c != '|'))),
    );

    it.for_each(|(pipe, ws, input)| {
        b.push(pipe);
        b.ws(ws);

        if input.is_empty() {
            return;
        }

        match input
            .as_bytes()
            .iter()
            .rposition(|b| !b.is_ascii_whitespace())
        {
            Some(idx) => {
                let (ws, cell) = input.take_split(idx + 1);
                b.push(node(ORG_TABLE_CELL, standard_object_nodes(cell)));
                b.ws(ws);
            }
            _ => {
                b.push(node(ORG_TABLE_CELL, standard_object_nodes(input)));
            }
        }
    });
    it.finish()?;

    Ok(b.finish(ORG_TABLE_STANDARD_ROW))
}

fn table_el_node_base(input: Input) -> IResult<Input, GreenElement, ()> {
    let mut start = 0;
    for i in line_ends_iter(input.as_str()) {
        let line = &input.s[start..i];
        let trimmed = line.trim();

        if start == 0 {
            // Table.el tables start at lines beginning with "+-" string and followed by plus or minus signs
            if !trimmed.starts_with("+-") || trimmed.bytes().any(|c| c != b'+' && c != b'-') {
                return Err(Err::Error(()));
            }
        }

        //  Table.el tables end at the first line not starting with either a vertical line or a plus sign.
        if !trimmed.starts_with('|') && !trimmed.starts_with('+') {
            break;
        }

        start = i;
    }

    let (input, contents) = input.take_split(start);
    let (input, post_blank) = blank_lines(input)?;

    let mut children = vec![];
    children.push(contents.text_token());
    children.extend(post_blank);

    Ok((input, node(TABLE_EL, children)))
}

#[tracing::instrument(level = "debug", skip(input), fields(input = input.s))]
pub fn org_table_node(input: Input) -> IResult<Input, GreenElement, ()> {
    crate::lossless_parser!(org_table_node_base, input)
}

#[tracing::instrument(level = "debug", skip(input), fields(input = input.s))]
pub fn table_el_node(input: Input) -> IResult<Input, GreenElement, ()> {
    crate::lossless_parser!(table_el_node_base, input)
}

#[test]
fn parse_org_table() {
    use crate::{ast::OrgTable, tests::to_ast};

    let to_org_table = to_ast::<OrgTable>(org_table_node);

    insta::assert_debug_snapshot!(
        to_org_table("|").syntax,
        @r###"
    ORG_TABLE@0..1
      ORG_TABLE_STANDARD_ROW@0..1
        PIPE@0..1 "|"
    "###
    );

    insta::assert_debug_snapshot!(
        to_org_table(
r#"|
|-
|a
|-
|   a  |
"#
        ).syntax,
        @r###"
    ORG_TABLE@0..20
      ORG_TABLE_STANDARD_ROW@0..2
        PIPE@0..1 "|"
        WHITESPACE@1..2 "\n"
      ORG_TABLE_RULE_ROW@2..5
        TEXT@2..5 "|-\n"
      ORG_TABLE_STANDARD_ROW@5..8
        PIPE@5..6 "|"
        ORG_TABLE_CELL@6..7
          TEXT@6..7 "a"
        WHITESPACE@7..8 "\n"
      ORG_TABLE_RULE_ROW@8..11
        TEXT@8..11 "|-\n"
      ORG_TABLE_STANDARD_ROW@11..20
        PIPE@11..12 "|"
        WHITESPACE@12..15 "   "
        ORG_TABLE_CELL@15..16
          TEXT@15..16 "a"
        WHITESPACE@16..18 "  "
        PIPE@18..19 "|"
        WHITESPACE@19..20 "\n"
    "###
    );
}

#[test]
fn parse_table_el() {
    use crate::{ast::TableEl, tests::to_ast, ParseConfig};

    let to_table_el = to_ast::<TableEl>(table_el_node);

    insta::assert_debug_snapshot!(
        to_table_el(
            r#"  +---+
      |   |
      +---+

    "#
        ).syntax,
        @r###"
    TABLE_EL@0..37
      TEXT@0..32 "  +---+\n      |   |\n  ..."
      BLANK_LINE@32..33 "\n"
      BLANK_LINE@33..37 "    "
    "###
    );

    let config = &ParseConfig::default();

    assert!(table_el_node(("", config).into()).is_err());
    assert!(table_el_node(("+----|---", config).into()).is_err());
}
