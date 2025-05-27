use std::{
    io::{BufRead, BufReader},
    process::{self, Child, Command},
    thread,
};

use anyhow::{Context, Ok};
use serde::Deserialize;
use tracing::debug;
use url::Url;

use crate::{
    config::ServerConfig,
    constants::{SERVER_DOWNLOAD_ENDPOINT, SERVER_UPDATER_ENDPOINT},
};

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct ServerUpdaterResponse {
    latest_version: String,
}

pub struct Server {
    config: ServerConfig,
    process: Option<Child>,
}

impl Server {
    pub fn new(config: ServerConfig) -> Self {
        Self {
            config,
            process: None,
        }
    }

    pub fn setup(&self) -> anyhow::Result<()> {
        let latest_version = reqwest::blocking::get(SERVER_UPDATER_ENDPOINT)?
            .json::<ServerUpdaterResponse>()?
            .latest_version;

        let should_download = self.config.version() != Some(latest_version.clone());

        if should_download {
            let download_url = Url::parse(
                SERVER_DOWNLOAD_ENDPOINT
                    .replace("VERSION", &latest_version)
                    .as_str(),
            )?;

            let latest_file = reqwest::blocking::get(download_url)?
                .bytes()
                .context("Failed to fetch server file")?;

            self.config
                .update_file(latest_file)
                .context("Failed to write server file")?;

            self.config
                .update_version(latest_version)
                .context("Failed to write version file")?;
        }

        Ok(())
    }

    pub fn start(&mut self, dev: bool) -> anyhow::Result<()> {
        let mut child = Command::new("node")
            .env("NO_CORS", (dev as i32).to_string())
            .arg(self.config.file.as_os_str())
            .stdout(process::Stdio::piped())
            .spawn()
            .context("Failed to start server")?;

        if let Some(stdout) = child.stdout.take() {
            let reader = BufReader::new(stdout);
            let mut lines = reader.lines();

            thread::spawn(move || {
                while let Some(Result::Ok(line)) = lines.next() {
                    debug!(target: "server", "{}", line);
                }
            });
        }

        self.process = Some(child);

        Ok(())
    }

    pub fn stop(&mut self) -> anyhow::Result<()> {
        if let Some(mut process) = self.process.take() {
            process.kill().context("Failed to kill server process")?;
        }

        Ok(())
    }
}

impl Drop for Server {
    fn drop(&mut self) {
        self.stop().expect("Failed to stop server");
    }
}
