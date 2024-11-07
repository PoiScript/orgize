use nom::{IResult, InputTake};

use super::{
    combinator::GreenElement,
    cookie::cookie_node,
    emphasis::{
        self, bold_node, code_node, italic_node, strike_node, underline_node, verbatim_node,
    },
    entity::entity_node,
    fn_ref::fn_ref_node,
    inline_call::inline_call_node,
    inline_src::inline_src_node,
    input::Input,
    latex_fragment::latex_fragment_node,
    line_break::line_break_node,
    link::link_node,
    macros::macros_node,
    radio_target::radio_target_node,
    snippet::snippet_node,
    subscript_superscript::{self, subscript_node, superscript_node},
    target::target_node,
    timestamp::{timestamp_active_node, timestamp_diary_node, timestamp_inactive_node},
};

struct ObjectPositions<'a> {
    input: Input<'a>,
    pos: usize,
    finder: jetscii::BytesConst,
}

impl ObjectPositions<'_> {
    fn standard(input: Input) -> ObjectPositions {
        ObjectPositions {
            input,
            pos: 0,
            finder: jetscii::bytes!(
                b'*', b'+', b'/', b'_', b'=', b'~', /* text markup */
                b'@', /* snippet */
                b'<', /* timestamp, target, radio target */
                b'[', /* link, cookie, fn_ref, timestamp */
                b'c', /* inline call */
                b's', /* inline source */
                b'\\', b'$', /* latex & entity */
                b'{', /* macros */
                b'^', /* superscript */
                b'_'  /* subscript */
            ),
        }
    }

    fn minimal(input: Input) -> ObjectPositions {
        ObjectPositions {
            input,
            pos: 0,
            finder: jetscii::bytes!(
                b'*', b'+', b'/', b'_', b'=', b'~', /* text markup */
                b'\\', b'$', /* latex & entity */
                b'^', /* superscript */
                b'_'  /* subscript */
            ),
        }
    }

    fn link_description(input: Input) -> ObjectPositions {
        ObjectPositions {
            input,
            pos: 0,
            finder: jetscii::bytes!(
                b'*', b'+', b'/', b'_', b'=', b'~', /* text markup */
                b'\\', b'$', /* latex & entity */
                b'@', /* snippet */
                b'c', /* inline call */
                b's', /* inline source */
                b'{', /* macros */
                b'[', /* cookie */
                b'^', /* superscript */
                b'_'  /* subscript */
            ),
        }
    }
}

impl<'a> Iterator for ObjectPositions<'a> {
    type Item = (Input<'a>, Input<'a>);

    fn next(&mut self) -> Option<Self::Item> {
        if self.input.len() < 2 || self.pos >= self.input.len() {
            return None;
        }

        let previous = self.pos;
        let i = self.finder.find(&self.input.as_bytes()[self.pos..])?;
        let p = self.pos + i;

        self.pos = p + 1;

        debug_assert!(
            previous < self.pos && self.pos <= self.input.s.len(),
            "{} < {} < {}",
            previous,
            self.pos,
            self.input.s.len()
        );

        // a valid object requires at least two characters
        if self.input.s.len() - p < 2 {
            return None;
        }

        Some(self.input.take_split(p))
    }
}

/// parse minimal sets of objects, including
/// - LaTeX fragments ('\\')
/// - Text markup (bold code strike verbatim underline italic) ('*', '~', '+', '=', '_', '/')
/// - Entities ('\\')
/// - Superscripts and Subscripts
pub fn minimal_object_nodes(input: Input) -> Vec<GreenElement> {
    object_nodes(
        ObjectPositions::minimal,
        |i: Input, pre: Input| match &i.as_bytes()[0] {
            b'*' if emphasis::verify_pre(pre.s) => bold_node(i),
            b'+' if emphasis::verify_pre(pre.s) => strike_node(i),
            b'/' if emphasis::verify_pre(pre.s) => italic_node(i),
            b'_' if emphasis::verify_pre(pre.s) => underline_node(i),
            b'=' if emphasis::verify_pre(pre.s) => verbatim_node(i),
            b'~' if emphasis::verify_pre(pre.s) => code_node(i),
            b'$' => latex_fragment_node(i),
            b'\\' => entity_node(i).or_else(|_| latex_fragment_node(i)),
            b'^' if subscript_superscript::verify_pre(&pre) => superscript_node(i),
            b'_' if subscript_superscript::verify_pre(&pre) => subscript_node(i),
            _ => Err(nom::Err::Error(())),
        },
        input,
    )
}

