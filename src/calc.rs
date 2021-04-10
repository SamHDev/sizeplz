use tokio::fs;
use std::io::{Result};
use crate::metadata::get_metadata_size;
use std::future::Future;
use std::pin::Pin;
use crate::util::{convert_os_string_option, convert_time};
use std::path::{PathBuf};

fn get_size(path: PathBuf) -> Pin<Box<dyn Future<Output = Result<u64>>>> {
    Box::pin(async move {
        let mut result: u64 = 0;

        let mut entries = fs::read_dir(path).await?;
        while let Some(entry) = entries.next_entry().await? {
            let entry_path = entry.path();

            if entry_path.is_file() {
                result += get_metadata_size(&fs::metadata(entry_path).await?);
            } else if entry_path.is_dir() {
                result += get_size(entry_path).await?;
            }
        }

        Ok(result)
    })
}

#[derive(Debug)]
pub struct Record {
    pub path: PathBuf,
    pub name: String,
    pub size: u64,
    pub modified: u64,
    pub created: u64,
    pub file: bool,
    pub children: Vec<Record>
}


pub(crate) fn handle_folder(path: PathBuf, depth: u16) -> Pin<Box<dyn Future<Output = Result<Record>>>> {
    Box::pin(async move {
        let mut result: u64 = 0;
        let mut children = Vec::new();

        let mut entries = fs::read_dir(&path).await?;
        while let Some(entry) = entries.next_entry().await? {
            let entry_path = entry.path();

            if entry_path.is_file() {
                let entry_meta = fs::metadata(&entry_path).await?;

                let file_size = get_metadata_size(&entry_meta);
                result += file_size;

                if depth != 0 {
                    children.push(Record {
                        path: entry_path.clone(),
                        name: convert_os_string_option(&entry_path.file_name(), "NAME"),
                        size: file_size,
                        modified: convert_time(entry_meta.modified()?),
                        created: convert_time(entry_meta.created()?),
                        file: true,
                        children: vec![]
                    })
                };

            } else if entry_path.is_dir() {
                result += get_size(entry_path.clone()).await?;

                if depth == 0 {
                    result += get_size(entry_path.to_path_buf()).await?;
                } else {
                    children.push(handle_folder(entry_path.to_path_buf(), depth - 1).await? );
                }
            }
        }

        let metadata = path.metadata()?;

        Ok(Record {
            name: convert_os_string_option(&path.file_name(), path.to_str().unwrap_or("?")),
            path,
            size: result,
            modified: convert_time(metadata.modified()?),
            created: convert_time(metadata.created()?),
            file: false,
            children
        })
    })
}
