use rowan::{
    ast::{support, AstNode},
    SyntaxNode, TextRange, TextSize, TokenAtOffset,
};

use crate::ast::Headline;
use crate::syntax::{
    combinator::line_starts_iter, document::document_node, headline::headline_node, OrgLanguage,
};
use crate::Org;

#[derive(Debug)]
enum RangeShape {
    InsideHeadline { headline: Headline, level: usize },
    ExactHeadline { headline: Headline, level: usize },
    Other,
}

impl RangeShape {
    pub fn new(mut node: SyntaxNode<OrgLanguage>, range: TextRange) -> Self {
        let mut result = RangeShape::Other;

        'l: loop {
            for headline in support::children::<Headline>(&node) {
                let level = headline.level();
                let start = headline.syntax.text_range().start();
                let end = headline.syntax.text_range().end();

                if headline.syntax.text_range() == range {
                    result = RangeShape::ExactHeadline { headline, level };
                    break 'l;
                }

                if TextRange::new(start + TextSize::from(level as u32 + 1), end)
                    .contains_range(range)
                {
                    node = headline.syntax.clone();
                    result = RangeShape::InsideHeadline { headline, level };
                    continue 'l;
                }
            }
            break;
        }

        result
    }
}

#[derive(Debug, PartialEq)]
enum ReplaceWithShape {
    IncludeHeadline { level: usize },
    ExactHeadline { level: usize },
    Other,
}

impl ReplaceWithShape {
    fn new(text: &str) -> Self {
        let mut result = ReplaceWithShape::Other;

        for start in line_starts_iter(text) {
            let level = text[start..].bytes().take_while(|&c| c == b'*').count();

            if level == 0 {
                continue;
            }

            if !matches!(text[start..].as_bytes().get(level), Some(b' ')) {
                continue;
            }

            match result {
                ReplaceWithShape::IncludeHeadline { level: l } => {
                    if level < l {
                        result = ReplaceWithShape::IncludeHeadline { level }
                    }
                }
                ReplaceWithShape::ExactHeadline { level: l } => {
                    if level <= l {
                        result = ReplaceWithShape::IncludeHeadline { level }
                    }
                }
                ReplaceWithShape::Other => {
                    if start == 0 {
                        result = ReplaceWithShape::ExactHeadline { level }
                    } else {
                        result = ReplaceWithShape::IncludeHeadline { level }
                    }
                }
            }
        }

        result
    }
}

impl Org {
    /// Replace specified range with given text, and reparse the syntax tree with current config
    ///
    /// This method optimizes parsing by analyzing the selected range and given text, and reducing
    /// the amount of data processed by parser.
    ///
    /// ```rust
    /// use orgize::{Org, ast::Headline, TextRange, TextSize};
    ///
    /// let mut org = Org::parse("** hello");
    /// let hdl = org.first_node::<Headline>().unwrap();
    /// assert_eq!(hdl.level(), 2);
    ///
    /// // replace '**' with '*****'
    /// org.replace_range(TextRange::new(0.into(), 2.into()), "*****");
    /// // since the syntax tree is changed, we have to query again
    /// let hdl = org.first_node::<Headline>().unwrap();
    /// assert_eq!(hdl.level(), 5);
    /// ```
    pub fn replace_range(&mut self, range: TextRange, replace_with: impl AsRef<str>) {
        let replace_with = replace_with.as_ref();
        match (
            RangeShape::new(self.document().syntax, range),
            ReplaceWithShape::new(replace_with),
        ) {
            (
                RangeShape::ExactHeadline { headline, level },
                ReplaceWithShape::IncludeHeadline { level: new_level },
            )
            | (
                RangeShape::InsideHeadline { headline, level },
                ReplaceWithShape::IncludeHeadline { level: new_level },
            ) if level < new_level => self.replace_headline(headline, range, replace_with),

            (
                RangeShape::ExactHeadline { headline, level },
                ReplaceWithShape::ExactHeadline { level: new_level },
            ) if level <= new_level
            // non-last headline must ends with a newline
                && (headline.end() == self.document().end()
                    || replace_with.ends_with(['\n', '\r'])) =>
            {
                self.replace_headline(headline, range, replace_with)
            }

            (
                RangeShape::InsideHeadline { headline, level },
                ReplaceWithShape::ExactHeadline { level: new_level },
            ) if level <= new_level && follows_newline(headline.syntax(), range.start()) => {
                self.replace_headline(headline, range, replace_with)
            }

            _ => self.full_parse(range, replace_with),
        }
    }

    fn full_parse(&mut self, range: TextRange, replace_with: &str) {
        if self.document().syntax().text_range() == range {
            let input = (replace_with, &self.config).into();
            self.green = document_node(input).unwrap().1.into_node().unwrap();
        } else {
            let start: usize = range.start().into();
            let end: usize = range.end().into();
            let mut text = self.green.to_string();
            text.replace_range(start..end, replace_with);
            let input = (text.as_ref(), &self.config).into();
            self.green = document_node(input).unwrap().1.into_node().unwrap();
        }
    }

