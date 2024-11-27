use crate::prelude::*;

use super::{ChangeInfo, ChangedType, FileInfo};

pub fn dir_crawling(
  dir: &String,
  files: &mut HashMap<String, FileInfo>,
  recurse: bool,
) -> Result<bool, Error> {
  let path = Path::new(&dir);
  if path.is_dir() {
    if let Ok(dir_entry) = fs::read_dir(path) {
      for entries in dir_entry {
        match entries {
          Ok(entry) => {
            let path = entry.path();
            let path_string = path.clone().into_os_string().into_string().unwrap();
            if path.is_dir() && recurse {
              let _ = dir_crawling(&path_string, files, recurse);
            } else if !files.contains_key(&path_string) && !path.is_dir() {
              if let Ok(metadata) = fs::metadata(&path) {
                files.insert(
                  path.clone().into_os_string().into_string().unwrap(),
                  FileInfo {
                    size: metadata.len(),
                    last_update: metadata.modified().unwrap_or(time::SystemTime::now()),
                    has_change: true,
                    type_change: ChangedType::Created,
                  },
                );
              }
            }
          }
          Err(err) => {
            return Err(err.into());
          }
        }
      }
    }
  } else if path.is_file() && !files.contains_key(&path.to_str().unwrap().to_string()) {
    if let Ok(metadata) = fs::metadata(path) {
      files.insert(
        path.to_str().unwrap().to_string(),
        FileInfo {
          size: metadata.len(),
          last_update: metadata.modified().unwrap_or(time::SystemTime::now()),
          has_change: true,
          type_change: ChangedType::Created,
        },
      );
    }
  }
  Ok(true)
}

pub fn watch_file(files: &mut HashMap<String, FileInfo>) -> Result<bool, Error> {
  for (path, info) in files {
    if let Ok(metadata) = fs::metadata(path) {
      if info.last_update != metadata.modified().unwrap_or(time::SystemTime::now()) {
        info.last_update = metadata.modified().unwrap_or(time::SystemTime::now());
        info.has_change = true;
        info.type_change = ChangedType::Changed;
      }
      if info.size != metadata.len() {
        info.size = metadata.len();
        info.has_change = true;
        info.type_change = ChangedType::Changed;
      }
    } else {
      info.has_change = true;
      info.type_change = ChangedType::Deleted;
    }
  }
  Ok(true)
}

pub fn should_trigger(files: &mut HashMap<String, FileInfo>) -> Option<ChangeInfo> {
  let mut change: Option<ChangeInfo> = None;
  let mut to_remove: Vec<String> = vec![];
  for (path, file_info) in &mut *files {
    if file_info.has_change {
      change = Some(ChangeInfo {
        path: path.to_string(),
        reason: file_info.type_change,
      });
      file_info.has_change = false;
      if file_info.type_change == ChangedType::Deleted {
        to_remove.push(path.clone());
      }
    }
  }
  for p in to_remove {
    files.remove_entry(&p);
  }
  change
}
