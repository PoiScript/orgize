pub mod block;
pub mod dyn_block;
pub mod fn_def;
pub mod keyword;
pub mod list;
pub mod rule;

pub use self::block::Block;
pub use self::dyn_block::DynBlock;
pub use self::fn_def::FnDef;
pub use self::keyword::Keyword;
pub use self::list::List;
pub use self::rule::Rule;

#[cfg_attr(test, derive(PartialEq, Debug))]
pub enum Element<'a> {
    Paragraph {
        cont_end: usize,
        end: usize,
    },
    Keyword {
        key: &'a str,
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
                    return if pos == start {
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
                macro_rules! ret {
                    ($ele:expr, $off:expr) => {
                        return if pos == start {
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
                    if let Some((ident, ordered, cont_end, end)) = List::parse(&src[end..]) {
                        ret!(
                            Element::List {
                                ident,
                                ordered,
                                cont_end,
                                end
                            },
                            0
                        );
                    }
                }

                if bytes[pos] == b'\n' {
                    return (
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
                        ret!(Element::Rule, off);
                    }
                }

                if bytes[pos] == b'#' && bytes.get(pos + 1).filter(|&&b| b == b'+').is_some() {
                    if let Some((name, args, contents_beg, cont_end, end)) =
                        Block::parse(&src[pos..])
                    {
                        match name.to_uppercase().as_str() {
                            "COMMENT" => ret!(
                                Element::CommentBlock {
                                    args,
                                    cont: &src[pos + contents_beg + 1..pos + cont_end - 1],
                                },
                                pos + end
                            ),
                            "EXAMPLE" => ret!(
                                Element::ExampleBlock {
                                    args,
                                    cont: &src[pos + contents_beg + 1..pos + cont_end - 1],
                                },
                                pos + end
                            ),
                            "EXPORT" => ret!(
                                Element::ExportBlock {
                                    args,
                                    cont: &src[pos + contents_beg + 1..pos + cont_end - 1],
                                },
                                pos + end
                            ),
                            "SRC" => ret!(
                                Element::SrcBlock {
                                    args,
                                    cont: &src[pos + contents_beg + 1..pos + cont_end - 1],
                                },
                                pos + end
                            ),
                            "VERSE" => ret!(
                                Element::VerseBlock {
                                    args,
                                    cont: &src[pos + contents_beg + 1..pos + cont_end - 1],
                                },
                                pos + end
                            ),
                            "CENTER" => ret!(
                                Element::CtrBlock {
                                    args,
                                    cont_end,
                                    end,
                                },
                                pos + contents_beg
                            ),
                            "QUOTE" => ret!(
                                Element::QteBlock {
                                    args,
                                    cont_end,
                                    end,
                                },
                                pos + contents_beg
                            ),
                            _ => ret!(
                                Element::SplBlock {
                                    name,
                                    args,
                                    cont_end,
                                    end,
                                },
                                pos + contents_beg
                            ),
                        };
                    }

                    if let Some((name, args, contents_beg, cont_end, end)) =
                        DynBlock::parse(&src[pos..])
                    {
                        ret!(
                            Element::DynBlock {
                                name,
                                args,
                                cont_end,
                                end,
                            },
                            pos + contents_beg
                        )
                    }

                    if let Some((key, value, off)) = Keyword::parse(&src[pos..]) {
                        ret!(Element::Keyword { key, value }, off)
                    }
                }

                // Comment
                if bytes[pos] == b'#' && bytes.get(pos + 1).map(|&b| b == b' ').unwrap_or(false) {
                    let eol = src[pos..]
                        .find('\n')
                        .map(|i| i + pos + 1)
                        .unwrap_or_else(|| src.len());
                    ret!(Element::Comment(&src[pos + 1..eol]), eol);
                }
            }

            if let Some(off) = &src[pos..].find('\n') {
                pos += off + 1;
                // last char
                if pos == src.len() {
                    return (
                        start,
                        Some(Element::Paragraph {
                            cont_end: pos - 1,
                            end: pos,
                        }),
                        None,
                    );
                }
            } else {
                return (
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
