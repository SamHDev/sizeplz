use std::fs::Metadata;

#[allow(unreachable_code)]
pub fn get_metadata_size(meta: &Metadata) -> u64 {
    #[cfg(windows)]
        return std::os::windows::fs::MetadataExt::file_size(meta);

    #[cfg(linux)]
        return std::os::linux::fs::MetadataExt::st_size(meta);

    #[cfg(linux)]
        return std::os::unix::fs::MetadataExt::size(meta);

    panic!("Failed to read file size from metadata. Os not supported.")
}