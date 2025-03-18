wit_bindgen::generate!({
    path: "wit/media-provider.wit",
    world: "media-provider-plugin-world"
});

struct FilesystemMediaProvider;

impl Guest for FilesystemMediaProvider {
    fn add_root_folder(path: _rt::String) -> Result<RootFolder, Error> {
        spaeher::media_provider_plugin::logging::log_info("Log test");
        spaeher::media_provider_plugin::logging::log_err("Err test");
        spaeher::media_provider_plugin::emit_events::emit_job_progress_event(
            &spaeher::media_provider_plugin::emit_events::JobProgress {
                id: "some-id".to_string(),
                total_children: None,
                finished_children: None,
                value: 50.0,
            },
        );
        return Result::Err(Error::NotImplemented);
    }

    fn remove_root_folder(root_folder_id: _rt::String) -> Result<(), Error> {
        return Result::Err(Error::NotImplemented);
    }

    fn index_path(
        root_folder_id: _rt::String,
        path: _rt::String,
        index_id: _rt::String,
    ) -> Result<u32, Error> {
        return Result::Err(Error::NotImplemented);
    }

    fn find_file(root_index_id: _rt::String, path: _rt::String) -> Result<File, Error> {
        return Result::Err(Error::NotImplemented);
    }

    fn get_file(root_folder_id: _rt::String, file_id: _rt::String) -> Result<File, Error> {
        return Result::Err(Error::NotImplemented);
    }

    fn find_folder(root_folder_id: _rt::String, path: _rt::String) -> Result<Folder, Error> {
        return Result::Err(Error::NotImplemented);
    }

    fn get_folder(root_folder_id: _rt::String, folder_id: _rt::String) -> Result<Folder, Error> {
        return Result::Err(Error::NotImplemented);
    }

    fn find_files(
        root_folder_id: _rt::String,
        path: _rt::String,
        page: u32,
        page_size: u32,
    ) -> Result<_rt::Vec<File>, Error> {
        return Result::Err(Error::NotImplemented);
    }

    fn get_files(
        root_folder_id: _rt::String,
        folder_id: _rt::String,
        page: u32,
        page_size: u32,
    ) -> Result<_rt::Vec<File>, Error> {
        return Result::Err(Error::NotImplemented);
    }

    fn find_folders(
        root_folder_id: _rt::String,
        path: _rt::String,
        page: u32,
        page_size: u32,
    ) -> Result<_rt::Vec<Folder>, Error> {
        return Result::Err(Error::NotImplemented);
    }

    fn get_folders(
        root_folder_id: _rt::String,
        folder_id: _rt::String,
        page: u32,
        page_size: u32,
    ) -> Result<_rt::Vec<Folder>, Error> {
        return Result::Err(Error::NotImplemented);
    }
}

export!(FilesystemMediaProvider);
