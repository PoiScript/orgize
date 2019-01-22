pub mod block;
pub mod dyn_block;
pub mod fn_def;
pub mod keyword;
pub mod list;
pub mod rule;

pub use self::block::Block;
pub use self::dyn_block::DynBlock;
pub use self::fn_def::FnDef;
pub use self::keyword::{Key, Keyword};
pub use self::list::List;
pub use self::rule::Rule;

#[cfg_attr(test, derive(PartialEq, Debug))]
pub enum Element<'a> {
    Paragraph {
        cont_end: usize,
        end: usize,
    },
    Keyword {
        key: Key<'a>,
        value: &'a str,
    },
    Call {
        value: &'a str,
    },
    FnDef {
        label: &'a str,
        cont: &'a str,
    },
    CtrBlock {
        args: Option<&'a str>,
        cont_end: usize,
        end: usize,
    },
    QteBlock {
        args: Option<&'a str>,
        cont_end: usize,
        end: usize,
    },
    SplBlock {
        args: Option<&'a str>,
        name: &'a str,
        cont_end: usize,
        end: usize,
    },
    CommentBlock {
        args: Option<&'a str>,
        cont: &'a str,
    },
    ExampleBlock {
        args: Option<&'a str>,
        cont: &'a str,
    },
    ExportBlock {
        args: Option<&'a str>,
        cont: &'a str,
    },
    SrcBlock {
        args: Option<&'a str>,
        cont: &'a str,
    },
    VerseBlock {
        args: Option<&'a str>,
        cont: &'a str,
    },
    DynBlock {
        args: Option<&'a str>,
        name: &'a str,
        cont_end: usize,
        end: usize,
    },
    Rule,
    Comment(&'a str),
    FixedWidth(&'a str),
    List {
        ident: usize,
        ordered: bool,
        cont_end: usize,
        end: usize,
    },
}

impl<'a> Element<'a> {
    pub fn next_2(src: &'a str) -> (usize, Option<Element<'a>>, Option<(Element<'a>, usize)>) {
        let bytes = src.as_bytes();

        let mut pos = skip_empty_line!(src, 0);
        let start = pos;

        if start == src.len() {
            return (start, None, None);
        }

        loop {
            // Unlike other element, footnote definition must starts at column 0
            if bytes[pos] == b'[' {
                if let Some((label, cont, off)) = FnDef::parse(&src[pos..]) {
                    break if pos == start {
                        (off + 1, Some(Element::FnDef { label, cont }), None)
                    } else {
                        (
                            start,
                            Some(Element::Paragraph {
                                cont_end: pos - 1,
                                end: pos,
                            }),
                            Some((Element::FnDef { label, cont }, off + 1)),
                        )
                    };
                }
            }

            let end = pos;
            pos = skip_space!(src, pos);

            if pos <= src.len() {
                macro_rules! brk {
                    ($ele:expr, $off:expr) => {
                        break if pos == start {
                            ($off, Some($ele), None)
                        } else {
                            (
                                start,
                                Some(Element::Paragraph {
                                    cont_end: end,
                                    end: pos - 1,
                                }),
                                Some(($ele, $off)),
                            )
                        };
                    };
                }

                if bytes[pos] == b'+'
                    || bytes[pos] == b'-'
                    || bytes[pos] == b'*'
                    || (bytes[pos] >= b'0' && bytes[pos] <= b'9')
                {
                    if let Some((ident, ordered, cont_end, list_end)) = List::parse(&src[end..]) {
                        let list = Element::List {
                            ident,
                            ordered,
                            cont_end,
                            end: list_end,
                        };
                        break if pos == start {
                            (1, Some(list), None)
                        } else {
                            (
                                start,
                                Some(Element::Paragraph {
                                    cont_end: end,
                                    end: end,
                                }),
                                Some((list, 1)),
                            )
                        };
                    }
                }

                if bytes[pos] == b'\n' {
                    break (
                        start,
                        Some(Element::Paragraph {
                            cont_end: end,
                            end: pos,
                        }),
                        None,
                    );
                }

                // TODO: LaTeX environment
                if bytes[pos] == b'\\' {}

                // Rule
                if bytes[pos] == b'-' {
                    let off = Rule::parse(&src[pos..]);
                    if off != 0 {
                        brk!(Element::Rule, off);
                    }
                }

                // TODO: multiple lines fixed width area
                if bytes[pos] == b':'
                    && bytes
                        .get(pos + 1)
                        .map(|&b| b == b' ' || b == b'\n')
                        .unwrap_or(false)
                {
                    let eol = memchr::memchr(b'\n', &src.as_bytes()[pos..])
                        .map(|i| i + 1)
                        .unwrap_or_else(|| src.len() - pos);
                    brk!(Element::FixedWidth(&src[pos + 1..pos + eol]), eol);
                }

                if bytes[pos] == b'#' && bytes.get(pos + 1).map(|&b| b == b'+').unwrap_or(false) {
                    if let Some((name, args, cont_beg, cont_end, end)) = Block::parse(&src[pos..]) {
                        let cont = &src[pos + cont_beg + 1..pos + cont_end - 1];
                        match name.to_uppercase().as_str() {
                            "COMMENT" => brk!(Element::CommentBlock { args, cont }, end),
                            "EXAMPLE" => brk!(Element::ExampleBlock { args, cont }, end),
                            "EXPORT" => brk!(Element::ExportBlock { args, cont }, end),
                            "SRC" => brk!(Element::SrcBlock { args, cont }, end),
                            "VERSE" => brk!(Element::VerseBlock { args, cont }, end),
                            "CENTER" => brk!(
                                Element::CtrBlock {
                                    args,
                                    cont_end,
                                    end,
                                },
                                cont_beg
                            ),
                            "QUOTE" => brk!(
                                Element::QteBlock {
                                    args,
                                    cont_end,
                                    end,
                                },
                                cont_beg
                            ),
                            _ => brk!(
                                Element::SplBlock {
                                    name,
                                    args,
                                    cont_end,
                                    end
                                },
                                cont_beg
                            ),
                        };
                    }

                    if let Some((name, args, cont_beg, cont_end, end)) =
                        DynBlock::parse(&src[pos..])
                    {
                        brk!(
                            Element::DynBlock {
                                name,
                                args,
                                cont_end,
                                end,
                            },
                            cont_beg
                        )
                    }

                    if let Some((key, value, off)) = Keyword::parse(&src[pos..]) {
                        brk!(
                            if let Key::Call = key {
                                Element::Call { value }
                            } else {
                                Element::Keyword { key, value }
                            },
                            off
                        )
                    }
                }

                // Comment
                // TODO: multiple lines comment
                if bytes[pos] == b'#' && bytes.get(pos + 1).map(|&b| b == b' ').unwrap_or(false) {
                    let eol = memchr::memchr(b'\n', &src.as_bytes()[pos..])
                        .map(|i| i + 1)
                        .unwrap_or_else(|| src.len() - pos);
                    brk!(Element::Comment(&src[pos + 1..pos + eol]), eol);
                }
            }

            if let Some(off) = memchr::memchr(b'\n', &src.as_bytes()[pos..]) {
                pos += off + 1;
                // last char
                if pos == src.len() {
                    break (
                        start,
                        Some(Element::Paragraph {
                            cont_end: pos - 1,
                            end: pos,
                        }),
                        None,
                    );
                }
            } else {
                break (
                    start,
                    Some(Element::Paragraph {
                        cont_end: src.len(),
                        end: src.len(),
                    }),
                    None,
                );
            }
        }
    }
}

#[test]
fn next_2() {
    // TODO: more tests
    assert_eq!(Element::next_2("\n\n\n\n"), (4, None, None));
}
