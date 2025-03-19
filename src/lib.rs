use std::{
    path::PathBuf,
    str::FromStr,
    sync::{Arc, RwLock},
};

use serde::{Deserialize, Serialize};
use spaeher::media_provider_plugin::logging::log_warn;

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

impl FilesystemMediaProvider {
    fn index_path(base_path: String, path: String, action_id: String) -> Result<u32, Error> {
        let config = CONFIG
            .read()
            .map_err(|_| Error::ConfigInvalid("Failed to acquire read lock".to_string()))?
            .clone()
            .ok_or(Error::ConfigInvalid("Config not initialized".to_string()))?;
        let path = PathBuf::from_str(&path).map_err(|e| Error::PathInvalid(e.to_string()))?;
        let mut total_indexed_files = 0u32;
        for entry in
            std::fs::read_dir(path.clone()).map_err(|e| Error::PathInvalid(e.to_string()))?
        {
            let Ok(entry) = entry else {
                continue;
            };
            let entry_path = entry.path();
            if entry_path.is_dir() {
                let Some(entry_path_str) = entry_path.to_str() else {
                    log_warn(&format!(
                        "Failed to convert path to string: {:?}",
                        entry_path
                    ));
                    continue;
                };
                match Self::index_path(
                    base_path.clone(),
                    entry_path_str.to_owned(),
                    action_id.clone(),
                ) {
                    Ok(indexed_files) => {
                        total_indexed_files += indexed_files;
                    }
                    Err(err) => {
                        log_warn(&format!(
                            "Failed to index subdirectory: '{}': {}",
                            entry_path_str, err
                        ));
                        continue;
                    }
                }
            } else {
                let file_name = entry.file_name();
                let Some(name_str) = file_name.to_str() else {
                    log_warn(&format!(
                        "Failed to convert file name to string: {:?}",
                        file_name
                    ));
                    continue;
                };
                for ext in &config.file_extensions {
                    if name_str.ends_with(ext) {
                        let path = path.clone();
                        let Ok(Some(relative_path)) =
                            path.strip_prefix(base_path.clone()).map(|p| p.to_str())
                        else {
                            log_warn(&format!(
                                "Failed to compute relative path for path: {:?}",
                                path
                            ));
                            continue;
                        };
                        emit_indexed_file_event(
                            &File {
                                name: name_str.to_string(),
                                relative_path: relative_path.to_string(),
                                duration: None,
                            },
                            &action_id,
                        );
                        break;
                    }
                }
            }
        }
        return Result::Ok(total_indexed_files);
    }
}

impl Guest for FilesystemMediaProvider {
    fn index_path(path: String, action_id: String) -> Result<u32, Error> {
        Self::index_path(path.clone(), path, action_id)
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
