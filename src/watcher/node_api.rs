use crate::prelude::*;

use super::{function::*, FileInfo, ThreadHandler};

#[napi(object)]
pub struct DirInfo {
  pub path: String,
  pub recursive: bool,
}

/// Add a watcher to a directory it's recursive by default
///
/// If you don't want it to be recursive, use { path: 'dir_path', recurse: false }
///
/// Every time a file is modified or added or deleted the callback is called with the message : "[dir_path] [change_type]"
///
/// enum change_type { Created, Changed, Deleted }
///
/// @returns a handler to terminate the watcher if the watcher has been started else false with an error message in the callback
///
/// ```js
/// const { setWatcher } = require('node_toolbox');
/// function listener(err, message) {
///   if (err) {
///     // log the error and exit the listener
///     console.error(err);
///     return;
///   }
///
///   console.log(data);
///   // if a new file named index.ts is created, the data will be 'index.ts Created'
///   // if a file named index.ts is modified, the data will be 'index.ts Changed'
///   // if a file named index.ts is deleted, the data will be 'index.ts Deleted'
/// }
/// const handler = setWatcher('path', listener);
/// if(!handler) {
///   // things to do if no watcher
/// }
/// ```
#[napi(
  ts_args_type = "config: string | DirInfo, callback: (err: null | Error, result: string) => void"
)]
pub fn set_watcher(
  config: Either<String, DirInfo>,
  listener: ThreadsafeFunction<String, ErrorStrategy::CalleeHandled>,
) -> Result<Either<External<ThreadHandler>, bool>, Error> {
  let (dir, recursive) = match config {
    Either::A(dir_path) => (dir_path, true),
    Either::B(config) => (config.path, config.recursive),
  };
  if dir.is_empty() {
    listener.call(
      Err(Error::from_reason("No path specified".to_string())),
      ThreadsafeFunctionCallMode::Blocking,
    );
    return Ok(Either::B(false));
  }

  let ret: bool = match fs::exists(&dir) {
    Ok(result) => result,
    Err(err) => {
      listener.call(
        Err(Error::from_reason(err.to_string())),
        ThreadsafeFunctionCallMode::Blocking,
      );
      return Ok(Either::B(false));
    }
  };

  if ret {
    let mut files: HashMap<String, FileInfo> = HashMap::new();
    // build files index
    match dir_crawling(&dir, &mut files, recursive) {
      Ok(_) => (),
      Err(err) => return Err(err),
    }
    // Reset the first round
    let _ = should_trigger(&mut files);
    let (tx, rx) = mpsc::channel::<String>();
    let instance = thread::spawn(move || loop {
      let root = dir.clone();
      match dir_crawling(&root, &mut files, recursive) {
        Ok(_) => (),
        Err(err) => {
          listener.call(Err(err), ThreadsafeFunctionCallMode::Blocking);
          break;
        }
      }
      match watch_file(&mut files) {
        Ok(_) => (),
        Err(err) => {
          listener.call(Err(err), ThreadsafeFunctionCallMode::Blocking);
          break;
        }
      }
      if let Some(file_changed) = should_trigger(&mut files) {
        listener.call(
          Ok(format!("{} {}", &file_changed.path, file_changed.reason)),
          ThreadsafeFunctionCallMode::NonBlocking,
        );
      }
      if files.is_empty() {
        break;
      }
      if let Ok(message) = rx.recv_timeout(time::Duration::from_millis(100)) {
        if message == "quit" {
          break;
        }
      }
    });
    return Ok(Either::A(External::new(ThreadHandler {
      handle: Some(instance),
      tx,
    })));
  } else {
    listener.call(
      Err(Error::from_reason("Directory doesn't exist")),
      ThreadsafeFunctionCallMode::Blocking,
    );
  }
  Ok(Either::B(false))
}

/// Clear a previously instantiate watcher
/// ```js
/// const handler = setWatcher('path', () => {});
/// if(!handler) {
///   // things to do if no watcher
/// }
///
/// // wait some time and cleanup the watcher
/// clearWatcher(handler);
/// ```
#[napi]
pub fn clear_watcher(mut handle: External<ThreadHandler>) -> Result<bool, Error> {
  handle.terminate()
}
