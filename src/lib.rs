#![deny(clippy::all)]

pub mod watcher;

pub mod prelude {
  pub use napi::{
    bindgen_prelude::External,
    threadsafe_function::{ErrorStrategy, ThreadsafeFunction, ThreadsafeFunctionCallMode},
    Either, Error,
  };

  pub use std::{collections::HashMap, fmt, fs, path::Path, sync::mpsc, thread, time};
}

#[macro_use]
extern crate napi_derive;
