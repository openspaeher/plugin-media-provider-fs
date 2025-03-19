use std::{
    path::{Path, PathBuf},
    str::FromStr,
    sync::{Arc, RwLock},
};

use glob::glob;
use serde::{Deserialize, Serialize};
use spaeher::media_provider_plugin::{logging::log_warn, media_provider::{emit_indexed_file_event, File}};

wit_bindgen::generate!({
    path: "wit/contract.wit",
    world: "media-provider-plugin-world"
});

const CONFIG_PATH: &str = "./resources/config.toml";

#[derive(Serialize, Deserialize, Clone)]
struct Config {
    file_extensions: Vec<String>,
}

lazy_static::lazy_static! {
    static ref CONFIG: Arc<RwLock<Option<Config>>> = Arc::new(RwLock::new(None));
}

struct FilesystemMediaProvider;

impl Guest for FilesystemMediaProvider {
    fn index_path(base_path_str: String, action_id: String) -> Result<u32, Error> {
        let config = CONFIG
            .read()
            .map_err(|_| Error::ConfigInvalid("Failed to acquire read lock".to_string()))?
            .clone()
            .ok_or(Error::ConfigInvalid("Config not initialized".to_string()))?;
        let base_path =
            PathBuf::from_str(&base_path_str).map_err(|e| Error::PathInvalid(e.to_string()))?;
        let mut total_indexed_files = 0u32;
        let glob_pattern = format!(
            "{}/**/*.{{{}}}",
            &base_path_str,
            config
                .file_extensions
                .iter()
                .map(|e| format!(".{}", e))
                .collect::<Vec<String>>()
                .join(",")
        );
        for entry in glob(&glob_pattern).map_err(|e| Error::PathInvalid(e.to_string()))? {
            let Ok(entry) = entry else {
                continue;
            };
            if entry.is_dir() {
                continue;
            }
            let Some(Some(file_name)) = entry.file_name().map(|f| f.to_str()) else {
                log_warn(&format!(
                    "Failed to convert file name to string: {:?}",
                    entry.file_name()
                ));
                continue;
            };
            let Ok(relative_path) = Path::strip_prefix(&entry, base_path.clone()) else {
                log_warn(&format!(
                    "Failed to compute relative path for path: {:?}",
                    entry
                ));
                continue;
            };
            let Some(relative_path_str) = relative_path.to_str() else {
                log_warn(&format!(
                    "Failed to convert relative path to string: {:?}",
                    relative_path
                ));
                continue;
            };
            emit_indexed_file_event(
                &File {
                    name: file_name.to_string(),
                    relative_path: relative_path_str.to_string(),
                    duration: None,
                },
                &action_id,
            );
            total_indexed_files += 1;
        }
        return Result::Ok(total_indexed_files);
    }

    fn on_config_changed(_action_id: String) -> Result<(), Error> {
        let config = toml::from_str::<Config>(
            &std::fs::read_to_string(
                PathBuf::from_str(CONFIG_PATH).map_err(|e| Error::ConfigInvalid(e.to_string()))?,
            )
            .map_err(|e| Error::ConfigInvalid(e.to_string()))?,
        )
        .map_err(|e| Error::ConfigInvalid(e.to_string()))?;
        CONFIG
            .write()
            .map_err(|_| Error::ConfigInvalid("Failed to acquire write lock".to_string()))?
            .replace(config);
        return Ok(());
    }
}

export!(FilesystemMediaProvider);
