use crate::{Error, Reportable};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub leptos: Leptos,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Leptos {
    pub app_path: Option<String>,
    pub client_path: Option<String>,
    pub server_path: Option<String>,
}

impl Config {
    pub fn read(path: &str) -> Result<Self, Error> {
        log::debug!("Reading config file {path}");
        let toml = fs::read_to_string(path)?;
        log::trace!("Config file content:\n{toml}");
        Ok(toml::from_str(&toml)?)
    }

    pub fn projects(&self) -> Projects {
        Projects {
            app: param_or_folder(&self.leptos.app_path, "./app"),
            client: param_or_folder(&self.leptos.client_path, "./client"),
            server: param_or_folder(&self.leptos.client_path, "./server"),
        }
    }

    pub fn save_default_file() -> Result<(), Reportable> {
        Self::try_save_default_to("leptos.toml").map_err(|e| e.file_context("", "leptos.toml"))
    }

    fn try_save_default_to(path: &str) -> Result<(), Error> {
        log::debug!("Adding default leptos.toml file");
        let toml = include_str!("leptos.toml");
        log::trace!("Content of leptos.toml:\n{toml}");
        Ok(std::fs::write(path, toml.as_bytes())?)
    }
}

#[derive(Debug, Default)]
pub struct Projects {
    pub app: Option<String>,
    pub client: Option<String>,
    pub server: Option<String>,
}

fn param_or_folder(param: &Option<String>, folder: &str) -> Option<String> {
    if let Some(path) = param {
        Some(path.to_string())
    } else {
        let path = PathBuf::from(folder);
        if path.exists() && path.is_dir() {
            Some(folder.to_string())
        } else {
            None
        }
    }
}