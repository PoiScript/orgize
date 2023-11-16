use nom::{AsBytes, IResult, InputLength, InputTake};

use super::{
    combinator::GreenElement,
    cookie::cookie_node,
    emphasis::{bold_node, code_node, italic_node, strike_node, underline_node, verbatim_node},
    fn_ref::fn_ref_node,
    inline_call::inline_call_node,
    inline_src::inline_src_node,
    input::Input,
    link::link_node,
    macros::macros_node,
    radio_target::radio_target_node,
    snippet::snippet_node,
    target::target_node,
    timestamp::{timestamp_active_node, timestamp_diary_node, timestamp_inactive_node},
};

struct ObjectPositions<'a> {
    input: Input<'a>,
    pos: usize,
    next: Option<usize>,
    finder: jetscii::BytesConst,
}

impl ObjectPositions<'_> {
    fn new(input: Input) -> ObjectPositions {
        ObjectPositions {
            input,
            pos: 0,
            next: Some(0),
            finder: jetscii::bytes!(b'@', b'<', b'[', b' ', b'(', b'{', b'\'', b'"', b'\n'),
        }
    }
}

impl<'a> Iterator for ObjectPositions<'a> {
    type Item = (Input<'a>, Input<'a>);

    fn next(&mut self) -> Option<Self::Item> {
        if self.input.input_len() < 3 {
            return None;
        }

        if let Some(p) = self.next.take() {
            return Some(self.input.take_split(p));
        }

        if self.pos >= self.input.input_len() {
            return None;
        }

        let bytes = &self.input.as_bytes()[self.pos..];
        let previous = self.pos;
        let i = self.finder.find(bytes)?;
        self.pos += i + 1;

        let p = match bytes[i] {
            b'{' => {
                self.next = Some(self.pos);
                self.pos - 1
            }
            b' ' | b'(' | b'\'' | b'"' | b'\n' => self.pos,
            _ => self.pos - 1,
        };

        debug_assert!(
            previous < self.pos && self.pos <= self.input.s.len(),
            "{} < {} < {}",
            previous,
            self.pos,
            self.input.s.len()
        );

        // a valid object requires at least three characters
        if self.input.s.len() - p < 3 {
            return None;
        }

        Some(self.input.take_split(p))
    }
}

pub fn object_nodes(input: Input) -> Vec<GreenElement> {
    // TODO:
    // debug_assert!(!input.is_empty());

    let mut i = input;
    let mut nodes = vec![];

    'l: while !i.is_empty() {
        for (input, head) in ObjectPositions::new(i) {
            debug_assert!(
                input.s.len() >= 3,
                "object must have at least three characters: {:?}",
                input.s
            );
            if let Ok((input, node)) = object_node(input) {
                if !head.is_empty() {
                    nodes.push(head.text_token())
                }
                nodes.push(node);
                debug_assert!(
                    input.input_len() < i.input_len(),
                    "{} < {}",
                    input.input_len(),
                    i.input_len()
                );
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

/// Recognizes an org-mode element expect text
fn object_node(i: Input) -> IResult<Input, GreenElement, ()> {
    match &i.as_bytes()[0] {
        b'*' => bold_node(i),
        b'+' => strike_node(i),
        b'/' => italic_node(i),
        b'_' => underline_node(i),
        b'=' => verbatim_node(i),
        b'~' => code_node(i),
        b'@' => snippet_node(i),
        b'{' => macros_node(i),
        b'<' => radio_target_node(i)
            .or_else(|_| target_node(i))
            .or_else(|_| timestamp_diary_node(i))
            .or_else(|_| timestamp_active_node(i)),
        b'[' => cookie_node(i)
            .or_else(|_| link_node(i))
            .or_else(|_| fn_ref_node(i))
            .or_else(|_| timestamp_inactive_node(i)),
        b'c' => inline_call_node(i),
        b's' => inline_src_node(i),
        _ => Err(nom::Err::Error(())),
    }
}

#[test]
fn positions() {
    let config = crate::ParseConfig::default();

    let vec = ObjectPositions::new(("*{", &config).into()).collect::<Vec<_>>();
    assert!(vec.is_empty());

    let vec = ObjectPositions::new(("*{()}//s\nc<<", &config).into()).collect::<Vec<_>>();
    assert_eq!(vec.len(), 5);
    assert_eq!(vec[0].0.s, "*{()}//s\nc<<");
    assert_eq!(vec[1].0.s, "{()}//s\nc<<");
    assert_eq!(vec[2].0.s, "()}//s\nc<<");
    assert_eq!(vec[3].0.s, ")}//s\nc<<");
    assert_eq!(vec[4].0.s, "c<<");
}

#[test]
fn parse() {
    use crate::{
        syntax::{combinator::node, SyntaxKind, SyntaxNode},
        ParseConfig,
    };

    let t = |input: &str| {
        let config = &ParseConfig::default();
        let children = object_nodes((input, config).into());
        SyntaxNode::new_root(node(SyntaxKind::PARAGRAPH, children).into_node().unwrap())
    };

    insta::assert_debug_snapshot!(
        t("~org-inlinetask-min-level~[fn:oiml:The default value of \n~org-inlinetask-min-level~ is =15=.]"),
        @r###"
    PARAGRAPH@0..93
      CODE@0..26
        TILDE@0..1 "~"
        TEXT@1..25 "org-inlinetask-min-level"
        TILDE@25..26 "~"
      FN_REF@26..93
        L_BRACKET@26..27 "["
        TEXT@27..29 "fn"
        COLON@29..30 ":"
        TEXT@30..34 "oiml"
        COLON@34..35 ":"
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
    "###
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
}
