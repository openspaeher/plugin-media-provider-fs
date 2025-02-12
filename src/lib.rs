wit_bindgen::generate!({
    path: "wit",
    world: "media-provider-plugin"
});

struct FilesystemMediaProvider;

impl Guest for FilesystemMediaProvider {
    fn add_root_folder(path: _rt::String) -> Result<RootFolder, Error> {
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
