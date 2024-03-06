use memchr::memrchr_iter;
use nom::{
    bytes::complete::take_while1,
    character::complete::{anychar, space0},
    combinator::{map, opt},
    sequence::tuple,
    IResult, InputTake, Slice,
};

use super::{
    combinator::{
        hash_token, l_bracket_token, line_starts_iter, node, r_bracket_token, token, trim_line_end,
        GreenElement, NodeBuilder,
    },
    drawer::property_drawer_node,
    element::element_nodes,
    input::Input,
    object::standard_object_nodes,
    planning::planning_node,
    SyntaxKind::*,
};

#[cfg_attr(
    feature = "tracing",
    tracing::instrument(level = "debug", skip(input), fields(input = input.s))
)]
pub fn headline_node(input: Input) -> IResult<Input, GreenElement, ()> {
    debug_assert!(!input.is_empty());
    crate::lossless_parser!(headline_node_base, input)
}

fn headline_node_base(input: Input) -> IResult<Input, GreenElement, ()> {
    let (input, stars) = headline_stars(input)?;

    let mut b = NodeBuilder::new();

    b.token(HEADLINE_STARS, stars);

    let (input, ws) = space0(input)?;
    b.ws(ws);

    let (input, headline_keyword) = opt(headline_keyword_token)(input)?;

    if let Some((headline_keyword, ws)) = headline_keyword {
        b.push(headline_keyword);
        b.ws(ws);
    }

    let (input, headline_priority) = opt(headline_priority_node)(input)?;

    if let Some((headline_priority, ws)) = headline_priority {
        b.push(headline_priority);
        b.ws(ws);
    }

    let (input, (title_and_tags, ws_, nl)) = trim_line_end(input)?;
    let (title, tags) = opt(headline_tags_node)(title_and_tags)?;

    if !title.is_empty() {
        b.push(node(HEADLINE_TITLE, standard_object_nodes(title)));
    }
    b.push_opt(tags);
    b.ws(ws_);
    b.nl(nl);

    if input.is_empty() {
        return Ok((input, b.finish(HEADLINE)));
    }

    let (input, planning) = opt(planning_node)(input)?;
    b.push_opt(planning);

    if input.is_empty() {
        return Ok((input, b.finish(HEADLINE)));
    }

    let (input, property_drawer) = opt(property_drawer_node)(input)?;
    b.push_opt(property_drawer);

    if input.is_empty() {
        return Ok((input, b.finish(HEADLINE)));
    }

    let (input, section) = opt(section_node)(input)?;
    b.push_opt(section);

    let mut i = input;
    let current_level = stars.len();
    while !i.is_empty() {
        let next_level = i.bytes().take_while(|&c| c == b'*').count();

        if next_level <= current_level {
            break;
        }

        let (input, headline) = headline_node(i)?;
        b.push(headline);
        debug_assert!(i.len() > input.len(), "{} > {}", i.len(), input.len());
        i = input;
    }

    Ok((i, b.finish(HEADLINE)))
}

#[cfg_attr(
    feature = "tracing",
    tracing::instrument(level = "debug", skip(input), fields(input = input.s))
)]
pub fn section_node(input: Input) -> IResult<Input, GreenElement, ()> {
    debug_assert!(!input.is_empty());
    let (input, section) = section_text(input)?;
    Ok((input, node(SECTION, element_nodes(section)?)))
}

fn section_text(input: Input) -> IResult<Input, Input, ()> {
    for (input, section) in line_starts_iter(input.as_str()).map(|i| input.take_split(i)) {
        if headline_stars(input).is_ok() {
            if section.is_empty() {
                return Err(nom::Err::Error(()));
            }

            return Ok((input, section));
        }
    }

    Ok(input.take_split(input.len()))
}

#[cfg_attr(
    feature = "tracing",
    tracing::instrument(level = "debug", skip(input), fields(input = input.s))
)]
fn headline_stars(input: Input) -> IResult<Input, Input, ()> {
    let bytes = input.as_bytes();
    let level = bytes.iter().take_while(|&&c| c == b'*').count();

    if level == 0 {
        Err(nom::Err::Error(()))
    }
    // headline stars must be followed by space
    else if matches!(bytes.get(level), Some(b' ')) {
        Ok(input.take_split(level))
    } else {
        Err(nom::Err::Error(()))
    }
}

