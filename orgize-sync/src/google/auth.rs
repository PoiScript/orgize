use chrono::{DateTime, Duration, Utc};
use colored::Colorize;
use isahc::prelude::{Request, RequestExt, ResponseExt};
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{stdin, BufRead};
use std::path::PathBuf;
use url::Url;

use crate::conf::GoogleCalendarGlobalConf;
use crate::error::Result;

#[derive(Serialize, Deserialize)]
pub struct Auth {
    access_token: String,
    expires_at: DateTime<Utc>,
    refresh_token: String,
}

impl Auth {
    pub async fn new(conf: &GoogleCalendarGlobalConf) -> Result<Self> {
        let mut path = conf.token_dir.clone();
        path.push(&conf.token_filename);
        if let Ok(json) = fs::read_to_string(path) {
            Ok(serde_json::from_str(&json)?)
        } else {
            Auth::sign_in(conf).await
        }
    }

    pub fn save(&self, conf: &GoogleCalendarGlobalConf) -> Result<()> {
        let mut path = conf.token_dir.clone();
        path.push(&conf.token_filename);
        fs::write(path, serde_json::to_string(&self)?)?;
        Ok(())
    }

    async fn sign_in(config: &GoogleCalendarGlobalConf) -> Result<Self> {
        let url = Url::parse_with_params(
            "https://accounts.google.com/o/oauth2/v2/auth",
            &[
                ("client_id", &*config.client_id),
                ("response_type", "code"),
                ("access_type", "offline"),
                ("redirect_uri", &*config.redirect_uri),
                ("scope", "https://www.googleapis.com/auth/calendar"),
            ],
        )?;

        println!("Visit: {}", url.as_str().underline());
        println!("Follow the instructions and paste the code here:");

        for line in stdin().lock().lines() {
            let line = line?;
            let code = line.trim();

            if code.is_empty() {
                continue;
            } else if code == "q" {
                panic!()
            }

            let mut response = Request::post("https://www.googleapis.com/oauth2/v4/token")
                .header("content-type", "application/x-www-form-urlencoded")
                .body(format!(
                    "code={}&client_id={}&client_secret={}&grant_type={}&redirect_uri={}",
                    &code,
                    &config.client_id,
                    &config.client_secret,
                    "authorization_code",
                    "http://localhost"
                ))?
                .send_async()
                .await?;

            if response.status().is_success() {
                #[derive(Deserialize)]
                struct ConfirmCodeResponse {
                    access_token: String,
                    expires_in: i64,
                    refresh_token: String,
                }

                let json = response.json::<ConfirmCodeResponse>()?;

                println!("Logging in successfully.");

                let auth = Auth {
                    access_token: json.access_token,
                    expires_at: Utc::now() + Duration::seconds(json.expires_in),
                    refresh_token: json.refresh_token,
                };

                auth.save(config)?;

                return Ok(auth);
            } else {
                panic!("Failed to authorize.");
            }
        }

        panic!("Failed to authorize.");
    }

    pub async fn refresh(&mut self, config: &GoogleCalendarGlobalConf) -> Result<()> {
        let mut response = Request::post("https://www.googleapis.com/oauth2/v4/token")
            .header("content-type", "application/x-www-form-urlencoded")
            .body(format!(
                "client_id={}&client_secret={}&refresh_token={}&grant_type={}",
                &config.client_id, &config.client_secret, self.refresh_token, "refresh_token",
            ))?
            .send_async()
            .await?;

        if response.status().is_success() {
            #[derive(Deserialize)]
            struct RefreshTokenResponse {
                access_token: String,
                expires_in: i64,
            }

            let json = response.json::<RefreshTokenResponse>()?;

            self.access_token = json.access_token;
            self.expires_at = Utc::now() + Duration::seconds(json.expires_in);
            self.save(config)?;
        } else {
            panic!("");
        }

        Ok(())
    }

    pub fn is_valid(&self) -> bool {
        self.expires_at > Utc::now()
    }
}
