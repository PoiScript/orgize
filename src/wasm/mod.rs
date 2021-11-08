#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

use serde::Serialize;
use serde_wasm_bindgen::Serializer;
use wasm_bindgen::prelude::*;

use crate::{Element, Event};

#[wasm_bindgen]
pub struct Org(crate::Org<'static>);

#[wasm_bindgen]
impl Org {
    #[wasm_bindgen]
    pub fn parse(input: String) -> Self {
        Org(crate::Org::parse_string(input))
    }

    #[wasm_bindgen(js_name = toJson)]
    pub fn to_json(&self) -> JsValue {
        to_value(&self.0)
    }
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(skip_typescript)]
    pub type Handler;

    #[wasm_bindgen(method)]
    pub fn text(this: &Handler, text: JsValue);
    #[wasm_bindgen(method)]
    pub fn code(this: &Handler, item: JsValue);
    #[wasm_bindgen(method)]
    pub fn cookie(this: &Handler, item: JsValue);
    #[wasm_bindgen(method)]
    pub fn rule(this: &Handler);
    #[wasm_bindgen(method, js_name = exampleBlock)]
    pub fn example_block(this: &Handler, item: JsValue);
    #[wasm_bindgen(method, js_name = exportBlock)]
    pub fn export_block(this: &Handler, item: JsValue);
    #[wasm_bindgen(method, js_name = sourceBlock)]
    pub fn source_block(this: &Handler, item: JsValue);
    #[wasm_bindgen(method, js_name = inlineSrc)]
    pub fn inline_src(this: &Handler, item: JsValue);
    #[wasm_bindgen(method)]
    pub fn link(this: &Handler, item: JsValue);
    #[wasm_bindgen(method)]
    pub fn snippet(this: &Handler, item: JsValue);
    #[wasm_bindgen(method)]
    pub fn timestamp(this: &Handler, item: JsValue);
    #[wasm_bindgen(method)]
    pub fn verbatim(this: &Handler, item: JsValue);
    #[wasm_bindgen(method)]
    pub fn fixedWidth(this: &Handler, item: JsValue);
    #[wasm_bindgen(method)]
    pub fn keyword(this: &Handler, item: JsValue);

    #[wasm_bindgen(method, js_name = listStart)]
    pub fn list_start(this: &Handler, item: JsValue);
    #[wasm_bindgen(method, js_name = listEnd)]
    pub fn list_end(this: &Handler, item: JsValue);
    #[wasm_bindgen(method, js_name = tableStart)]
    pub fn table_start(this: &Handler, item: JsValue);
    #[wasm_bindgen(method, js_name = tableEnd)]
    pub fn table_end(this: &Handler, item: JsValue);
    #[wasm_bindgen(method, js_name = tableRowStart)]
    pub fn table_row_start(this: &Handler, item: JsValue);
    #[wasm_bindgen(method, js_name = tableRowEnd)]
    pub fn table_row_end(this: &Handler, item: JsValue);
    #[wasm_bindgen(method, js_name = tableCellStart)]
    pub fn table_cell_start(this: &Handler, item: JsValue);
    #[wasm_bindgen(method, js_name = tableCellEnd)]
    pub fn table_cell_end(this: &Handler, item: JsValue);
    #[wasm_bindgen(method, js_name = titleStart)]
    pub fn title_start(this: &Handler, item: JsValue);
    #[wasm_bindgen(method, js_name = titleEnd)]
    pub fn title_end(this: &Handler, item: JsValue);
    #[wasm_bindgen(method, js_name = boldStart)]
    pub fn bold_start(this: &Handler);
    #[wasm_bindgen(method, js_name = boldEnd)]
    pub fn bold_end(this: &Handler);
    #[wasm_bindgen(method, js_name = centerBlockStart)]
    pub fn center_block_start(this: &Handler, item: JsValue);
    #[wasm_bindgen(method, js_name = centerBlockEnd)]
    pub fn center_block_end(this: &Handler, item: JsValue);
    #[wasm_bindgen(method, js_name = documentStart)]
    pub fn document_start(this: &Handler);
    #[wasm_bindgen(method, js_name = documentEnd)]
    pub fn document_end(this: &Handler);
    #[wasm_bindgen(method, js_name = italicStart)]
    pub fn italic_start(this: &Handler);
    #[wasm_bindgen(method, js_name = italicEnd)]
    pub fn italic_end(this: &Handler);
    #[wasm_bindgen(method, js_name = listItemStart)]
    pub fn list_item_start(this: &Handler);
    #[wasm_bindgen(method, js_name = listItemEnd)]
    pub fn list_item_end(this: &Handler);
    #[wasm_bindgen(method, js_name = paragraphStart)]
    pub fn paragraph_start(this: &Handler);
    #[wasm_bindgen(method, js_name = paragraphEnd)]
    pub fn paragraph_end(this: &Handler);
    #[wasm_bindgen(method, js_name = quoteBlockStart)]
    pub fn quote_block_start(this: &Handler, item: JsValue);
    #[wasm_bindgen(method, js_name = quoteBlockEnd)]
    pub fn quote_block_end(this: &Handler, item: JsValue);
    #[wasm_bindgen(method, js_name = sectionStart)]
    pub fn section_start(this: &Handler);
    #[wasm_bindgen(method, js_name = sectionEnd)]
    pub fn section_end(this: &Handler);
    #[wasm_bindgen(method, js_name = strikeStart)]
    pub fn strike_start(this: &Handler);
    #[wasm_bindgen(method, js_name = strikeEnd)]
    pub fn strike_end(this: &Handler);
    #[wasm_bindgen(method, js_name = underlineStart)]
    pub fn underline_start(this: &Handler);
    #[wasm_bindgen(method, js_name = underlineEnd)]
    pub fn underline_end(this: &Handler);
    #[wasm_bindgen(method, js_name = verseBlockStart)]
    pub fn verse_block_start(this: &Handler, item: JsValue);
    #[wasm_bindgen(method, js_name = verseBlockEnd)]
    pub fn verse_block_end(this: &Handler, item: JsValue);
}

