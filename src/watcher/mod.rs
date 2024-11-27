pub mod function;
pub mod node_api;

use crate::prelude::*;

/// Internal
#[derive(Debug)]
pub struct ThreadHandler {
  pub handle: Option<thread::JoinHandle<()>>,
  pub tx: mpsc::Sender<String>,
}

impl ThreadHandler {
  pub fn terminate(&mut self) -> Result<bool, Error> {
    let _ = self.tx.send("quit".to_string());
    if let Some(th) = self.handle.take() {
      return match th.join() {
        Ok(()) => Ok(true),
        Err(_) => Ok(false),
      };
    }
    Err(Error::from_reason(
      "No thread to terminate for this handler",
    ))
  }
}

#[derive(Clone, Copy, Debug)]
pub struct FileInfo {
  size: u64,
  last_update: time::SystemTime,
  has_change: bool,
  type_change: ChangedType,
}

#[derive(Debug)]
pub struct ChangeInfo {
  path: String,
  reason: ChangedType,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum ChangedType {
  Created,
  Changed,
  Deleted,
}

impl fmt::Display for ChangedType {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Self::Created => write!(f, "Created"),
      Self::Changed => write!(f, "Changed"),
      Self::Deleted => write!(f, "Deleted"),
    }
  }
}
