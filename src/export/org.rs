use crate::elements::Element;
use std::io::{Error, Write};

pub trait OrgHandler<E: From<Error>> {
    fn start<W: Write>(&mut self, mut w: W, element: &Element) -> Result<(), E> {
        use Element::*;

        match element {
            // container elements
            Block(block) => {
                write!(&mut w, "#+BEGIN_{}", block.name)?;
                if let Some(parameters) = block.args {
                    write!(&mut w, " {}", parameters)?;
                }
                writeln!(&mut w)?;
            }
            Bold => write!(w, "*")?,
            Document => (),
            DynBlock(dyn_block) => {
                write!(&mut w, "#+BEGIN: {}", dyn_block.block_name)?;
                if let Some(parameters) = dyn_block.arguments {
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
            BabelCall(_babel_call) => (),
            InlineSrc(inline_src) => {
                write!(&mut w, "src_{}", inline_src.lang)?;
                if let Some(options) = inline_src.options {
                    write!(&mut w, "[{}]", options)?;
                }
                write!(&mut w, "{{{}}}", inline_src.body)?;
            }
            Code { value } => write!(w, "~{}~", value)?,
            FnRef(fn_ref) => {
                write!(&mut w, "[fn:")?;
                if let Some(label) = fn_ref.label {
                    write!(&mut w, "{}", label)?;
                }
                if let Some(definition) = fn_ref.definition {
                    write!(&mut w, ":{}", definition)?;
                }
                write!(&mut w, "]")?;
            }
            InlineCall(inline_call) => {
                write!(&mut w, "call_{}", inline_call.name)?;
                if let Some(header) = inline_call.inside_header {
                    write!(&mut w, "[{}]", header)?;
                }
                write!(&mut w, "({})", inline_call.arguments)?;
                if let Some(header) = inline_call.end_header {
                    write!(&mut w, "[{}]", header)?;
                }
            }
            Link(link) => {
                write!(&mut w, "[[{}]", link.path)?;
                if let Some(desc) = link.desc {
                    write!(&mut w, "[{}]", desc)?;
                }
                write!(&mut w, "]")?;
            }
            Macros(_macros) => (),
            Planning(_planning) => (),
            RadioTarget(_radio_target) => (),
            Snippet(snippet) => write!(w, "@@{}:{}@@", snippet.name, snippet.value)?,
            Target(_target) => (),
            Text { value } => write!(w, "{}", value)?,
            Timestamp(timestamp) => {
                use crate::elements::{Date, Time, Timestamp::*};

                fn write_date<W: Write>(mut w: W, date: &Date) -> Result<(), Error> {
                    write!(
                        w,
                        "{}-{}-{} {}",
                        date.year, date.month, date.day, date.dayname
                    )
                }

                fn write_time<W: Write>(mut w: W, time: &Option<Time>) -> Result<(), Error> {
                    if let Some(time) = time {
                        write!(w, " {}:{}", time.hour, time.minute)
                    } else {
                        Ok(())
                    }
                }

                match timestamp {
                    Active {
                        start_date,
                        start_time,
                        ..
                    } => {
                        write!(&mut w, "<")?;
                        write_date(&mut w, start_date)?;
                        write_time(&mut w, start_time)?;
                        write!(&mut w, ">")?;
                    }
                    Inactive {
                        start_date,
                        start_time,
                        ..
                    } => {
                        write!(&mut w, "[")?;
                        write_date(&mut w, start_date)?;
                        write_time(&mut w, start_time)?;
                        write!(&mut w, "]")?;
                    }
                    ActiveRange {
                        start_date,
                        start_time,
                        end_date,
                        end_time,
                        ..
                    } => {
                        write!(&mut w, "<")?;
                        write_date(&mut w, start_date)?;
                        write_time(&mut w, start_time)?;
                        write!(&mut w, ">--<")?;
                        write_date(&mut w, end_date)?;
                        write_time(&mut w, end_time)?;
                        write!(&mut w, ">")?;
                    }
                    InactiveRange {
                        start_date,
                        start_time,
                        end_date,
                        end_time,
                        ..
                    } => {
                        write!(&mut w, "[")?;
                        write_date(&mut w, start_date)?;
                        write_time(&mut w, start_time)?;
                        write!(&mut w, "]--[")?;
                        write_date(&mut w, end_date)?;
                        write_time(&mut w, end_time)?;
                        write!(&mut w, "]")?;
                    }
                    Diary(value) => write!(w, "<%%({})>", value)?,
                }
            }
            Verbatim { value } => write!(w, "={}=", value)?,
            FnDef(_fn_def) => (),
            Clock(_clock) => (),
            Comment { value } => write!(w, "{}", value)?,
            FixedWidth { value } => write!(w, "{}", value)?,
            Keyword(keyword) => {
                write!(&mut w, "#+{}", keyword.key)?;
                if let Some(optional) = keyword.optional {
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
                if let Some(keyword) = title.keyword {
                    write!(&mut w, " {}", keyword)?;
                }
                if let Some(priority) = title.priority {
                    write!(&mut w, " [#{}]", priority)?;
                }
                write!(&mut w, " ")?;
            }
        }

        Ok(())
    }

    fn end<W: Write>(&mut self, mut w: W, element: &Element) -> Result<(), E> {
        use Element::*;

        match element {
            // container elements
            Block(block) => writeln!(w, "#+END_{}", block.name)?,
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
            }
            // non-container elements
            _ => debug_assert!(!element.is_container()),
        }

        Ok(())
    }
}

pub struct DefaultOrgHandler;

impl OrgHandler<Error> for DefaultOrgHandler {}
