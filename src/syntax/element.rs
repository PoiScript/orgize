use nom::{AsBytes, IResult, InputTake};

use super::{
    block::block_node,
    clock::clock_node,
    combinator::{line_starts_iter, GreenElement},
    comment::comment_node,
    drawer::drawer_node,
    dyn_block::dyn_block_node,
    fixed_width::fixed_width_node,
    fn_def::fn_def_node,
    input::Input,
    keyword::keyword_node,
    list::list_node,
    paragraph::paragraph_nodes,
    rule::rule_node,
    table::{org_table_node, table_el_node},
};

/// Parses input into multiple element
#[tracing::instrument(skip(input), fields(input = input.s))]
pub fn element_nodes(input: Input) -> Result<Vec<GreenElement>, nom::Err<()>> {
    // TODO:
    // debug_assert!(!input.is_empty());
    let nodes = element_nodes_base(input)?;
    debug_assert_eq!(
        input.as_str(),
        nodes.iter().fold(String::new(), |s, n| s + &n.to_string()),
        "parser must be lossless"
    );
    Ok(nodes)
}

/// Parses input into multiple elements
///
/// input must not contains blank line in the beginning
fn element_nodes_base(input: Input) -> Result<Vec<GreenElement>, nom::Err<()>> {
    #[derive(PartialEq, Eq)]
    enum PreviousLine {
        None,
        BlankLine,
        AffiliatedKeyword,
        Other,
    }

    let mut children = vec![];

    let mut i = input;

    let mut previous_line = PreviousLine::None;

    'l: loop {
        for (input, head) in line_starts_iter(i.as_str()).map(|idx| i.take_split(idx)) {
            // find the first byte that's not a whitespace
            let trimmed = input.as_str().trim_start_matches(|c| c == ' ' || c == '\t');

            // if this line is an affiliated keyword, that skip it
            if is_affiliated_keyword(trimmed) {
                if previous_line == PreviousLine::BlankLine {
                    children.extend(paragraph_nodes(head)?);
                }
                previous_line = PreviousLine::AffiliatedKeyword;
                continue;
            }

            // if this line is a blank line
            if is_blank_line(trimmed) {
                if previous_line == PreviousLine::AffiliatedKeyword {
                    previous_line = PreviousLine::BlankLine;
                    if let Ok((input, node)) = keyword_node(input) {
                        if !head.is_empty() {
                            children.extend(paragraph_nodes(head)?);
                        }
                        children.push(node);
                        i = input;
                        continue 'l;
                    }
                }
                continue;
            }

            if let Ok((input, node)) = match trimmed.bytes().next() {
                Some(b'[') => fn_def_node(input),
                Some(b'0'..=b'9') | Some(b'*') => list_node(input),
                Some(b'C') => clock_node(input),
                Some(b'-') => rule_node(input).or_else(|_| list_node(input)),
                Some(b':') => drawer_node(input).or_else(|_| fixed_width_node(input)),
                Some(b'|') => org_table_node(input),
                Some(b'+') => table_el_node(input).or_else(|_| list_node(input)),
                Some(b'#') => block_node(input)
                    .or_else(|_| keyword_node(input))
                    .or_else(|_| dyn_block_node(input))
                    .or_else(|_| comment_node(input)),
                _ => Err(nom::Err::Error(())),
            } {
                if !head.is_empty() {
                    children.extend(paragraph_nodes(head)?);
                }
                children.push(node);
                i = input;
                continue 'l;
            }
        }

        break;
    }

    if !i.is_empty() {
        children.extend(paragraph_nodes(i)?);
    }

    Ok(children)
}

