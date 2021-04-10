use tokio::fs;
use std::io::{Result, ErrorKind};
use crate::metadata::get_metadata_size;
use crate::util::{convert_os_string_option, convert_time};
use std::path::{PathBuf, Path};
use futures::future::{BoxFuture, FutureExt};

async fn get_file_size(path: &Path) -> Result<u64> {
    match catch_permission(path, fs::metadata(&path).await)? {
        Some(meta) => Ok(get_metadata_size(&meta)),
        None => Ok(0)
    }
}

fn get_size(path: PathBuf) -> BoxFuture<'static, Result<u64>> {
    async move {
        let mut result: u64 = 0;

        let mut entries = if let Some(e) = catch_permission(&path, fs::read_dir(&path).await)? { e } else { return Ok(0) };
        while let Ok(Some(entry)) = entries.next_entry().await {
            let entry_path = entry.path();

            if entry_path.is_file() {
                result += get_file_size(&entry_path).await?;
            } else if entry_path.is_dir() {
                result += get_size(entry_path).await?;
            }
        }

        Ok(result)
    }.boxed()
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

pub(crate) fn handle_folder(path: PathBuf, depth: u16) -> BoxFuture<'static, Result<Record>> {
    async move {
        let mut result: u64 = 0;
        let mut children = Vec::new();

        let mut children_tasks = Vec::new();

        let mut entries = if let Some(e) = catch_permission(&path, fs::read_dir(&path).await)? { e } else {
            return Ok(Record {
                name: convert_os_string_option(&path.file_name(), path.to_str().unwrap_or("?")),
                path,
                size: result,
                modified: 0,
                created: 0,
                file: false,
                children
            })
        };

        while let Ok(Some(entry)) = entries.next_entry().await {
            let entry_path = entry.path();

            if entry_path.is_file() {
                let entry_meta = if let Some(x) = catch_permission(&entry_path, fs::metadata(&entry_path).await)? { x }
                else { if depth != 0 {
                    children.push(Record {
                        path: entry_path.clone(),
                        name: convert_os_string_option(&entry_path.file_name(), "NAME"),
                        size: 0,
                        modified: 0,
                        created: 0,
                        file: true,
                        children: vec![]
                    })
                }; continue; };

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
                if depth != 0 {
                    children_tasks.push(tokio::task::spawn(handle_folder(entry_path.to_path_buf(), depth - 1) ));
                } else {
                    result += get_size(entry_path.to_path_buf()).await?;
                }
            }
        }

        for task in children_tasks {
            let child = task.await.unwrap()?;
            result += child.size;
            children.push(child);
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
    }.boxed()
}

pub fn catch_permission<T>(path: &Path, x: Result<T>) -> Result<Option<T>> {
    match x {
        Ok(x) => Ok(Some(x)),
        Err(e) => match e.kind() {
            ErrorKind::PermissionDenied => {println!("Permission Denied: {:?}", path); Ok(None)},
            _ => Err(e)
        }
    }
}