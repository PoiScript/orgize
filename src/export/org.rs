use std::io::{Error, Result as IOResult, Write};

use crate::elements::{Clock, Element, Table, Timestamp};
use crate::export::write_datetime;

pub trait OrgHandler<E: From<Error>>: Default {
    fn start<W: Write>(&mut self, w: W, element: &Element) -> Result<(), E>;
    fn end<W: Write>(&mut self, w: W, element: &Element) -> Result<(), E>;
}

#[derive(Default)]
pub struct DefaultOrgHandler;

impl OrgHandler<Error> for DefaultOrgHandler {
    fn start<W: Write>(&mut self, mut w: W, element: &Element) -> IOResult<()> {
        match element {
            // container elements
            Element::SpecialBlock(block) => {
                writeln!(w, "#+BEGIN_{}", block.name)?;
                write_blank_lines(&mut w, block.pre_blank)?;
            }
            Element::QuoteBlock(block) => {
                writeln!(&mut w, "#+BEGIN_QUOTE")?;
                write_blank_lines(&mut w, block.pre_blank)?;
            }
            Element::CenterBlock(block) => {
                writeln!(&mut w, "#+BEGIN_CENTER")?;
                write_blank_lines(&mut w, block.pre_blank)?;
            }
            Element::VerseBlock(block) => {
                writeln!(&mut w, "#+BEGIN_VERSE")?;
                write_blank_lines(&mut w, block.pre_blank)?;
            }
            Element::Bold => write!(w, "*")?,
            Element::Document { pre_blank } => {
                write_blank_lines(w, *pre_blank)?;
            }
            Element::DynBlock(dyn_block) => {
                write!(&mut w, "#+BEGIN: {}", dyn_block.block_name)?;
                if let Some(parameters) = &dyn_block.arguments {
                    write!(&mut w, " {}", parameters)?;
                }
                write_blank_lines(&mut w, dyn_block.pre_blank + 1)?;
            }
            Element::Headline { .. } => (),
            Element::List(_list) => (),
            Element::Italic => write!(w, "/")?,
            Element::ListItem(list_item) => {
                for _ in 0..list_item.indent {
                    write!(&mut w, " ")?;
                }
                write!(&mut w, "{}", list_item.bullet)?;
            }
            Element::Paragraph { .. } => (),
            Element::Section => (),
            Element::Strike => write!(w, "+")?,
            Element::Underline => write!(w, "_")?,
            Element::Drawer(drawer) => {
                writeln!(&mut w, ":{}:", drawer.name)?;
                write_blank_lines(&mut w, drawer.pre_blank)?;
            }
            // non-container elements
            Element::CommentBlock(block) => {
                writeln!(&mut w, "#+BEGIN_COMMENT")?;
                write!(&mut w, "{}", block.contents)?;
                writeln!(&mut w, "#+END_COMMENT")?;
                write_blank_lines(&mut w, block.post_blank)?;
            }
            Element::ExampleBlock(block) => {
                writeln!(&mut w, "#+BEGIN_EXAMPLE")?;
                write!(&mut w, "{}", block.contents)?;
                writeln!(&mut w, "#+END_EXAMPLE")?;
                write_blank_lines(&mut w, block.post_blank)?;
            }
            Element::ExportBlock(block) => {
                writeln!(&mut w, "#+BEGIN_EXPORT {}", block.data)?;
                write!(&mut w, "{}", block.contents)?;
                writeln!(&mut w, "#+END_EXPORT")?;
                write_blank_lines(&mut w, block.post_blank)?;
            }
            Element::SourceBlock(block) => {
                writeln!(&mut w, "#+BEGIN_SRC {}", block.language)?;
                write!(&mut w, "{}", block.contents)?;
                writeln!(&mut w, "#+END_SRC")?;
                write_blank_lines(&mut w, block.post_blank)?;
            }
            Element::BabelCall(call) => {
                writeln!(&mut w, "#+CALL: {}", call.value)?;
                write_blank_lines(w, call.post_blank)?;
            }
            Element::InlineSrc(inline_src) => {
                write!(&mut w, "src_{}", inline_src.lang)?;
                if let Some(options) = &inline_src.options {
                    write!(&mut w, "[{}]", options)?;
                }
                write!(&mut w, "{{{}}}", inline_src.body)?;
            }
            Element::Code { value } => write!(w, "~{}~", value)?,
            Element::FnRef(fn_ref) => {
                write!(&mut w, "[fn:{}", fn_ref.label)?;
                if let Some(definition) = &fn_ref.definition {
                    write!(&mut w, ":{}", definition)?;
                }
                write!(&mut w, "]")?;
            }
            Element::InlineCall(inline_call) => {
                write!(&mut w, "call_{}", inline_call.name)?;
                if let Some(header) = &inline_call.inside_header {
                    write!(&mut w, "[{}]", header)?;
                }
                write!(&mut w, "({})", inline_call.arguments)?;
                if let Some(header) = &inline_call.end_header {
                    write!(&mut w, "[{}]", header)?;
                }
            }
            Element::Link(link) => {
                write!(&mut w, "[[{}]", link.path)?;
                if let Some(desc) = &link.desc {
                    write!(&mut w, "[{}]", desc)?;
                }
                write!(&mut w, "]")?;
            }
            Element::Macros(_macros) => (),
            Element::RadioTarget => (),
            Element::Snippet(snippet) => write!(w, "@@{}:{}@@", snippet.name, snippet.value)?,
            Element::Target(_target) => (),
            Element::Text { value } => write!(w, "{}", value)?,
            Element::Timestamp(timestamp) => {
                write_timestamp(&mut w, &timestamp)?;
            }
            Element::Verbatim { value } => write!(w, "={}=", value)?,
            Element::FnDef(fn_def) => {
                write_blank_lines(w, fn_def.post_blank)?;
            }
            Element::Clock(clock) => {
                write!(w, "CLOCK: ")?;

                match clock {
                    Clock::Closed {
                        start,
                        end,
                        duration,
                        post_blank,
                        ..
                    } => {
                        write_datetime(&mut w, "[", &start, "]--")?;
                        write_datetime(&mut w, "[", &end, "]")?;
                        writeln!(&mut w, " => {}", duration)?;
                        write_blank_lines(&mut w, *post_blank)?;
                    }
                    Clock::Running {
                        start, post_blank, ..
                    } => {
                        write_datetime(&mut w, "[", &start, "]\n")?;
                        write_blank_lines(&mut w, *post_blank)?;
                    }
                }
            }
            Element::Comment(comment) => {
                write!(w, "{}", comment.value)?;
                write_blank_lines(&mut w, comment.post_blank)?;
            }
            Element::FixedWidth(fixed_width) => {
                write!(&mut w, "{}", fixed_width.value)?;
                write_blank_lines(&mut w, fixed_width.post_blank)?;
            }
            Element::Keyword(keyword) => {
                write!(&mut w, "#+{}", keyword.key)?;
                if let Some(optional) = &keyword.optional {
                    write!(&mut w, "[{}]", optional)?;
                }
                writeln!(&mut w, ": {}", keyword.value)?;
                write_blank_lines(&mut w, keyword.post_blank)?;
            }
            Element::Rule(rule) => {
                writeln!(w, "-----")?;
                write_blank_lines(&mut w, rule.post_blank)?;
            }
            Element::Cookie(_cookie) => (),
            Element::Title(title) => {
                for _ in 0..title.level {
                    write!(&mut w, "*")?;
                }
                if let Some(keyword) = &title.keyword {
                    write!(&mut w, " {}", keyword)?;
                }
                if let Some(priority) = title.priority {
                    write!(&mut w, " [#{}]", priority)?;
                }
                write!(&mut w, " ")?;
            }
            Element::Table(_) => (),
            Element::TableRow(_) => (),
            Element::TableCell(_) => (),
        }

        Ok(())
    }