pub fn is_affiliated_keyword(line: &str) -> bool {
    line.starts_with("#+CAPTION:")
        || line.starts_with("#+DATA:")
        || line.starts_with("#+HEADER:")
        || line.starts_with("#+HEADERS:")
        || line.starts_with("#+LABEL:")
        || line.starts_with("#+NAME:")
        || line.starts_with("#+PLOT:")
        || line.starts_with("#+RESNAME:")
        || line.starts_with("#+RESULT:")
        || line.starts_with("#+RESULTS:")
        || line.starts_with("#+SOURCE:")
        || line.starts_with("#+SRCNAME:")
        || line.starts_with("#+TBLNAME:")
        || line.starts_with("#+ATTR_")
}

pub fn is_blank_line(line: &str) -> bool {
    matches!(line.bytes().next(), None | Some(b'\n') | Some(b'\r'))
}

pub fn element_node(input: Input) -> IResult<Input, GreenElement, ()> {
    let mut has_affiliated_keyword = false;

    for offset in line_starts_iter(input.as_str()) {
        // find the first byte that's not a whitespace
        let Some(idx) = input.as_bytes()[offset..]
            .iter()
            .position(|b| *b != b' ' && *b != b'\t')
        else {
            break;
        };

        let line = &input.as_str()[(idx + offset)..];

        // if this line is an affiliated keyword, that we skip it
        if line.starts_with("#+CAPTION:")
            || line.starts_with("#+DATA:")
            || line.starts_with("#+HEADER:")
            || line.starts_with("#+HEADERS:")
            || line.starts_with("#+LABEL:")
            || line.starts_with("#+NAME:")
            || line.starts_with("#+PLOT:")
            || line.starts_with("#+RESNAME:")
            || line.starts_with("#+RESULT:")
            || line.starts_with("#+RESULTS:")
            || line.starts_with("#+SOURCE:")
            || line.starts_with("#+SRCNAME:")
            || line.starts_with("#+TBLNAME:")
            || line.starts_with("#+ATTR_")
        {
            has_affiliated_keyword = true;
            continue;
        }

        return match input.as_bytes()[idx + offset] {
            b'[' => fn_def_node(input),
            b'0'..=b'9' | b'*' => list_node(input),
            b'C' => clock_node(input),
            b'-' => rule_node(input).or_else(|_| list_node(input)),
            b':' => drawer_node(input).or_else(|_| fixed_width_node(input)),
            b'|' => org_table_node(input),
            b'+' => table_el_node(input).or_else(|_| list_node(input)),
            b'#' => block_node(input)
                .or_else(|_| keyword_node(input))
                .or_else(|_| dyn_block_node(input))
                .or_else(|_| comment_node(input)),
            _ => Err(nom::Err::Error(())),
        };
    }

    // we find an affiliated keyword, but it's not followed by any element
    // in this case, we treat it as a simple keyword

    return Err(nom::Err::Error(()));
}

#[test]
fn parse() {
    use crate::syntax::{SyntaxKind, SyntaxNode};
    use crate::{syntax::combinator::node, ParseConfig};

    let t = |input: &str| {
        let config = &ParseConfig::default();
        let children = element_nodes((input, config).into()).unwrap();
        SyntaxNode::new_root(node(SyntaxKind::SECTION, children).into_node().unwrap())
    };

    insta::assert_debug_snapshot!(
        t(r#"a

b"#),
        @r###"
    SECTION@0..4
      PARAGRAPH@0..3
        TEXT@0..2 "a\n"
        BLANK_LINE@2..3 "\n"
      PARAGRAPH@3..4
        TEXT@3..4 "b"
    "###
    );

    insta::assert_debug_snapshot!(
        t("#+ATTR_HTML: :width 300px\n[[./img/a.jpg]]"),
        @r###"
    SECTION@0..41
      KEYWORD@0..26
        HASH_PLUS@0..2 "#+"
        TEXT@2..11 "ATTR_HTML"
        COLON@11..12 ":"
        TEXT@12..25 " :width 300px"
        NEW_LINE@25..26 "\n"
      PARAGRAPH@26..41
        LINK@26..41
          L_BRACKET2@26..28 "[["
          LINK_PATH@28..39 "./img/a.jpg"
          R_BRACKET2@39..41 "]]"
    "###
    )
}
