use clap::Args;
use orgize::{
    export::{Container, Event, TraversalContext, Traverser},
    Org,
};
use std::path::PathBuf;

use crate::diff;

#[derive(Debug, Args)]
pub struct Command {
    path: Vec<PathBuf>,

    #[arg(short, long)]
    dry_run: bool,
}

impl Command {
    pub fn run(self) -> anyhow::Result<()> {
        for path in self.path {
            if !path.exists() {
                tracing::error!("{:?} is not existed", path);
            }

            let orgi = std::fs::read_to_string(&path)?;

            let mut t = DetangleTraverser {
                results: Vec::new(),
                org_file_path: path,
            };

            let org = Org::parse(&orgi);
            org.traverse(&mut t);

            if self.dry_run {
                diff::print(&orgi, t.results);
            } else {
                diff::write_to_file(&orgi, t.results, t.org_file_path)?;
            }
        }

        Ok(())
    }
}

struct DetangleTraverser {
    results: Vec<(usize, usize, String)>,
    org_file_path: PathBuf,
}

impl Traverser for DetangleTraverser {
    fn event(&mut self, event: Event, ctx: &mut TraversalContext) {
        match event {
            Event::Enter(Container::SourceBlock(block)) => {
                if let Ok(Some((start, end, content))) =
                    orgize_common::detangle(block, &self.org_file_path)
                {
                    self.results.push((start, end, content));
                }

                ctx.skip();
            }

            // skip some containers for performance
            Event::Enter(Container::List(_))
            | Event::Enter(Container::OrgTable(_))
            | Event::Enter(Container::SpecialBlock(_))
            | Event::Enter(Container::QuoteBlock(_))
            | Event::Enter(Container::CenterBlock(_))
            | Event::Enter(Container::VerseBlock(_))
            | Event::Enter(Container::CommentBlock(_))
            | Event::Enter(Container::ExampleBlock(_))
            | Event::Enter(Container::ExportBlock(_)) => {
                ctx.skip();
            }

            _ => {}
        }
    }
}
