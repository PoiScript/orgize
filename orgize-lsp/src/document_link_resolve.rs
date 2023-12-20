use orgize::export::{Container, Event, TraversalContext, Traverser};
use tower_lsp::lsp_types::{DocumentLink, Url};

use crate::{org_document::OrgDocument, Backend};

pub fn document_link_resolve(
    document_link: &DocumentLink,
    backend: &Backend,
) -> Option<DocumentLink> {
    // don't need to resolve
    if document_link.target.is_some() {
        return None;
    }

    let data = document_link.data.as_ref()?;
    let data = data.as_array()?;

    match (
        data.get(0)?.as_str()?,
        data.get(1)?.as_str()?,
        data.get(2)?.as_str()?,
    ) {
        ("headline-id", url, id) => {
            let mut parsed = Url::parse(url).ok()?;

            let doc = backend.documents.get(url)?;
            let mut h = HeadlineIdTraverser {
                id: id.to_string(),
                line_number: None,
                doc: &doc,
            };
            doc.traverse(&mut h);
            if let Some(line) = h.line_number.take() {
                // results is zero-based
                parsed.set_fragment(Some(&(line + 1).to_string()));
                Some(DocumentLink {
                    target: Some(parsed),
                    data: None,
                    tooltip: None,
                    range: document_link.range,
                })
            } else {
                Some(DocumentLink {
                    target: Some(parsed),
                    data: None,
                    tooltip: None,
                    range: document_link.range,
                })
            }
        }
        _ => None,
    }
}

struct HeadlineIdTraverser<'a> {
    id: String,
    line_number: Option<u32>,
    doc: &'a OrgDocument,
}

impl<'a> Traverser for HeadlineIdTraverser<'a> {
    fn event(&mut self, event: Event, ctx: &mut TraversalContext) {
        if self.line_number.is_some() {
            return ctx.stop();
        }

        match event {
            Event::Enter(Container::Document(_)) => {}
            Event::Enter(Container::Headline(headline)) => {
                let slug = orgize_common::headline_slug(&headline);

                if slug == self.id {
                    let line = self.doc.line_of(headline.begin());
                    self.line_number = Some(line + 1)
                }
            }
            Event::Enter(Container::Section(_)) => ctx.skip(),
            _ => {}
        }
    }
}
