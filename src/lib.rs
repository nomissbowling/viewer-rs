#![doc(html_root_url = "https://docs.rs/viewer/0.0.4")]
//! viewer
//!
//! Cargo test with [-- --nocapture] or with [-- --show-output]
//!
//! - cargo test --release -- --nocapture
//! - cargo test --release -- --show-output
//!

pub mod core;

/// test with [-- --nocapture] or [-- --show-output]
#[cfg(test)]
mod tests {
  use std::{result::Result, error::Error};
  use super::core::*;

  /// test lib
  #[test]
  fn test_lib() -> Result<(), Box<dyn Error>> {
    assert_eq!(lib_test()?, ());
    Ok(())
  }
}
