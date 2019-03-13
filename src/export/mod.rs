#[macro_use]
macro_rules! handle_event {
    ($event:expr, $handler:expr, $writer:expr) => {
        use crate::parser::Event::*;

        match $event {
            HeadlineBeg(hdl) => $handler.handle_headline_beg($writer, hdl)?,
            HeadlineEnd => $handler.handle_headline_end($writer)?,
            SectionBeg => $handler.handle_section_beg($writer)?,
            SectionEnd => $handler.handle_section_end($writer)?,
            ParagraphBeg => $handler.handle_paragraph_beg($writer)?,
            ParagraphEnd => $handler.handle_paragraph_end($writer)?,
            CtrBlockBeg => $handler.handle_ctr_block_beg($writer)?,
            CtrBlockEnd => $handler.handle_ctr_block_end($writer)?,
            QteBlockBeg => $handler.handle_qte_block_beg($writer)?,
            QteBlockEnd => $handler.handle_qte_block_end($writer)?,
            SplBlockBeg { name, args } => $handler.handle_spl_block_beg($writer, name, args)?,
            SplBlockEnd => $handler.handle_spl_block_end($writer)?,
            CommentBlock { cont, args } => $handler.handle_comment_block($writer, cont, args)?,
            ExampleBlock { cont, args } => $handler.handle_example_block($writer, cont, args)?,
            ExportBlock { cont, args } => $handler.handle_export_block($writer, cont, args)?,
            SrcBlock { cont, args } => $handler.handle_src_block($writer, cont, args)?,
            VerseBlock { cont, args } => $handler.handle_verse_block($writer, cont, args)?,
            DynBlockBeg { name, args } => $handler.handle_dyn_block_beg($writer, name, args)?,
            DynBlockEnd => $handler.handle_dyn_block_end($writer)?,
            ListBeg { ordered } => $handler.handle_list_beg($writer, ordered)?,
            ListEnd { ordered } => $handler.handle_list_end($writer, ordered)?,
            ListItemBeg { bullet } => $handler.handle_list_beg_item($writer, bullet)?,
            ListItemEnd => $handler.handle_list_end_item($writer)?,
            Call { value } => $handler.handle_call($writer, value)?,
            Clock => $handler.handle_clock($writer)?,
            Comment(c) => $handler.handle_comment($writer, c)?,
            FixedWidth(f) => $handler.handle_fixed_width($writer, f)?,
            TableStart => $handler.handle_table_start($writer)?,
            TableEnd => $handler.handle_table_end($writer)?,
            TableCell => $handler.handle_table_cell($writer)?,
            LatexEnv => $handler.handle_latex_env($writer)?,
            FnDef { label, cont } => $handler.handle_fn_def($writer, label, cont)?,
            Keyword { key, value } => $handler.handle_keyword($writer, key, value)?,
            Rule => $handler.handle_rule($writer)?,
            Cookie(cookie) => $handler.handle_cookie($writer, cookie)?,
            FnRef { label, def } => $handler.handle_fn_ref($writer, label, def)?,
            InlineSrc { lang, option, body } => {
                $handler.handle_inline_src($writer, lang, option, body)?
            }
            InlineCall {
                name,
                args,
                inside_header,
                end_header,
            } => $handler.handle_inline_call($writer, name, args, inside_header, end_header)?,
            Link { path, desc } => $handler.handle_link($writer, path, desc)?,
            Macros { name, args } => $handler.handle_macros($writer, name, args)?,
            RadioTarget { target } => $handler.handle_radio_target($writer, target)?,
            Snippet { name, value } => $handler.handle_snippet($writer, name, value)?,
            Target { target } => $handler.handle_target($writer, target)?,
            BoldBeg => $handler.handle_bold_beg($writer)?,
            BoldEnd => $handler.handle_bold_end($writer)?,
            ItalicBeg => $handler.handle_italic_beg($writer)?,
            ItalicEnd => $handler.handle_italic_end($writer)?,
            StrikeBeg => $handler.handle_strike_beg($writer)?,
            StrikeEnd => $handler.handle_strike_end($writer)?,
            UnderlineBeg => $handler.handle_underline_beg($writer)?,
            UnderlineEnd => $handler.handle_underline_end($writer)?,
            Verbatim(cont) => $handler.handle_verbatim($writer, cont)?,
            Code(cont) => $handler.handle_code($writer, cont)?,
            Text(cont) => $handler.handle_text($writer, cont)?,
        }
    };
}

mod html;

pub use html::*;
