mod conf;
mod error;

use std::path::PathBuf;
use structopt::StructOpt;

use crate::conf::default_conf_path;
use crate::error::Result;

#[derive(StructOpt, Debug)]
#[structopt(name = "orgize-sync")]
struct Opt {
    #[structopt(subcommand)]
    subcommand: Cmd,
}

#[derive(StructOpt, Debug)]
enum Cmd {
    #[structopt(name = "init")]
    Init,
    #[structopt(name = "sync")]
    Sync {
        #[structopt(long = "skip-google-calendar")]
        skip_google_calendar: bool,
        #[structopt(long = "skip-toggl")]
        skip_toggl: bool,
        #[structopt(short = "c", long = "conf", parse(from_os_str))]
        conf_path: Option<PathBuf>,
    },
    #[structopt(name = "conf")]
    Conf,
}

fn main() -> Result<()> {
    let opt = Opt::from_args();

    match opt.subcommand {
        Cmd::Sync {
            conf_path,
            skip_google_calendar,
            skip_toggl,
        } => {
            let conf_path = conf_path
                .map(Result::Ok)
                .unwrap_or_else(default_conf_path)?;

            println!("{:#?}", conf_path);

            if cfg!(feature = "google_calendar") && !skip_google_calendar {}

            if cfg!(feature = "toggl") && !skip_toggl {}
        }
        Cmd::Init => (),
        Cmd::Conf => (),
    }

    Ok(())
}
