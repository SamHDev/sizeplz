use std::io::{ErrorKind, Result};
use std::path::{Path, PathBuf};

use futures::future::{BoxFuture, FutureExt};
use tokio::fs;

use crate::metadata::get_metadata_size;
use crate::util::{convert_os_string_option, convert_time};

// Get the size of a file from a path
// - catches permissions errors
async fn get_file_size(path: &Path) -> Result<u64> {
    // catch
    match catch_permission(path, fs::metadata(&path).await)? {
        Some(meta) => Ok(get_metadata_size(&meta)),
        None => Ok(0)
    }
}

// Get the size of a directory
fn get_size(path: PathBuf) -> BoxFuture<'static, Result<u64>> {
    // box future bs
    async move {
        // create result buffer/var
        let mut result: u64 = 0;

        // get dir entries (while catching for permissions)
        let mut entries = if let Some(e) = catch_permission(&path, fs::read_dir(&path).await)? { e } else { return Ok(0); };

        // loop over entries
        while let Ok(Some(entry)) = entries.next_entry().await {
            // get entry path
            let entry_path = entry.path();

            // add to result
            if entry_path.is_file() {
                // get file size
                result += get_file_size(&entry_path).await?;
            } else if entry_path.is_dir() {
                // recursive
                result += get_size(entry_path).await?;
            }
        }

        // return size
        Ok(result)
    }.boxed()
}

// Record object.
// Used for storing calc results to formatter.
#[derive(Debug)]
pub struct Record {
    // path of the record
    pub path: PathBuf,
    // name of the record, used for output
    pub name: String,
    // size of the record.
    pub size: u64,
    // modified epoch, used in sort
    pub modified: u64,
    // created epoch, used in sort
    pub created: u64,
    // if the record is file or record
    pub file: bool,
    // children of record (if not file)
    pub children: Vec<Record>,
}

// Read a folder returning results until depth limit.
pub(crate) fn handle_folder(path: PathBuf, depth: u16) -> BoxFuture<'static, Result<Record>> {
    // async bs
    async move {
        // temp vars
        let mut result: u64 = 0;
        let mut children = Vec::new();
        let mut children_tasks = Vec::new();

        // get dir entries (while catching permission errors)
        let mut entries = if let Some(e) = catch_permission(&path, fs::read_dir(&path).await)? { e } else {
            // return empty record.
            return Ok(Record {
                name: convert_os_string_option(&path.file_name(), path.to_str().unwrap_or("?")),
                path,
                size: result,
                modified: 0,
                created: 0,
                file: false,
                children,
            });
        };

        // iterate over entries
        while let Ok(Some(entry)) = entries.next_entry().await {
            // get path
            let entry_path = entry.path();

            // if file
            if entry_path.is_file() {
                // get file meta (while catching permissions)
                let entry_meta =
                    if let Some(x) = catch_permission(&entry_path, fs::metadata(&entry_path).await)? { x } else {
                        // return empty record.
                        if depth != 0 {
                            children.push(Record {
                                path: entry_path.clone(),
                                name: convert_os_string_option(&entry_path.file_name(), "NAME"),
                                size: 0,
                                modified: 0,
                                created: 0,
                                file: true,
                                children: vec![],
                            })
                        };
                        continue;
                    };

                // add file size to result.
                result += get_metadata_size(&entry_meta);

                if depth != 0 {
                    // create file record.
                    children.push(Record {
                        path: entry_path.clone(),
                        name: convert_os_string_option(&entry_path.file_name(), "NAME"),
                        size: file_size,
                        modified: convert_time(entry_meta.modified()?),
                        created: convert_time(entry_meta.created()?),
                        file: true,
                        children: vec![],
                    })
                };
            } else if entry_path.is_dir() {
                if depth != 0 {
                    // offload recursive task to tokio task.
                    children_tasks.push(tokio::task::spawn(
                        handle_folder(entry_path.to_path_buf(), depth - 1)
                    ));
                } else {
                    // add to size.
                    result += get_size(entry_path.to_path_buf()).await?;
                }
            }
        }

        // wait for task completion
        for task in children_tasks {
            let child = task.await.unwrap()?;
            result += child.size; // add child size to total.
            children.push(child);
        }

        // folder metadata.
        let metadata = path.metadata()?;

        // create record..
        Ok(Record {
            name: convert_os_string_option(&path.file_name(), path.to_str().unwrap_or("?")),
            path,
            size: result,
            modified: convert_time(metadata.modified()?),
            created: convert_time(metadata.created()?),
            file: false,
            children,
        })
    }.boxed()
}

// catch permission denied error.
pub fn catch_permission<T>(path: &Path, x: Result<T>) -> Result<Option<T>> {
    match x {
        Ok(x) => Ok(Some(x)),
        Err(e) => match e.kind() {
            ErrorKind::PermissionDenied => {
                println!("Permission Denied: {:?}", path);
                Ok(None)
            }
            _ => Err(e)
        }
    }
}