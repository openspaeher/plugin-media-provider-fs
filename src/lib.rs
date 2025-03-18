wit_bindgen::generate!({
    path: "wit/media-provider.wit",
    world: "media-provider-plugin-world"
});

struct FilesystemMediaProvider;

impl Guest for FilesystemMediaProvider {
    fn index_path(
        path: _rt::String,
        job_id: _rt::String,
    ) -> Result<u32, Error> {
        emit_indexed_folder_event(&Folder {name: "".to_string(), relative_path: "".to_string()});
        emit_indexed_file_event(&File { name: "".to_string(), relative_path: "".to_string(), duration: None });
        return Result::Err(Error::NotImplemented);
    }
}

export!(FilesystemMediaProvider);