    fn end<W: Write>(&mut self, mut w: W, element: &Element) -> IOResult<()> {
        match element {
            // container elements
            Element::SpecialBlock(block) => {
                writeln!(&mut w, "#+END_{}", block.name)?;
                write_blank_lines(&mut w, block.post_blank)?;
            }
            Element::QuoteBlock(block) => {
                writeln!(&mut w, "#+END_QUOTE")?;
                write_blank_lines(&mut w, block.post_blank)?;
            }
            Element::CenterBlock(block) => {
                writeln!(&mut w, "#+END_CENTER")?;
                write_blank_lines(&mut w, block.post_blank)?;
            }
            Element::VerseBlock(block) => {
                writeln!(&mut w, "#+END_VERSE")?;
                write_blank_lines(&mut w, block.post_blank)?;
            }
            Element::Bold => write!(w, "*")?,
            Element::Document { .. } => (),
            Element::DynBlock(dyn_block) => {
                writeln!(w, "#+END:")?;
                write_blank_lines(w, dyn_block.post_blank)?;
            }
            Element::Headline { .. } => (),
            Element::List(list) => {
                write_blank_lines(w, list.post_blank)?;
            }
            Element::Italic => write!(w, "/")?,
            Element::ListItem(_) => (),
            Element::Paragraph { post_blank } => {
                write_blank_lines(w, post_blank + 1)?;
            }
            Element::Section => (),
            Element::Strike => write!(w, "+")?,
            Element::Underline => write!(w, "_")?,
            Element::Drawer(drawer) => {
                writeln!(&mut w, ":END:")?;
                write_blank_lines(&mut w, drawer.post_blank)?;
            }
            Element::Title(title) => {
                if !title.tags.is_empty() {
                    write!(&mut w, " :")?;
                    for tag in &title.tags {
                        write!(&mut w, "{}:", tag)?;
                    }
                }
                writeln!(&mut w)?;
                if let Some(planning) = &title.planning {
                    if let Some(scheduled) = &planning.scheduled {
                        write!(&mut w, "SCHEDULED: ")?;
                        write_timestamp(&mut w, &scheduled)?;
                    }
                    if let Some(deadline) = &planning.deadline {
                        if planning.scheduled.is_some() {
                            write!(&mut w, " ")?;
                        }
                        write!(&mut w, "DEADLINE: ")?;
                        write_timestamp(&mut w, &deadline)?;
                    }
                    if let Some(closed) = &planning.closed {
                        if planning.deadline.is_some() {
                            write!(&mut w, " ")?;
                        }
                        write!(&mut w, "CLOSED: ")?;
                        write_timestamp(&mut w, &closed)?;
                    }
                    writeln!(&mut w)?;
                }
                if !title.properties.is_empty() {
                    writeln!(&mut w, ":PROPERTIES:")?;
                    for (key, value) in title.properties.iter() {
                        writeln!(&mut w, ":{}: {}", key, value)?;
                    }
                    writeln!(&mut w, ":END:")?;
                }
                write_blank_lines(&mut w, title.post_blank)?;
            }
            Element::Table(Table::Org { post_blank, .. }) => {
                write_blank_lines(w, *post_blank)?;
            }
            Element::Table(Table::TableEl { post_blank, .. }) => {
                write_blank_lines(w, *post_blank)?;
            }
            Element::TableRow(_) => (),
            Element::TableCell(_) => (),
            // non-container elements
            _ => debug_assert!(!element.is_container()),
        }

        Ok(())
    }
}

fn write_blank_lines<W: Write>(mut w: W, count: usize) -> Result<(), Error> {
    for _ in 0..count {
        writeln!(w)?;
    }
    Ok(())
}

fn write_timestamp<W: Write>(mut w: W, timestamp: &Timestamp) -> Result<(), Error> {
    match timestamp {
        Timestamp::Active { start, .. } => {
            write_datetime(w, "<", start, ">")?;
        }
        Timestamp::Inactive { start, .. } => {
            write_datetime(w, "[", start, "]")?;
        }
        Timestamp::ActiveRange { start, end, .. } => {
            write_datetime(&mut w, "<", start, ">--")?;
            write_datetime(&mut w, "<", end, ">")?;
        }
        Timestamp::InactiveRange { start, end, .. } => {
            write_datetime(&mut w, "[", start, "]--")?;
            write_datetime(&mut w, "[", end, "]")?;
        }
        Timestamp::Diary { value } => write!(w, "<%%({})>", value)?,
    }
    Ok(())
}
