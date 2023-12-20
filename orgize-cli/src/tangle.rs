use clap::{
    builder::styling::{AnsiColor, Color, Style},
    Args,
};
use orgize::{
    export::{Container, Event, TraversalContext, Traverser},
    Org,
};
use std::fs;
use std::{collections::HashMap, path::PathBuf};

#[derive(Debug, Args)]
pub struct Command {
    path: Vec<PathBuf>,

    #[arg(short, long)]
    dry_run: bool,
}

impl Command {
    pub fn run(self) -> anyhow::Result<()> {
        let mut t = TangleTraverser::default();

        for path in self.path {
            if !path.exists() {
                tracing::error!("{:?} is not existed", path);
            }

            let string = std::fs::read_to_string(&path)?;
            let org = Org::parse(string);
            t.org_file_path = path;
            t.count = 0;
            org.traverse(&mut t);
            tracing::info!(
                "Found {} code block from {}",
                t.count,
                t.org_file_path.to_string_lossy()
            );
        }

        if self.dry_run {
            for (path, (permission, content, mkdir)) in t.results {
                let style = Style::new()
                    .fg_color(Color::Ansi(AnsiColor::BrightYellow).into())
                    .underline()
                    .bold();
                print!(
                    "{}{}{}",
                    style.render(),
                    path.to_string_lossy(),
                    style.render_reset(),
                );
                if let Some(permission) = permission {
                    print!(" (permission: {:o})", permission);
                }
                if mkdir {
                    print!(" (mkdir: yes)");
                }
                println!("\n{}", content);
            }
        } else {
            for (path, (_, contents, _)) in t.results {
                fs::write(&path, contents)?;
                tracing::info!("Wrote to {}", path.to_string_lossy());
            }
        }

        Ok(())
    }
}

#[derive(Default)]
struct TangleTraverser {
    results: HashMap<PathBuf, (Option<u32>, String, bool)>,
    count: usize,
    org_file_path: PathBuf,
}

impl Traverser for TangleTraverser {
    fn event(&mut self, event: Event, ctx: &mut TraversalContext) {
        match event {
            Event::Enter(Container::SourceBlock(block)) => {
                if let Ok(Some((path, permission, content, mkdir))) =
                    orgize_common::tangle(block, &self.org_file_path)
                {
                    let value = self.results.entry(path).or_default();
                    value.0 = permission;
                    value.1.push_str(&content);
                    value.2 = mkdir;
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