#[cfg_attr(
    feature = "tracing",
    tracing::instrument(level = "debug", skip(input), fields(input = input.s))
)]
fn headline_tags_node(input: Input) -> IResult<Input, GreenElement, ()> {
    if !input.s.ends_with(':') {
        return Err(nom::Err::Error(()));
    };

    let bytes = input.as_bytes();

    // we're going to skip to first colon, so we start from the
    // second last character
    let mut i = input.len() - 1;
    let mut can_not_be_ws = true;
    let mut children = vec![token(COLON, ":")];

    for ii in memrchr_iter(b':', bytes).skip(1) {
        let item = &bytes[ii + 1..i];

        if item.is_empty() {
            children.push(token(COLON, ":"));
            can_not_be_ws = false;
            debug_assert!(i > ii, "{} > {}", i, ii);
            i = ii;
        } else if String::from_utf8_lossy(item)
            .chars()
            // https://github.com/yyr/org-mode/blob/d8494b5668ad4d4e68e83228ae8451eaa01d2220/lisp/org-element.el#L922C25-L922C32
            .all(|c| c.is_alphanumeric() || c == '_' || c == '@' || c == '#' || c == '%')
        {
            children.push(input.slice(ii + 1..i).text_token());
            children.push(token(COLON, ":"));
            can_not_be_ws = false;
            debug_assert!(i > ii, "{} > {}", i, ii);
            i = ii;
        } else if item.iter().all(|&c| c == b' ' || c == b'\t') && !can_not_be_ws {
            children.push(input.slice(ii + 1..i).ws_token());
            children.push(token(COLON, ":"));
            can_not_be_ws = true;
            debug_assert!(i > ii, "{} > {}", i, ii);
            i = ii;
        } else {
            break;
        }
    }

    if children.len() <= 2 {
        return Err(nom::Err::Error(()));
    }

    if i != 0 && bytes[i - 1] != b' ' && bytes[i - 1] != b'\t' {
        return Err(nom::Err::Error(()));
    }

    // we parse headline tag from right to left,
    // so we need to reverse the result after it finishes
    children.reverse();

    Ok((input.slice(0..i), node(HEADLINE_TAGS, children)))
}

fn headline_keyword_token(input: Input) -> IResult<Input, (GreenElement, Input), ()> {
    let (input, word) = take_while1(|c: char| !c.is_ascii_whitespace())(input)?;
    let (input, ws) = space0(input)?;
    if input.c.todo_keywords.0.iter().any(|k| k == word.s) {
        Ok((input, (word.token(HEADLINE_KEYWORD_TODO), ws)))
    } else if input.c.todo_keywords.1.iter().any(|k| k == word.s) {
        Ok((input, (word.token(HEADLINE_KEYWORD_DONE), ws)))
    } else {
        Err(nom::Err::Error(()))
    }
}

fn headline_priority_node(input: Input) -> IResult<Input, (GreenElement, Input), ()> {
    let (input, node) = map(
        tuple((l_bracket_token, hash_token, anychar, r_bracket_token)),
        |(l_bracket, hash, char, r_bracket)| {
            node(
                HEADLINE_PRIORITY,
                [l_bracket, hash, token(TEXT, &char.to_string()), r_bracket],
            )
        },
    )(input)?;

    let (input, ws) = space0(input)?;

    Ok((input, (node, ws)))
}