/// parses standard sets of objects, including
///
/// - Entities
/// - LaTeX Fragments
/// - Export Snippets
/// - Footnote References
/// - Inline Babel Calls
/// - Inline Source Blocks
/// - Links
/// - Macros
/// - Targets and Radio Targets
/// - Statistics Cookies
/// - Timestamps
/// - Text Markup (bold code strike verbatim underline italic)
/// - Line Breaks
/// - Subscript and Superscript
/// - Cloze (if `syntax-org-fc` is enabled)
///
/// // todo:
/// - Citations
pub fn standard_object_nodes(input: Input) -> Vec<GreenElement> {
    object_nodes(
        ObjectPositions::standard,
        |i: Input, pre: Input| match &i.as_bytes()[0] {
            b'*' if emphasis::verify_pre(pre.s) => bold_node(i),
            b'+' if emphasis::verify_pre(pre.s) => strike_node(i),
            b'/' if emphasis::verify_pre(pre.s) => italic_node(i),
            b'_' if emphasis::verify_pre(pre.s) => underline_node(i),
            b'=' if emphasis::verify_pre(pre.s) => verbatim_node(i),
            b'~' if emphasis::verify_pre(pre.s) => code_node(i),
            b'@' => snippet_node(i),
            b'{' => {
                cfg_if::cfg_if! {
                    if #[cfg(feature = "syntax-org-fc")] {
                        macros_node(i).or_else(|_| super::cloze::cloze_node(i))
                    } else {
                        macros_node(i)
                    }
                }
            }
            b'<' => radio_target_node(i)
                .or_else(|_| target_node(i))
                .or_else(|_| timestamp_diary_node(i))
                .or_else(|_| timestamp_active_node(i)),
            b'[' => cookie_node(i)
                .or_else(|_| link_node(i))
                .or_else(|_| fn_ref_node(i))
                .or_else(|_| timestamp_inactive_node(i)),
            // NOTE: although not specified in document, inline call and inline src follows the
            // same pre tokens rule as text markup
            b'c' if emphasis::verify_pre(pre.s) => inline_call_node(i),
            b's' if emphasis::verify_pre(pre.s) => inline_src_node(i),
            b'$' => latex_fragment_node(i),
            b'\\' if !pre.s.ends_with('\\') && i.as_bytes()[1] == b'\\' => line_break_node(i),
            b'\\' => entity_node(i).or_else(|_| latex_fragment_node(i)),
            b'^' if subscript_superscript::verify_pre(&pre) => superscript_node(i),
            b'_' if subscript_superscript::verify_pre(&pre) => subscript_node(i),
            _ => Err(nom::Err::Error(())),
        },
        input,
    )
}

pub fn link_description_object_nodes(input: Input) -> Vec<GreenElement> {
    object_nodes(
        ObjectPositions::link_description,
        |i: Input<'_>, pre: Input<'_>| match &i.as_bytes()[0] {
            b'@' => snippet_node(i),
            b'c' if emphasis::verify_pre(pre.s) => inline_call_node(i),
            b's' if emphasis::verify_pre(pre.s) => inline_src_node(i),
            b'{' => macros_node(i),
            b'[' => cookie_node(i),
            b'*' if emphasis::verify_pre(pre.s) => bold_node(i),
            b'+' if emphasis::verify_pre(pre.s) => strike_node(i),
            b'/' if emphasis::verify_pre(pre.s) => italic_node(i),
            b'_' if emphasis::verify_pre(pre.s) => underline_node(i),
            b'=' if emphasis::verify_pre(pre.s) => verbatim_node(i),
            b'~' if emphasis::verify_pre(pre.s) => code_node(i),
            b'$' => latex_fragment_node(i),
            b'\\' => entity_node(i).or_else(|_| latex_fragment_node(i)),
            b'^' if subscript_superscript::verify_pre(&pre) => superscript_node(i),
            b'_' if subscript_superscript::verify_pre(&pre) => subscript_node(i),
            _ => Err(nom::Err::Error(())),
        },
        input,
    )
}

fn object_nodes<'a, F, P>(position: F, parse: P, input: Input<'a>) -> Vec<GreenElement>
where
    F: Fn(Input) -> ObjectPositions,
    P: Fn(Input<'a>, Input<'a>) -> IResult<Input<'a>, GreenElement, ()>,
{
    let mut i = input;
    let mut nodes = vec![];

    'l: while !i.is_empty() {
        for (input, head) in position(i) {
            debug_assert!(
                input.s.len() >= 2,
                "object must have at least two characters: {:?}",
                input.s
            );

            if let Ok((input, pre)) = parse(input, head) {
                if !head.is_empty() {
                    nodes.push(head.text_token())
                }
                nodes.push(pre);
                debug_assert!(input.len() < i.len(), "{} < {}", input.len(), i.len());
                i = input;
                continue 'l;
            }
        }
        nodes.push(i.text_token());
        break;
    }

    debug_assert_eq!(
        input.as_str(),
        nodes.iter().fold(String::new(), |s, i| s + &i.to_string()),
        "parser must be lossless"
    );

    nodes
}