    fn replace_headline(&mut self, headline: Headline, range: TextRange, replace_with: &str) {
        if headline.syntax().text_range() == range {
            let input = (replace_with, &self.config).into();

            self.green = headline
                .syntax
                .replace_with(headline_node(input).unwrap().1.into_node().unwrap());
        } else {
            let offset: usize = headline.syntax.text_range().start().into();
            let start: usize = range.start().into();
            let end: usize = range.end().into();

            let mut text = headline.syntax.to_string();
            text.replace_range((start - offset)..(end - offset), replace_with);

            let input = (text.as_ref(), &self.config).into();

            self.green = headline
                .syntax
                .replace_with(headline_node(input).unwrap().1.into_node().unwrap());
        }
    }
}

fn follows_newline(syntax: &SyntaxNode<OrgLanguage>, offset: TextSize) -> bool {
    match syntax.token_at_offset(offset) {
        TokenAtOffset::None => false,
        TokenAtOffset::Single(t) => {
            let offset: usize = (offset - t.text_range().start()).into();
            t.text()[offset..].ends_with('\n') || t.text()[offset..].ends_with('\r')
        }
        TokenAtOffset::Between(t, _) => t.text().ends_with('\n') || t.text().ends_with('\r'),
    }
}

#[test]
fn replace() {
    assert!(follows_newline(
        Org::parse("\n*a*").document().syntax(),
        TextSize::new(1)
    ));
    assert!(follows_newline(
        Org::parse(" \na").document().syntax(),
        TextSize::new(1)
    ));
    assert!(follows_newline(
        Org::parse(" \ra").document().syntax(),
        TextSize::new(1)
    ));
    assert!(!follows_newline(
        Org::parse(" *a*").document().syntax(),
        TextSize::new(1)
    ));
    assert!(!follows_newline(
        Org::parse(" a").document().syntax(),
        TextSize::new(1)
    ));

    assert_eq!(ReplaceWithShape::new(""), ReplaceWithShape::Other);
    assert_eq!(ReplaceWithShape::new(" ** a"), ReplaceWithShape::Other);
    assert_eq!(
        ReplaceWithShape::new("\n** a"),
        ReplaceWithShape::IncludeHeadline { level: 2 }
    );
    assert_eq!(
        ReplaceWithShape::new("** a"),
        ReplaceWithShape::ExactHeadline { level: 2 }
    );
    assert_eq!(
        ReplaceWithShape::new("** a\n* 1"),
        ReplaceWithShape::IncludeHeadline { level: 1 }
    );
    assert_eq!(
        ReplaceWithShape::new("* a\n** 1"),
        ReplaceWithShape::ExactHeadline { level: 1 }
    );
    assert_eq!(
        ReplaceWithShape::new("** a\n** 1"),
        ReplaceWithShape::IncludeHeadline { level: 2 }
    );

    assert!(matches!(
        RangeShape::new(
            Org::parse("** abc\n** b").document().syntax,
            TextRange::new(0.into(), 7.into())
        ),
        RangeShape::ExactHeadline { level: 2, .. }
    ));
    assert!(matches!(
        RangeShape::new(
            Org::parse("** abc\n** b").document().syntax,
            TextRange::new(3.into(), 7.into())
        ),
        RangeShape::InsideHeadline { level: 2, .. }
    ));
    assert!(matches!(
        RangeShape::new(
            Org::parse("** abc\n** b").document().syntax,
            TextRange::new(2.into(), 7.into())
        ),
        RangeShape::Other
    ));
    assert!(matches!(
        RangeShape::new(
            Org::parse("* abc\n** b").document().syntax,
            TextRange::new(4.into(), 7.into())
        ),
        RangeShape::InsideHeadline { level: 1, .. }
    ));

    macro_rules! t {
        ($input:literal, $replace:literal) => {
            let start = $input.find('|').unwrap();
            let end = $input.rfind('|').unwrap();

            let input = format!(
                "{}{}{}",
                &$input[0..start],
                &$input[start + 1..end],
                &$input[end + 1..]
            );
            let output = format!("{}{}{}", &$input[0..start], $replace, &$input[end + 1..]);

            let mut org = Org::parse(input);
            org.replace_range(
                TextRange::new((start as u32).into(), (end as u32 - 1).into()),
                $replace,
            );

            debug_assert_eq!(
                format!("{:#?}", org.document().syntax),
                format!("{:#?}", Org::parse(output).document().syntax),
            );
        };
    }

    t!("||", "");
    t!("||", "** abc");
    t!("*** abc |edf|", "fde");
    t!("*|** abc edf|", "fde");
    t!("* abc \n|** edf|", "** abc");
    t!("* ab|c \n*| edf", "** abc");

    t!("* abc \n|** edf|", "**   abc");
    t!("* abc \n|** edf|", "**   eee\n**   eee");
    t!("* abc \n|** edf|", "*** abc");
    t!("* abc \n*|* edf|", "*** abc");
    t!("* abc \n**| edf|", "*** abc");
    t!("* abc \n**| |edf", "*** abc");
    t!("* abc \n** |edf|", "*** abc");
    t!("* abc \n** |edf|", "\n*** abc");
    t!("* abc \n** |edf|", "\n** abc");
    t!("* abc \n** |edf|", "\n* abc");
    t!("* abc \n** \n|edf|", "* abc");
    t!("* abc \n** \n|edf|", "* abc\n* abc");
    t!("* abc \n** |edf|", "* abc");
    t!("* abc \n** |edf|", "* abc\n* abc");
    t!("* abc \n|* edf\n|* gh", "* hg");
    t!("* abc \n|* edf\n|* gh", "* hg\n");
    t!("* abc \n* edf\n|* gh|", "* hg");
}