#[test]
fn parse() {
    use crate::{ast::Headline, tests::to_ast, ParseConfig};

    let to_headline = to_ast::<Headline>(headline_node);

    insta::assert_debug_snapshot!(
        to_headline("* foo").syntax,
        @r###"
    HEADLINE@0..5
      HEADLINE_STARS@0..1 "*"
      WHITESPACE@1..2 " "
      HEADLINE_TITLE@2..5
        TEXT@2..5 "foo"
    "###
    );

    insta::assert_debug_snapshot!(
        to_headline("* foo\n\n** bar").syntax,
        @r###"
    HEADLINE@0..13
      HEADLINE_STARS@0..1 "*"
      WHITESPACE@1..2 " "
      HEADLINE_TITLE@2..5
        TEXT@2..5 "foo"
      NEW_LINE@5..6 "\n"
      SECTION@6..7
        PARAGRAPH@6..7
          BLANK_LINE@6..7 "\n"
      HEADLINE@7..13
        HEADLINE_STARS@7..9 "**"
        WHITESPACE@9..10 " "
        HEADLINE_TITLE@10..13
          TEXT@10..13 "bar"
    "###
    );

    insta::assert_debug_snapshot!(
        to_headline("* TODO foo\nbar\n** baz\n").syntax,
        @r###"
    HEADLINE@0..22
      HEADLINE_STARS@0..1 "*"
      WHITESPACE@1..2 " "
      HEADLINE_KEYWORD_TODO@2..6 "TODO"
      WHITESPACE@6..7 " "
      HEADLINE_TITLE@7..10
        TEXT@7..10 "foo"
      NEW_LINE@10..11 "\n"
      SECTION@11..15
        PARAGRAPH@11..15
          TEXT@11..15 "bar\n"
      HEADLINE@15..22
        HEADLINE_STARS@15..17 "**"
        WHITESPACE@17..18 " "
        HEADLINE_TITLE@18..21
          TEXT@18..21 "baz"
        NEW_LINE@21..22 "\n"
    "###
    );

    insta::assert_debug_snapshot!(
        to_headline("** [#A] foo\n* baz").syntax,
        @r###"
    HEADLINE@0..12
      HEADLINE_STARS@0..2 "**"
      WHITESPACE@2..3 " "
      HEADLINE_PRIORITY@3..7
        L_BRACKET@3..4 "["
        HASH@4..5 "#"
        TEXT@5..6 "A"
        R_BRACKET@6..7 "]"
      WHITESPACE@7..8 " "
      HEADLINE_TITLE@8..11
        TEXT@8..11 "foo"
      NEW_LINE@11..12 "\n"
    "###
    );

    let config = &ParseConfig::default();

    assert!(headline_node(("_ ", config).into()).is_err());
    assert!(headline_node(("*", config).into()).is_err());
    assert!(headline_node((" * ", config).into()).is_err());
    assert!(headline_node(("**", config).into()).is_err());
    assert!(headline_node(("**\n", config).into()).is_err());
    assert!(headline_node(("**\r", config).into()).is_err());
    assert!(headline_node(("**\t", config).into()).is_err());
}

#[test]
fn issue_15_16() {
    use crate::{ast::Headline, tests::to_ast};

    let to_headline = to_ast::<Headline>(headline_node);

    assert!(to_headline("* a ::").tags().count() == 0);
    assert!(to_headline("* a : :").tags().count() == 0);
    assert!(to_headline("* a :(:").tags().count() == 0);
    assert!(to_headline("* a :a: :").tags().count() == 0);
    assert!(to_headline("* a :a :").tags().count() == 0);
    assert!(to_headline("* a a:").tags().count() == 0);
    assert!(to_headline("* a :a").tags().count() == 0);

    let tags = to_headline("* a \t:_:").tags();
    assert_eq!(
        vec!["_".to_string()],
        tags.map(|x| x.to_string()).collect::<Vec<_>>(),
    );

    let tags = to_headline("* a \t :@:").tags();
    assert_eq!(
        vec!["@".to_string()],
        tags.map(|x| x.to_string()).collect::<Vec<_>>(),
    );

    let tags = to_headline("* a :#:").tags();
    assert_eq!(
        vec!["#".to_string()],
        tags.map(|x| x.to_string()).collect::<Vec<_>>(),
    );

    let tags = to_headline("* a\t :%:").tags();
    assert_eq!(
        vec!["%".to_string()],
        tags.map(|x| x.to_string()).collect::<Vec<_>>(),
    );

    let tags = to_headline("* a :余: :破:").tags();
    assert_eq!(
        vec!["余".to_string(), "破".to_string()],
        tags.map(|x| x.to_string()).collect::<Vec<_>>(),
    );
}