#[test]
fn positions() {
    let config = crate::ParseConfig::default();

    let vec = ObjectPositions::standard(("*", &config).into()).collect::<Vec<_>>();
    assert!(vec.is_empty());

    let vec = ObjectPositions::standard(("*{", &config).into()).collect::<Vec<_>>();
    assert_eq!(vec.len(), 1);
    assert_eq!(vec[0].0.s, "*{");

    // https://github.com/PoiScript/orgize/issues/69
    let vec = ObjectPositions::standard(("{3}", &config).into()).collect::<Vec<_>>();
    assert_eq!(vec.len(), 1);
    assert_eq!(vec[0].0.s, "{3}");

    let vec = ObjectPositions::standard(("*{()}//s\nc<<", &config).into()).collect::<Vec<_>>();
    assert_eq!(vec.len(), 7);
    assert_eq!(vec[0].0.s, "*{()}//s\nc<<");
    assert_eq!(vec[1].0.s, "{()}//s\nc<<");
    assert_eq!(vec[2].0.s, "//s\nc<<");
    assert_eq!(vec[3].0.s, "/s\nc<<");
    assert_eq!(vec[4].0.s, "s\nc<<");
    assert_eq!(vec[5].0.s, "c<<");
    assert_eq!(vec[6].0.s, "<<");
}

#[test]
fn parse() {
    use crate::{
        syntax::{combinator::node, SyntaxKind, SyntaxNode},
        ParseConfig,
    };

    let t = |input: &str| {
        let config = &ParseConfig::default();
        let children = standard_object_nodes((input, config).into());
        SyntaxNode::new_root(node(SyntaxKind::PARAGRAPH, children).into_node().unwrap())
    };

    insta::assert_debug_snapshot!(
        t("~org-inlinetask-min-level~[fn:oiml:The default value of \n~org-inlinetask-min-level~ is =15=.]"),
        @r#"
    PARAGRAPH@0..93
      CODE@0..26
        TILDE@0..1 "~"
        TEXT@1..25 "org-inlinetask-min-level"
        TILDE@25..26 "~"
      FN_REF@26..93
        L_BRACKET@26..27 "["
        KEYWORD@27..29 "fn"
        COLON@29..30 ":"
        FN_LABEL@30..34 "oiml"
        COLON@34..35 ":"
        FN_CONTENT@35..92
          TEXT@35..57 "The default value of \n"
          CODE@57..83
            TILDE@57..58 "~"
            TEXT@58..82 "org-inlinetask-min-level"
            TILDE@82..83 "~"
          TEXT@83..87 " is "
          VERBATIM@87..91
            EQUAL@87..88 "="
            TEXT@88..90 "15"
            EQUAL@90..91 "="
          TEXT@91..92 "."
        R_BRACKET@92..93 "]"
    "#
    );

    insta::assert_debug_snapshot!(
      t(r#"Org is a /plaintext markup syntax/ developed with *Emacs* in 2003.
The canonical parser is =org-element.el=, which provides a number of
functions starting with ~org-element-~."#),
      @r###"
    PARAGRAPH@0..175
      TEXT@0..9 "Org is a "
      ITALIC@9..34
        SLASH@9..10 "/"
        TEXT@10..33 "plaintext markup syntax"
        SLASH@33..34 "/"
      TEXT@34..50 " developed with "
      BOLD@50..57
        STAR@50..51 "*"
        TEXT@51..56 "Emacs"
        STAR@56..57 "*"
      TEXT@57..91 " in 2003.\nThe canonic ..."
      VERBATIM@91..107
        EQUAL@91..92 "="
        TEXT@92..106 "org-element.el"
        EQUAL@106..107 "="
      TEXT@107..160 ", which provides a nu ..."
      CODE@160..174
        TILDE@160..161 "~"
        TEXT@161..173 "org-element-"
        TILDE@173..174 "~"
      TEXT@174..175 "."
    "###
    );

    insta::assert_debug_snapshot!(
        t("a^abc"),
        @r###"
    PARAGRAPH@0..5
      TEXT@0..1 "a"
      SUPERSCRIPT@1..5
        CARET@1..2 "^"
        TEXT@2..5 "abc"
    "###
    );
}
