pub mod block;
pub mod dyn_block;
pub mod fn_def;
pub mod keyword;
pub mod list;
pub mod rule;

pub use self::keyword::Key;

use memchr::memchr_iter;

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

    // Element::Empty actually means Option<Element>::None
    Empty,
}

// return (element, off, next element, next offset)
// the end of first element is relative to the offset
// next offset is relative to the end of the first element
pub fn parse(src: &str) -> (Element<'_>, usize, Option<(Element<'_>, usize)>) {
    // skip empty lines
    let mut pos = match src.chars().position(|c| c != '\n') {
        Some(pos) => pos,
        None => return (Element::Empty, src.len(), None),
    };
    let start = pos;
    let bytes = src.as_bytes();
    let mut line_ends = memchr_iter(b'\n', &bytes[start..]).map(|i| i + start);

    loop {
        let line_beg = pos;

        macro_rules! brk {
            ($ele:expr, $off:expr) => {
                break if line_beg == start || pos == start {
                    ($ele, pos + $off, None)
                } else {
                    (
                        Element::Paragraph {
                            cont_end: line_beg - start - 1,
                            end: line_beg - start,
                        },
                        start,
                        Some(($ele, $off)),
                    )
                };
            };
        }

        let tail = &src[pos..];

        // Unlike other element, footnote def must starts at column 0
        if tail.starts_with("[fn:") {
            if let Some((label, cont, off)) = fn_def::parse(tail) {
                brk!(Element::FnDef { label, cont }, off + 1);
            }
        }

        if bytes[pos] == b'\n' {
            break (
                Element::Paragraph {
                    cont_end: pos - start - 1,
                    end: pos - start + 1,
                },
                start,
                None,
            );
        }

        pos = skip_space!(src, pos);

        let tail = &src[pos..];

        let (is_item, ordered) = list::is_item(tail);
        if is_item {
            let list = Element::List {
                ident: pos - line_beg,
                ordered,
            };
            break if line_beg == start {
                (list, start, None)
            } else {
                (
                    Element::Paragraph {
                        cont_end: line_beg - start - 1,
                        end: line_beg - start,
                    },
                    start,
                    Some((list, 0)),
                )
            };
        }

        // TODO: LaTeX environment
        if tail.starts_with("\\begin{") {}

        // rule
        if tail.starts_with("-----") {
            let off = rule::parse(tail);
            if off != 0 {
                brk!(Element::Rule, off);
            }
        }

        // fixed width
        if tail.starts_with(": ") || tail.starts_with(":\n") {
            let end = line_ends
                .skip_while(|&i| src[i + 1..].starts_with(": ") || src[i + 1..].starts_with(":\n"))
                .next()
                .map(|i| i + 1)
                .unwrap_or_else(|| src.len());
            let off = end - pos;
            brk!(Element::FixedWidth(&tail[0..off]), off);
        }

        // comment
        if tail.starts_with("# ") || tail.starts_with("#\n") {
            let end = line_ends
                .skip_while(|&i| src[i + 1..].starts_with("# ") || src[i + 1..].starts_with("#\n"))
                .next()
                .map(|i| i + 1)
                .unwrap_or_else(|| src.len());
            let off = end - pos;
            brk!(Element::Comment(&tail[0..off]), off);
        }

        if tail.starts_with("#+") {
            if let Some((name, args, cont_beg, cont_end, end)) = block::parse(tail) {
                let cont = &tail[cont_beg..cont_end];
                match &*name.to_uppercase() {
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

            if let Some((name, args, cont_beg, cont_end, end)) = dyn_block::parse(tail) {
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

            if let Some((key, value, off)) = keyword::parse(tail) {
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

        // move to the beginning of the next line
        if let Some(off) = line_ends.next() {
            pos = off + 1;

            // the last character
            if pos >= src.len() {
                break (
                    Element::Paragraph {
                        cont_end: src.len() - start - 1,
                        end: src.len() - start,
                    },
                    start,
                    None,
                );
            }
        } else {
            break (
                Element::Paragraph {
                    cont_end: src.len() - start,
                    end: src.len() - start,
                },
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
        use super::keyword::Key;
        use super::parse;
        use super::Element::*;

        assert_eq!(parse("\n\n\n"), (Empty, 3, None));

        let len = "Lorem ipsum dolor sit amet.".len();
        assert_eq!(
            parse("\nLorem ipsum dolor sit amet.\n\n\n"),
            (
                Paragraph {
                    cont_end: len,
                    end: len + 2,
                },
                1,
                None
            )
        );
        assert_eq!(
            parse("\n\nLorem ipsum dolor sit amet.\n\n"),
            (
                Paragraph {
                    cont_end: len,
                    end: len + 2,
                },
                2,
                None
            )
        );
        assert_eq!(
            parse("\nLorem ipsum dolor sit amet.\n"),
            (
                Paragraph {
                    cont_end: len,
                    end: len + 1,
                },
                1,
                None
            )
        );
        assert_eq!(
            parse("\n\n\nLorem ipsum dolor sit amet."),
            (
                Paragraph {
                    cont_end: len,
                    end: len,
                },
                3,
                None
            )
        );

        assert_eq!(
            parse("\n\n\n: Lorem ipsum dolor sit amet.\n"),
            (
                FixedWidth(": Lorem ipsum dolor sit amet.\n"),
                "\n\n\n: Lorem ipsum dolor sit amet.\n".len(),
                None
            )
        );
        assert_eq!(
            parse("\n\n\n: Lorem ipsum dolor sit amet."),
            (
                FixedWidth(": Lorem ipsum dolor sit amet."),
                "\n\n\n: Lorem ipsum dolor sit amet.".len(),
                None
            )
        );

        assert_eq!(
            parse("\n\nLorem ipsum dolor sit amet.\n: Lorem ipsum dolor sit amet.\n"),
            (
                Paragraph {
                    cont_end: len,
                    end: len + 1,
                },
                2,
                Some((FixedWidth(": Lorem ipsum dolor sit amet.\n"), 30))
            )
        );

        assert_eq!(
            parse("\n\nLorem ipsum dolor sit amet.\n: Lorem ipsum dolor sit amet.\n:\n: Lorem ipsum dolor sit amet."),
            (
                Paragraph {
                    cont_end: len,
                    end: len + 1,
                },
                2,
                Some((FixedWidth(": Lorem ipsum dolor sit amet.\n:\n: Lorem ipsum dolor sit amet."), 61))
            )
        );

        assert_eq!(
            parse("\n\nLorem ipsum dolor sit amet.\n+ Lorem ipsum dolor sit amet.\n"),
            (
                Paragraph {
                    cont_end: len,
                    end: len + 1,
                },
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
                Paragraph {
                    cont_end: len,
                    end: len + 1,
                },
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
        assert_eq!(
            parse("\n  #+ATTR_HTML: :width 200px"),
            (
                Keyword {
                    key: Key::Attr { backend: "HTML" },
                    value: ":width 200px"
                },
                "\n  #+ATTR_HTML: :width 200px".len(),
                None
            )
        );
        // TODO: more tests
    }
}