#[wasm_bindgen]
pub fn handle(org: &Org, handler: Handler) {
    for event in org.0.iter() {
        use Element::*;

        match event {
            Event::Start(Text { value }) => handler.text(JsValue::from_str(value)),
            Event::Start(ExampleBlock(block)) => handler.example_block(to_value(block)),
            Event::Start(ExportBlock(block)) => handler.export_block(to_value(block)),
            Event::Start(SourceBlock(block)) => handler.source_block(to_value(block)),
            Event::Start(InlineSrc(src)) => handler.inline_src(to_value(src)),
            Event::Start(Code { value }) => handler.code(JsValue::from_str(value)),
            Event::Start(Link(link)) => handler.link(to_value(link)),
            Event::Start(Snippet(snippet)) => handler.snippet(to_value(snippet)),
            Event::Start(Timestamp(timestamp)) => handler.timestamp(to_value(timestamp)),
            Event::Start(Verbatim { value }) => handler.verbatim(JsValue::from_str(value)),
            Event::Start(FixedWidth(fixed_width)) => handler.fixedWidth(to_value(fixed_width)),
            Event::Start(Rule(_)) => handler.rule(),
            Event::Start(Cookie(cookie)) => handler.cookie(to_value(cookie)),
            Event::Start(Keyword(keyword)) => handler.keyword(to_value(keyword)),

            Event::Start(Table(table)) => handler.table_start(to_value(table)),
            Event::End(Table(table)) => handler.table_start(to_value(table)),
            Event::Start(TableRow(row)) => handler.table_row_start(to_value(row)),
            Event::End(TableRow(row)) => handler.table_row_start(to_value(row)),
            Event::Start(TableCell(cell)) => handler.table_cell_start(to_value(cell)),
            Event::End(TableCell(cell)) => handler.table_cell_start(to_value(cell)),
            Event::Start(Title(title)) => handler.title_start(to_value(title)),
            Event::End(Title(title)) => handler.title_end(to_value(title)),
            Event::Start(QuoteBlock(block)) => handler.quote_block_start(to_value(block)),
            Event::End(QuoteBlock(block)) => handler.quote_block_end(to_value(block)),
            Event::Start(CenterBlock(block)) => handler.center_block_start(to_value(block)),
            Event::End(CenterBlock(block)) => handler.center_block_end(to_value(block)),
            Event::Start(VerseBlock(block)) => handler.verse_block_start(to_value(block)),
            Event::End(VerseBlock(block)) => handler.verse_block_end(to_value(block)),
            Event::Start(Bold) => handler.bold_start(),
            Event::End(Bold) => handler.bold_end(),
            Event::Start(Document { .. }) => handler.document_start(),
            Event::End(Document { .. }) => handler.document_end(),
            Event::Start(List(list)) => handler.list_start(to_value(list)),
            Event::End(List(list)) => handler.list_end(to_value(list)),
            Event::Start(Italic) => handler.italic_start(),
            Event::End(Italic) => handler.italic_end(),
            Event::Start(ListItem(_)) => handler.list_item_start(),
            Event::End(ListItem(_)) => handler.list_item_end(),
            Event::Start(Paragraph { .. }) => handler.paragraph_start(),
            Event::End(Paragraph { .. }) => handler.paragraph_end(),
            Event::Start(Section) => handler.section_start(),
            Event::End(Section) => handler.section_end(),
            Event::Start(Strike) => handler.strike_start(),
            Event::End(Strike) => handler.strike_end(),
            Event::Start(Underline) => handler.underline_start(),
            Event::End(Underline) => handler.underline_end(),

            _ => continue,
        };
    }
}

pub fn to_value<T: Serialize + ?Sized>(value: &T) -> JsValue {
    value
        .serialize(&Serializer::new().serialize_maps_as_objects(true))
        .unwrap()
}
