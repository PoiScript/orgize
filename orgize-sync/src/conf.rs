use app_dirs::{app_root, AppDataType, AppInfo};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use crate::error::Result;

#[derive(Serialize, Deserialize)]
pub struct Conf {
    pub files: Vec<File>,
    #[cfg(feature = "google_calendar")]
    pub google_calendar: Option<GoogleCalendarGlobalConf>,
}

#[derive(Serialize, Deserialize)]
#[cfg(feature = "google_calendar")]
pub struct GoogleCalendarGlobalConf {
    pub client_id: String,
    pub client_secret: String,
    pub token_dir: String,
    pub token_filename: String,
    pub property: String,
}

#[derive(Serialize, Deserialize)]
#[cfg(feature = "google_calendar")]
pub struct GoogleCalendarConf {
    pub calendar: String,
    pub append_new: bool,
    pub append_headline: String,
}

#[derive(Serialize, Deserialize)]
pub struct File {
    pub path: String,
    pub name: String,
    #[cfg(feature = "google_calendar")]
    pub google_calendar: Option<GoogleCalendarConf>,
}

pub fn default_conf_path() -> Result<PathBuf> {
    let mut path = app_root(
        AppDataType::UserConfig,
        &AppInfo {
            name: "orgize-sync",
            author: "PoiScript",
        },
    )?;
    path.push("conf.toml");
    Ok(path)
}
