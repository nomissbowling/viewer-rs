//! viewer
//!
//! Cargo test with [-- --nocapture] or with [-- --show-output]
//!
//! - cargo test --bin viewer --release -- --nocapture
//! - cargo test --bin viewer --release -- --show-output
//! - cargo run --bin viewer --release
//! - cargo build --bin viewer --release
//!

use wktk::core::*;
// use rstk::*;
use viewer::core::*;

use std::{result::Result, error::Error};

/// main
pub fn main() -> Result<(), Box<dyn Error>> {
  Ok(())
}

/// viewer test
pub fn viewer_test(rt: &rstk::TkTopLevel) -> Result<(), Box<dyn Error>> {
  assert_eq!(rstk_test(rt, "test viewer test")?, ()); // same as lib
  assert_eq!(uploader_test(&rt)?, ());
  Ok(())
}

/// test with [-- --nocapture] or [-- --show-output]
#[cfg(test)]
mod tests {
  use std::{result::Result, error::Error};
  use wktk::{core::*, wk};
  use rstk;
  use viewer::core::*; // lib
  use crate::*; // main

  /// test viewer
  #[test]
  fn test_viewer() -> Result<(), Box<dyn Error>> {
    let rt = wish_toplevel(640, 480)?;
    assert_eq!(rstk_test(&rt, "test viewer")?, ()); // same as lib
    assert_eq!(uploader_test(&rt)?, ());
    assert_eq!(viewer_test(&rt)?, ());
    wk::topmost(&rt)?;
    rstk::mainloop();
    Ok(())
  }
}
