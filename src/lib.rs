use std::{
    path::{Path, PathBuf},
    str::FromStr,
    sync::{Arc, RwLock},
};

use ignore::WalkBuilder;
use serde::{Deserialize, Serialize};
use spaeher::media_provider_plugin::{
    logging::log_warn,
    media_provider::{emit_indexed_file_event, File},
};

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

        WalkBuilder::new(base_path_str)
            .hidden(false)
            .parents(false)
            .ignore(false)
            .git_ignore(false)
            .git_global(false)
            .git_exclude(false)
            .follow_links(false)
            .build()
            .filter_map(Result::ok)
            .filter(|entry| {
                entry.path().is_file()
                    && entry
                        .path()
                        .extension()
                        .map(|ext| {
                            config
                                .file_extensions
                                .contains(&ext.to_string_lossy().to_string())
                        })
                        .unwrap_or(false)
            })
            .for_each(|entry| {
                match Path::strip_prefix(&entry.clone().into_path(), base_path.clone()) {
                    Ok(relative_path) => {
                        emit_indexed_file_event(
                            &File {
                                name: entry.file_name().to_string_lossy().to_string(),
                                relative_path: relative_path.to_string_lossy().to_string(),
                                duration: None,
                            },
                            &action_id,
                        );
                        total_indexed_files += 1;
                    }
                    Err(_) => {
                        log_warn(&format!(
                            "Failed to compute relative path for path: {:?}",
                            entry
                        ));
                        return;
                    }
                }
            });
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

    fn get_streamable_url(path: String, _action_id: String) -> Result<StreamableUrl, Error> {
        let config = CONFIG
            .read()
            .map_err(|_| Error::ConfigInvalid("Failed to acquire read lock".to_string()))?
            .clone()
            .ok_or(Error::ConfigInvalid("Config not initialized".to_string()))?;

        let path = Path::new(&path);
        let path_str = path.to_string_lossy().to_string();
        if path.exists()
            && path
                .extension()
                .map(|ext| {
                    config
                        .file_extensions
                        .contains(&ext.to_string_lossy().to_string())
                })
                .unwrap_or(false)
        {
            Ok(StreamableUrl {
                url: path_str,
                is_local: true
            })
        } else {
            Err(Error::PathInvalid(format!(
                "Path '{}' is not valid, or the file is not a supported file type.",
                path_str
            )))
        }
    }
}

export!(FilesystemMediaProvider);
