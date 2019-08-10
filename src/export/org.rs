use super::write_datetime;
use crate::elements::{Element, Timestamp};
use std::io::{Error, Write};

pub trait OrgHandler<E: From<Error>> {
    fn start<W: Write>(&mut self, mut w: W, element: &Element) -> Result<(), E> {
        use Element::*;

        match element {
            // container elements
            SpecialBlock(block) => writeln!(w, "#+BEGIN_{}", block.name)?,
            QuoteBlock(_) => write!(w, "#+BEGIN_QUOTE")?,
            CenterBlock(_) => write!(w, "#+BEGIN_CENTER")?,
            VerseBlock(_) => write!(w, "#+BEGIN_VERSE")?,
            Bold => write!(w, "*")?,
            Document => (),
            DynBlock(dyn_block) => {
                write!(&mut w, "#+BEGIN: {}", dyn_block.block_name)?;
                if let Some(parameters) = &dyn_block.arguments {
                    write!(&mut w, " {}", parameters)?;
                }
                writeln!(&mut w)?;
            }
            Headline => (),
            List(_list) => (),
            Italic => write!(w, "/")?,
            ListItem(list_item) => write!(w, "{}", list_item.bullet)?,
            Paragraph => (),
            Section => (),
            Strike => write!(w, "+")?,
            Underline => write!(w, "_")?,
            Drawer(drawer) => writeln!(w, ":{}:", drawer.name)?,
            // non-container elements
            CommentBlock(block) => {
                writeln!(w, "#+BEGIN_COMMENT\n{}\n#+END_COMMENT", block.contents)?
            }
            ExampleBlock(block) => {
                writeln!(w, "#+BEGIN_EXAMPLE\n{}\n#+END_EXAMPLE", block.contents)?
            }
            ExportBlock(block) => writeln!(
                w,
                "#+BEGIN_EXPORT {}\n{}\n#+END_EXPORT",
                block.data, block.contents
            )?,
            SourceBlock(block) => writeln!(
                w,
                "#+BEGIN_SRC {}\n{}\n#+END_SRC",
                block.language, block.contents
            )?,
            BabelCall(_babel_call) => (),
            InlineSrc(inline_src) => {
                write!(&mut w, "src_{}", inline_src.lang)?;
                if let Some(options) = &inline_src.options {
                    write!(&mut w, "[{}]", options)?;
                }
                write!(&mut w, "{{{}}}", inline_src.body)?;
            }
            Code { value } => write!(w, "~{}~", value)?,
            FnRef(fn_ref) => {
                write!(&mut w, "[fn:{}", fn_ref.label)?;
                if let Some(definition) = &fn_ref.definition {
                    write!(&mut w, ":{}", definition)?;
                }
                write!(&mut w, "]")?;
            }
            InlineCall(inline_call) => {
                write!(&mut w, "call_{}", inline_call.name)?;
                if let Some(header) = &inline_call.inside_header {
                    write!(&mut w, "[{}]", header)?;
                }
                write!(&mut w, "({})", inline_call.arguments)?;
                if let Some(header) = &inline_call.end_header {
                    write!(&mut w, "[{}]", header)?;
                }
            }
            Link(link) => {
                write!(&mut w, "[[{}]", link.path)?;
                if let Some(desc) = &link.desc {
                    write!(&mut w, "[{}]", desc)?;
                }
                write!(&mut w, "]")?;
            }
            Macros(_macros) => (),
            RadioTarget => (),
            Snippet(snippet) => write!(w, "@@{}:{}@@", snippet.name, snippet.value)?,
            Target(_target) => (),
            Text { value } => write!(w, "{}", value)?,
            Timestamp(timestamp) => {
                write_timestamp(&mut w, &timestamp)?;
            }
            Verbatim { value } => write!(w, "={}=", value)?,
            FnDef(_fn_def) => (),
            Clock(clock) => {
                use crate::elements::Clock;

                write!(w, "CLOCK: ")?;

                match clock {
                    Clock::Closed {
                        start,
                        end,
                        duration,
                        ..
                    } => {
                        write_datetime(&mut w, "[", start, "]--")?;
                        write_datetime(&mut w, "[", end, "]")?;
                        writeln!(w, " => {}", duration)?;
                    }
                    Clock::Running { start, .. } => {
                        write_datetime(&mut w, "[", start, "]\n")?;
                    }
                }
            }
            Comment { value } => write!(w, "{}", value)?,
            FixedWidth { value } => write!(w, "{}", value)?,
            Keyword(keyword) => {
                write!(&mut w, "#+{}", keyword.key)?;
                if let Some(optional) = &keyword.optional {
                    write!(&mut w, "[{}]", optional)?;
                }
                writeln!(&mut w, ": {}", keyword.value)?;
            }
            Rule => writeln!(w, "-----")?,
            Cookie(_cookie) => (),
            Title(title) => {
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
            Table(_) => (),
            TableRow(_) => (),
            TableCell => (),
        }

        Ok(())
    }

    fn end<W: Write>(&mut self, mut w: W, element: &Element) -> Result<(), E> {
        use Element::*;

        match element {
            // container elements
            SpecialBlock(block) => writeln!(w, "#+END_{}", block.name)?,
            QuoteBlock(_) => writeln!(w, "#+END_QUOTE")?,
            CenterBlock(_) => writeln!(w, "#+END_CENTER")?,
            VerseBlock(_) => writeln!(w, "#+END_VERSE")?,
            Bold => write!(w, "*")?,
            Document => (),
            DynBlock(_dyn_block) => writeln!(w, "#+END:")?,
            Headline => (),
            List(_list) => (),
            Italic => write!(w, "/")?,
            ListItem(_) => (),
            Paragraph => write!(w, "\n\n")?,
            Section => (),
            Strike => write!(w, "+")?,
            Underline => write!(w, "_")?,
            Drawer(_) => writeln!(w, ":END:")?,
            Title(title) => {
                if !title.tags.is_empty() {
                    write!(&mut w, " :")?;
                }
                for tag in &title.tags {
                    write!(&mut w, "{}:", tag)?;
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
                    for (key, value) in &title.properties {
                        writeln!(&mut w, ":{}: {}", key, value)?;
                    }
                    writeln!(&mut w, ":END:")?;
                }
            }
            Table(_) => (),
            TableRow(_) => (),
            TableCell => (),
            // non-container elements
            _ => debug_assert!(!element.is_container()),
        }

        Ok(())
    }
}

fn write_timestamp<W: Write>(mut w: W, timestamp: &Timestamp) -> std::io::Result<()> {
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

pub struct DefaultOrgHandler;

impl OrgHandler<Error> for DefaultOrgHandler {}
