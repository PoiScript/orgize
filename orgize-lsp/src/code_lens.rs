use orgize::{
    export::{Container, Event, TraversalContext, Traverser},
    rowan::ast::AstNode,
};
use orgize_common::{header_argument, property_drawer, property_keyword};
use tower_lsp::lsp_types::{CodeLens, Url};

use crate::org_document::OrgDocument;

use super::OrgizeCommand;

pub struct CodeLensTraverser<'a> {
    pub url: Url,
    pub doc: &'a OrgDocument,
    pub lens: Vec<CodeLens>,
}

impl<'a> Traverser for CodeLensTraverser<'a> {
    fn event(&mut self, event: Event, ctx: &mut TraversalContext) {
        match event {
            Event::Enter(Container::SourceBlock(block)) => {
                let start = block.begin();

                let arg1 = block.parameters().unwrap_or_default();
                let arg2 = property_drawer(block.syntax()).unwrap_or_default();
                let arg3 = property_keyword(block.syntax()).unwrap_or_default();

                let range = self.doc.range_of(start, start);

                let tangle = header_argument(&arg1, &arg2, &arg3, ":tangle", "no");

                if header_argument(&arg1, &arg2, &arg3, ":results", "no") != "no" {
                    self.lens.push(CodeLens {
                        range,
                        command: Some(
                            OrgizeCommand::SrcBlockExecute {
                                block_offset: start,
                                url: self.url.clone(),
                            }
                            .into(),
                        ),
                        data: None,
                    });
                }

                if tangle != "no" {
                    self.lens.push(CodeLens {
                        range,
                        command: Some(
                            OrgizeCommand::SrcBlockTangle {
                                block_offset: start,
                                url: self.url.clone(),
                            }
                            .into(),
                        ),
                        data: None,
                    });

                    self.lens.push(CodeLens {
                        range,
                        command: Some(
                            OrgizeCommand::SrcBlockDetangle {
                                block_offset: start,
                                url: self.url.clone(),
                            }
                            .into(),
                        ),
                        data: None,
                    });
                }

                ctx.skip();
            }
            Event::Enter(Container::Headline(headline)) => {
                if headline.tags().any(|t| t.eq_ignore_ascii_case("TOC")) {
                    let start = headline.begin();

                    self.lens.push(CodeLens {
                        range: self.doc.range_of(start, start),
                        command: Some(
                            OrgizeCommand::HeadlineToc {
                                heading_offset: start,
                                url: self.url.clone(),
                            }
                            .into(),
                        ),
                        data: None,
                    });
                }
            }
            _ => {}
        }
    }
}

impl<'a> CodeLensTraverser<'a> {
    pub fn new(url: Url, doc: &'a OrgDocument) -> Self {
        CodeLensTraverser {
            url,
            lens: vec![],
            doc,
        }
    }
}
