use std::sync::{Arc, RwLock};

use serde::{Deserialize, Serialize};

wit_bindgen::generate!({
    path: "wit/contract.wit",
    world: "media-provider-plugin-world"
});

#[derive(Serialize, Deserialize)]
struct Config {
    file_extensions: Vec<String>,
}

lazy_static::lazy_static! {
    static ref CONFIG: Arc<RwLock<Option<Config>>> = Arc::new(RwLock::new(None));
}

struct FilesystemMediaProvider;

impl Guest for FilesystemMediaProvider {
    fn index_path(path: _rt::String, job_id: _rt::String) -> Result<u32, Error> {
        emit_indexed_folder_event(&Folder {
            name: "".to_string(),
            relative_path: "".to_string(),
        });
        emit_indexed_file_event(&File {
            name: "".to_string(),
            relative_path: "".to_string(),
            duration: None,
        });
        return Result::Err(Error::NotImplemented);
    }

    fn on_config_changed(action_id:_rt::String,) -> Result<(),Error> {
        return Result::Err(Error::NotImplemented);
    }
}

export!(FilesystemMediaProvider);
