mod detangle;
mod diff;
mod execute_src_block;
mod tangle;

use clap::{Parser, Subcommand};
use clap_verbosity_flag::{InfoLevel, LevelFilter as CLevelFilter, Verbosity};
use tracing::level_filters::LevelFilter;

#[derive(Debug, Parser)]
#[clap(name = "orgize-tools", version)]
pub struct App {
    #[clap(subcommand)]
    command: Command,

    #[command(flatten)]
    verbose: Verbosity<InfoLevel>,
}

#[derive(Debug, Subcommand)]
enum Command {
    #[clap(name = "tangle")]
    Tangle(tangle::Command),

    #[clap(name = "detangle")]
    Detangle(detangle::Command),

    #[clap(name = "execute-src-block")]
    ExecuteSrcBlock(execute_src_block::Command),
}

fn main() -> anyhow::Result<()> {
    let parsed = App::parse();

    tracing_subscriber::fmt()
        .with_max_level(match parsed.verbose.log_level_filter() {
            CLevelFilter::Off => LevelFilter::OFF,
            CLevelFilter::Error => LevelFilter::ERROR,
            CLevelFilter::Warn => LevelFilter::WARN,
            CLevelFilter::Info => LevelFilter::INFO,
            CLevelFilter::Debug => LevelFilter::DEBUG,
            CLevelFilter::Trace => LevelFilter::TRACE,
        })
        .without_time()
        .with_file(false)
        .with_line_number(false)
        .init();

    match parsed.command {
        Command::Tangle(cmd) => cmd.run(),
        Command::Detangle(cmd) => cmd.run(),
        Command::ExecuteSrcBlock(cmd) => cmd.run(),
    }
}
