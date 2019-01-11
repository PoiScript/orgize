pub mod block;
pub mod dyn_block;
pub mod fn_def;
pub mod keyword;
pub mod rule;

pub use self::block::Block;
pub use self::dyn_block::DynBlock;
pub use self::fn_def::FnDef;
pub use self::keyword::Keyword;
pub use self::rule::Rule;

#[cfg_attr(test, derive(PartialEq, Debug))]
pub enum Element<'a> {
    Paragraph {
        // end of the contents
        end: usize,
        // trailing space
        trailing: usize,
    },
    Keyword(Keyword<'a>),
    FnDef(FnDef<'a>),

    CenterBlock {
        args: Option<&'a str>,
        content_end: usize,
        end: usize,
    },
    QuoteBlock {
        args: Option<&'a str>,
        content_end: usize,
        end: usize,
    },
    SpecialBlock {
        args: Option<&'a str>,
        name: &'a str,
        content_end: usize,
        end: usize,
    },
    CommentBlock {
        content: &'a str,
        args: Option<&'a str>,
    },
    ExampleBlock {
        content: &'a str,
        args: Option<&'a str>,
    },
    ExportBlock {
        content: &'a str,
        args: Option<&'a str>,
    },
    SrcBlock {
        content: &'a str,
        args: Option<&'a str>,
    },
    VerseBlock {
        content: &'a str,
        args: Option<&'a str>,
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
                if let Some((fd, off)) = FnDef::parse(&src[pos..]) {
                    return if pos == start {
                        (off + 1, Some(Element::FnDef(fd)), None)
                    } else {
                        (
                            start,
                            Some(Element::Paragraph {
                                end: pos - 1,
                                trailing: pos,
                            }),
                            Some((Element::FnDef(fd), off + 1)),
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
                                    end: end - 1,
                                    trailing: pos - 1,
                                }),
                                Some(($ele, $off)),
                            )
                        };
                    };
                }

                if bytes[pos] == b'\n' {
                    return (
                        start,
                        Some(Element::Paragraph {
                            end: end - 1,
                            trailing: pos,
                        }),
                        None,
                    );
                }

                // TODO: LaTeX environment
                if bytes[pos] == b'\\' {}

                // Rule
                if bytes[pos] == b'-' {
                    if let Some(off) = Rule::parse(&src[pos..]) {
                        ret!(Element::Rule, off);
                    }
                }

                if bytes[pos] == b'#' && bytes[pos + 1] == b'+' {
                    if let Some((name, args, content, end)) = Block::parse(&src[pos..]) {
                        match &src[pos + 8..pos + name] {
                            block_name if block_name.eq_ignore_ascii_case("CENTER") => ret!(
                                Element::CenterBlock {
                                    args: if name == args {
                                        None
                                    } else {
                                        Some(&src[pos + name..pos + args])
                                    },
                                    content_end: content + 1,
                                    end,
                                },
                                pos + args
                            ),
                            block_name if block_name.eq_ignore_ascii_case("QUOTE") => ret!(
                                Element::QuoteBlock {
                                    args: if name == args {
                                        None
                                    } else {
                                        Some(&src[pos + name..pos + args].trim())
                                    },
                                    content_end: content + 1,
                                    end,
                                },
                                pos + args
                            ),
                            block_name if block_name.eq_ignore_ascii_case("COMMENT") => ret!(
                                Element::CommentBlock {
                                    args: if name == args {
                                        None
                                    } else {
                                        Some(&src[pos + name..pos + args])
                                    },
                                    content: &src[pos + args + 1..pos + content],
                                },
                                pos + end
                            ),
                            block_name if block_name.eq_ignore_ascii_case("EXAMPLE") => ret!(
                                Element::ExampleBlock {
                                    args: if name == args {
                                        None
                                    } else {
                                        Some(&src[pos + name..pos + args])
                                    },
                                    content: &src[pos + args + 1..pos + content],
                                },
                                pos + end
                            ),
                            block_name if block_name.eq_ignore_ascii_case("EXPORT") => ret!(
                                Element::ExportBlock {
                                    args: if name == args {
                                        None
                                    } else {
                                        Some(&src[pos + name..pos + args])
                                    },
                                    content: &src[pos + args + 1..pos + content],
                                },
                                pos + end
                            ),
                            block_name if block_name.eq_ignore_ascii_case("SRC") => ret!(
                                Element::SrcBlock {
                                    args: if name == args {
                                        None
                                    } else {
                                        Some(&src[pos + name..pos + args])
                                    },
                                    content: &src[pos + args + 1..pos + content],
                                },
                                pos + end
                            ),
                            block_name if block_name.eq_ignore_ascii_case("VERSE") => ret!(
                                Element::VerseBlock {
                                    args: if name == args {
                                        None
                                    } else {
                                        Some(&src[pos + name..pos + args])
                                    },
                                    content: &src[pos + args + 1..pos + content],
                                },
                                pos + end
                            ),
                            block_name => ret!(
                                Element::SpecialBlock {
                                    name: block_name,
                                    args: if name == args {
                                        None
                                    } else {
                                        Some(&src[pos + name..pos + args].trim())
                                    },
                                    content_end: content + 1,
                                    end,
                                },
                                pos + args
                            ),
                        };
                    }

                    if let Some((kw, off)) = Keyword::parse(&src[pos..]) {
                        ret!(Element::Keyword(kw), off)
                    }
                }

                // Comment
                if bytes[pos] == b'#' && bytes[pos + 1] == b' ' {
                    let eol = eol!(src, pos);
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
