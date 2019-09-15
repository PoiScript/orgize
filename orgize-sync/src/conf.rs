use std::env;
use std::fs;
use std::path::PathBuf;

use app_dirs::{app_root, AppDataType, AppInfo};
use serde::{Deserialize, Serialize};

pub use crate::conf::google_calendar::*;
use crate::error::Result;

const APP_INFO: AppInfo = AppInfo {
    name: "orgize-sync",
    author: "PoiScript",
};

#[derive(Serialize, Deserialize, Debug)]
pub struct Conf {
    #[serde(default = "default_env_path")]
    pub env_path: PathBuf,
    #[serde(default)]
    pub files: Vec<File>,
    #[cfg(feature = "google_calendar")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub google_calendar: Option<GoogleCalendarGlobalConf>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EnvConf {
    #[serde(default = "default_env_path")]
    pub env_path: PathBuf,
}

pub fn user_config_path() -> PathBuf {
    app_root(AppDataType::UserConfig, &APP_INFO).unwrap()
}

pub fn user_cache_path() -> PathBuf {
    app_root(AppDataType::UserCache, &APP_INFO).unwrap()
}

pub fn default_config_path() -> PathBuf {
    let mut path = user_config_path();
    path.push("conf.toml");
    path
}

pub fn default_env_path() -> PathBuf {
    let mut path = user_cache_path();
    path.push(".env");
    path
}

impl Conf {
    pub fn new(path: Option<PathBuf>) -> Result<Self> {
        let path = path.unwrap_or_else(default_config_path);

        let content = fs::read(&path).expect(&format!(
            "Failed to read file: {}",
            path.as_path().display()
        ));

        if cfg!(feature = "dotenv") {
            let env_conf: EnvConf = toml::from_slice(&content)?;
            dotenv::from_path(env_conf.env_path.as_path())?;
        }

        Ok(toml::from_slice(&content)?)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct File {
    pub path: String,
    pub name: Option<String>,
    #[cfg(feature = "google_calendar")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub google_calendar: Option<GoogleCalendarConf>,
}

#[cfg(feature = "google_calendar")]
pub mod google_calendar {
    use super::*;

    #[derive(Serialize, Deserialize, Debug)]
    pub struct GoogleCalendarGlobalConf {
        #[serde(default = "default_client_id")]
        pub client_id: String,
        #[serde(default = "default_client_secret")]
        pub client_secret: String,
        #[serde(default = "default_token_dir")]
        pub token_dir: PathBuf,
        #[serde(default = "default_token_filename")]
        pub token_filename: String,
        #[serde(default = "default_property")]
        pub property: String,
        #[serde(default = "default_redirect_uri")]
        pub redirect_uri: String,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct GoogleCalendarConf {
        pub calendar: String,
        #[serde(default)]
        pub append_new: bool,
        #[serde(default = "default_append_headline")]
        pub append_headline: String,
    }

    fn default_client_id() -> String {
        env::var("GOOGLE_CLIENT_ID").unwrap()
    }

    fn default_client_secret() -> String {
        env::var("GOOGLE_CLIENT_SECRET").unwrap()
    }

    fn default_token_dir() -> PathBuf {
        app_root(AppDataType::UserCache, &APP_INFO).unwrap()
    }

    fn default_token_filename() -> String {
        "google-token.json".into()
    }

    fn default_property() -> String {
        "EVENT_ID".into()
    }

    fn default_redirect_uri() -> String {
        "http://localhost".into()
    }

    fn default_append_headline() -> String {
        "Sync".into()
    }
}
