mod conf;
mod error;

use std::fs;
use std::path::PathBuf;
use structopt::StructOpt;
use toml::to_string_pretty;

use crate::conf::{
    default_config_path, default_env_path, user_cache_path, user_config_path, Conf, EnvConf,
};
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
    Conf {
        #[structopt(short = "c", long = "conf", parse(from_os_str))]
        conf_path: Option<PathBuf>,
    },
}

fn main() -> Result<()> {
    let opt = Opt::from_args();

    match opt.subcommand {
        Cmd::Init => {
            fs::create_dir_all(user_config_path())?;
            fs::create_dir_all(user_cache_path())?;

            let default_env_path = default_env_path();
            let default_config_path = default_config_path();

            if default_env_path.as_path().exists() {
                println!(
                    "{} already existed, skipping ...",
                    default_env_path.as_path().display()
                );
            } else {
                println!("Creating {} ...", default_env_path.as_path().display());
                fs::write(default_env_path.clone(), "")?;
            }

            if default_config_path.as_path().exists() {
                println!(
                    "{} already existed, skipping ...",
                    default_config_path.as_path().display()
                );
            } else {
                println!("Creating {} ...", default_config_path.as_path().display());
                fs::write(
                    default_config_path,
                    to_string_pretty(&EnvConf {
                        env_path: default_env_path,
                    })?,
                )?;
            }
        }
        Cmd::Sync {
            conf_path,
            skip_google_calendar,
            skip_toggl,
        } => {
            let conf = Conf::new(conf_path)?;

            if cfg!(feature = "google_calendar") && !skip_google_calendar {}

            if cfg!(feature = "toggl") && !skip_toggl {}
        }
        Cmd::Conf { conf_path } => {
            let conf = Conf::new(conf_path)?;

            println!("{}", to_string_pretty(&conf)?);
        }
    }

    Ok(())
}
