pub mod block;
pub mod dyn_block;
pub mod fn_def;
pub mod keyword;
pub mod list;
pub mod rule;

pub use self::keyword::Key;

use memchr::{memchr, memchr_iter};

#[cfg_attr(test, derive(PartialEq))]
#[derive(Debug)]
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
    },
}

// return (element, off, next element, next offset)
// the end of first element is relative to the offset
// next offset is relative to the end of the first element
pub fn parse<'a>(src: &'a str) -> (Option<Element<'a>>, usize, Option<(Element<'a>, usize)>) {
    // skip empty lines
    let mut pos = match src.chars().position(|c| c != '\n') {
        Some(pos) => pos,
        None => return (None, src.len(), None),
    };
    let start = pos;
    let bytes = src.as_bytes();
    let mut line_ends = memchr_iter(b'\n', &bytes[start..]).map(|i| i + start);

    loop {
        let line_beg = pos;

        macro_rules! brk {
            ($ele:expr, $off:expr) => {
                break if line_beg == 0 || pos == start {
                    (Some($ele), start + $off, None)
                } else {
                    (
                        Some(Element::Paragraph {
                            cont_end: line_beg - start - 1,
                            end: line_beg - start,
                        }),
                        start,
                        Some(($ele, $off)),
                    )
                };
            };
        }

        // Unlike other element, footnote def must starts at column 0
        if bytes[pos..].starts_with(b"[fn:") {
            if let Some((label, cont, off)) = fn_def::parse(&src[pos..]) {
                brk!(Element::FnDef { label, cont }, off + 1);
            }
        }

        if bytes[pos] == b'\n' {
            break (
                Some(Element::Paragraph {
                    cont_end: pos - start - 1,
                    end: pos - start + 1,
                }),
                start,
                None,
            );
        }

        pos = skip_space!(src, pos);

        let (is_item, ordered) = list::is_item(&src[pos..]);
        if is_item {
            let list = Element::List {
                ident: pos - line_beg,
                ordered,
            };
            break if line_beg == start {
                (Some(list), start, None)
            } else {
                (
                    Some(Element::Paragraph {
                        cont_end: line_beg - start - 1,
                        end: line_beg - start,
                    }),
                    start,
                    Some((list, 0)),
                )
            };
        }

        // TODO: LaTeX environment
        if bytes[pos..].starts_with(b"\\begin{") {}

        // Rule
        if bytes[pos] == b'-' {
            let off = rule::parse(&src[pos..]);
            if off != 0 {
                brk!(Element::Rule, off);
            }
        }

        // TODO: multiple lines fixed width area
        if bytes[pos..].starts_with(b": ") || bytes[pos..].starts_with(b":\n") {
            let eol = memchr(b'\n', &bytes[pos..])
                .map(|i| i + 1)
                .unwrap_or_else(|| src.len() - pos);
            brk!(Element::FixedWidth(&src[pos + 1..pos + eol].trim()), eol);
        }

        if bytes[pos..].starts_with(b"#+") {
            if let Some((name, args, cont_beg, cont_end, end)) = block::parse(&src[pos..]) {
                let cont = &src[pos + cont_beg..pos + cont_end];
                match name.to_uppercase().as_str() {
                    "COMMENT" => brk!(Element::CommentBlock { args, cont }, end),
                    "EXAMPLE" => brk!(Element::ExampleBlock { args, cont }, end),
                    "EXPORT" => brk!(Element::ExportBlock { args, cont }, end),
                    "SRC" => brk!(Element::SrcBlock { args, cont }, end),
                    "VERSE" => brk!(Element::VerseBlock { args, cont }, end),
                    "CENTER" => brk!(
                        Element::CtrBlock {
                            args,
                            cont_end: cont_end - cont_beg,
                            end: end - cont_beg,
                        },
                        cont_beg
                    ),
                    "QUOTE" => brk!(
                        Element::QteBlock {
                            args,
                            cont_end: cont_end - cont_beg,
                            end: end - cont_beg,
                        },
                        cont_beg
                    ),
                    _ => brk!(
                        Element::SplBlock {
                            name,
                            args,
                            cont_end: cont_end - cont_beg,
                            end: end - cont_beg,
                        },
                        cont_beg
                    ),
                };
            }

            if let Some((name, args, cont_beg, cont_end, end)) = dyn_block::parse(&src[pos..]) {
                brk!(
                    Element::DynBlock {
                        name,
                        args,
                        cont_end: cont_end - cont_beg,
                        end: end - cont_beg,
                    },
                    cont_beg
                )
            }

            if let Some((key, value, off)) = keyword::parse(&src[pos..]) {
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
        if bytes[pos..].starts_with(b"# ") || bytes[pos..].starts_with(b"#\n") {
            let eol = memchr(b'\n', &bytes[pos..])
                .map(|i| i + 1)
                .unwrap_or_else(|| src.len() - pos);
            brk!(Element::Comment(&src[pos + 1..pos + eol].trim()), eol);
        }

        // move to the beginning of the next line
        if let Some(off) = line_ends.next() {
            pos = off + 1;

            // the last character
            if pos >= src.len() {
                break (
                    Some(Element::Paragraph {
                        cont_end: src.len() - start - 1,
                        end: src.len() - start,
                    }),
                    start,
                    None,
                );
            }
        } else {
            break (
                Some(Element::Paragraph {
                    cont_end: src.len() - start,
                    end: src.len() - start,
                }),
                start,
                None,
            );
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn parse() {
        use super::parse;
        use super::Element::*;

        assert_eq!(parse("\n\n\n"), (None, 3, None));

        let len = "Lorem ipsum dolor sit amet.".len();
        assert_eq!(
            parse("\nLorem ipsum dolor sit amet.\n\n\n"),
            (
                Some(Paragraph {
                    cont_end: len,
                    end: len + 2,
                }),
                1,
                None
            )
        );
        assert_eq!(
            parse("\n\nLorem ipsum dolor sit amet.\n\n"),
            (
                Some(Paragraph {
                    cont_end: len,
                    end: len + 2,
                }),
                2,
                None
            )
        );
        assert_eq!(
            parse("\nLorem ipsum dolor sit amet.\n"),
            (
                Some(Paragraph {
                    cont_end: len,
                    end: len + 1,
                }),
                1,
                None
            )
        );
        assert_eq!(
            parse("\n\n\nLorem ipsum dolor sit amet."),
            (
                Some(Paragraph {
                    cont_end: len,
                    end: len,
                }),
                3,
                None
            )
        );

        assert_eq!(
            parse("\n\n\n: Lorem ipsum dolor sit amet.\n"),
            (
                Some(FixedWidth("Lorem ipsum dolor sit amet.")),
                "\n\n\n: Lorem ipsum dolor sit amet.\n".len(),
                None
            )
        );
        assert_eq!(
            parse("\n\n\n: Lorem ipsum dolor sit amet."),
            (
                Some(FixedWidth("Lorem ipsum dolor sit amet.")),
                "\n\n\n: Lorem ipsum dolor sit amet.".len(),
                None
            )
        );

        assert_eq!(
            parse("\n\nLorem ipsum dolor sit amet.\n: Lorem ipsum dolor sit amet.\n"),
            (
                Some(Paragraph {
                    cont_end: len,
                    end: len + 1,
                }),
                2,
                Some((FixedWidth("Lorem ipsum dolor sit amet."), 30))
            )
        );

        assert_eq!(
            parse("\n\nLorem ipsum dolor sit amet.\n+ Lorem ipsum dolor sit amet.\n"),
            (
                Some(Paragraph {
                    cont_end: len,
                    end: len + 1,
                }),
                2,
                Some((
                    List {
                        ident: 0,
                        ordered: false,
                    },
                    0
                ))
            )
        );

        assert_eq!(
            parse("\n\nLorem ipsum dolor sit amet.\n#+BEGIN_QUOTE\nLorem ipsum dolor sit amet.\n#+END_QUOTE\n"),
            (
                Some(Paragraph {
                    cont_end: len,
                    end: len + 1,
                }),
                2,
                Some((
                    QteBlock {
                        args: None,
                        cont_end: len + 1,
                        end: len + 1 + "#+END_QUOTE\n".len()
                    },
                    "#+BEGIN_QUOTE\n".len()
                ))
            )
        );
        // TODO: more tests
    }
}
