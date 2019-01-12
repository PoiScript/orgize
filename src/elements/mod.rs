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
        // end of the contents
        end: usize,
        // trailing space
        trailing: usize,
    },
    Keyword {
        key: &'a str,
        value: &'a str,
    },
    FnDef {
        label: &'a str,
        contents: &'a str,
    },
    CenterBlock {
        args: Option<&'a str>,
        contents_end: usize,
        end: usize,
    },
    QuoteBlock {
        args: Option<&'a str>,
        contents_end: usize,
        end: usize,
    },
    SpecialBlock {
        args: Option<&'a str>,
        name: &'a str,
        contents_end: usize,
        end: usize,
    },
    CommentBlock {
        args: Option<&'a str>,
        contents: &'a str,
    },
    ExampleBlock {
        args: Option<&'a str>,
        contents: &'a str,
    },
    ExportBlock {
        args: Option<&'a str>,
        contents: &'a str,
    },
    SrcBlock {
        args: Option<&'a str>,
        contents: &'a str,
    },
    VerseBlock {
        args: Option<&'a str>,
        contents: &'a str,
    },
    DynBlock {
        args: Option<&'a str>,
        name: &'a str,
        contents_end: usize,
        end: usize,
    },
    Rule,
    Comment(&'a str),
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
                if let Some((label, contents, off)) = FnDef::parse(&src[pos..]) {
                    return if pos == start {
                        (off + 1, Some(Element::FnDef { label, contents }), None)
                    } else {
                        (
                            start,
                            Some(Element::Paragraph {
                                end: pos - 1,
                                trailing: pos,
                            }),
                            Some((Element::FnDef { label, contents }, off + 1)),
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
                                    end,
                                    trailing: pos - 1,
                                }),
                                Some(($ele, $off)),
                            )
                        };
                    };
                }

                if bytes[pos] == b'\n' {
                    return (start, Some(Element::Paragraph { end, trailing: pos }), None);
                }

                // TODO: LaTeX environment
                if bytes[pos] == b'\\' {}

                // Rule
                if bytes[pos] == b'-' {
                    if let Some(off) = Rule::parse(&src[pos..]) {
                        ret!(Element::Rule, off);
                    }
                }

                if bytes[pos] == b'#' && bytes.get(pos + 1).filter(|&&b| b == b'+').is_some() {
                    if let Some((name, args, contents_beg, contents_end, end)) =
                        Block::parse(&src[pos..])
                    {
                        match name.to_uppercase().as_str() {
                            "COMMENT" => ret!(
                                Element::CommentBlock {
                                    args,
                                    contents: &src[pos + contents_beg + 1..pos + contents_end - 1],
                                },
                                pos + end
                            ),
                            "EXAMPLE" => ret!(
                                Element::ExampleBlock {
                                    args,
                                    contents: &src[pos + contents_beg + 1..pos + contents_end - 1],
                                },
                                pos + end
                            ),
                            "EXPORT" => ret!(
                                Element::ExportBlock {
                                    args,
                                    contents: &src[pos + contents_beg + 1..pos + contents_end - 1],
                                },
                                pos + end
                            ),
                            "SRC" => ret!(
                                Element::SrcBlock {
                                    args,
                                    contents: &src[pos + contents_beg + 1..pos + contents_end - 1],
                                },
                                pos + end
                            ),
                            "VERSE" => ret!(
                                Element::VerseBlock {
                                    args,
                                    contents: &src[pos + contents_beg + 1..pos + contents_end - 1],
                                },
                                pos + end
                            ),
                            "CENTER" => ret!(
                                Element::CenterBlock {
                                    args,
                                    contents_end,
                                    end,
                                },
                                pos + contents_beg
                            ),
                            "QUOTE" => ret!(
                                Element::QuoteBlock {
                                    args,
                                    contents_end,
                                    end,
                                },
                                pos + contents_beg
                            ),
                            _ => ret!(
                                Element::SpecialBlock {
                                    name,
                                    args,
                                    contents_end,
                                    end,
                                },
                                pos + contents_beg
                            ),
                        };
                    }

                    if let Some((name, args, contents_beg, contents_end, end)) =
                        DynBlock::parse(&src[pos..])
                    {
                        ret!(
                            Element::DynBlock {
                                name,
                                args,
                                contents_end,
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
                if bytes[pos] == b'#' && bytes.get(pos + 1).filter(|&&b| b == b' ').is_some() {
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
                            end: pos - 1,
                            trailing: pos,
                        }),
                        None,
                    );
                }
            } else {
                return (
                    start,
                    Some(Element::Paragraph {
                        end: src.len(),
                        trailing: src.len(),
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
