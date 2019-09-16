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

#[derive(Serialize, Deserialize)]
#[serde(default)]
pub struct Conf {
    #[cfg(feature = "dotenv")]
    pub env_path: PathBuf,
    pub up_days: i64,
    pub down_days: i64,
    pub files: Vec<FileConf>,
    #[cfg(feature = "google_calendar")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub google_calendar: Option<GoogleCalendarGlobalConf>,
}

impl Default for Conf {
    fn default() -> Self {
        Conf {
            #[cfg(feature = "dotenv")]
            env_path: default_env_path(),
            up_days: 7,
            down_days: 7,
            files: Vec::new(),
            google_calendar: None,
        }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(default)]
pub struct EnvConf {
    pub env_path: PathBuf,
}

impl Default for EnvConf {
    fn default() -> Self {
        EnvConf {
            env_path: default_env_path(),
        }
    }
}

impl Conf {
    pub fn new(path: Option<PathBuf>) -> Result<Self> {
        let path = path.unwrap_or_else(default_config_path);

        let content = fs::read(&path).expect(&format!(
            "Failed to read fileConf: {}",
            path.as_path().display()
        ));

        if cfg!(feature = "dotenv") {
            let env_conf: EnvConf = toml::from_slice(&content)?;
            if env_conf.env_path.as_path().exists() {
                dotenv::from_path(env_conf.env_path.as_path())?;
            }
        }

        Ok(toml::from_slice(&content)?)
    }
}

#[derive(Serialize, Deserialize)]
pub struct FileConf {
    pub path: String,
    pub name: Option<String>,
    #[cfg(feature = "google_calendar")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub google_calendar: Option<GoogleCalendarConf>,
}

#[cfg(feature = "google_calendar")]
pub mod google_calendar {
    use super::*;

    #[derive(Serialize, Deserialize)]
    #[serde(default)]
    pub struct GoogleCalendarGlobalConf {
        pub client_id: String,
        pub client_secret: String,
        pub token_dir: PathBuf,
        pub token_filename: String,
        pub redirect_uri: String,
    }

    impl Default for GoogleCalendarGlobalConf {
        fn default() -> Self {
            GoogleCalendarGlobalConf {
                client_id: env::var("GOOGLE_CLIENT_ID").unwrap(),
                client_secret: env::var("GOOGLE_CLIENT_SECRET").unwrap(),
                token_dir: user_cache_path(),
                token_filename: "google-token.json".into(),
                redirect_uri: "http://localhost".into(),
            }
        }
    }

    #[derive(Serialize, Deserialize)]
    #[serde(default)]
    pub struct GoogleCalendarConf {
        pub calendar: String,
        pub append_new: bool,
        pub append_headline: String,
        pub property: String,
    }

    impl Default for GoogleCalendarConf {
        fn default() -> Self {
            GoogleCalendarConf {
                calendar: String::new(),
                append_new: false,
                append_headline: "Sync".into(),
                property: "EVENT_ID".into(),
            }
        }
    }
}
