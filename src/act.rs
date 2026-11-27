//! act
//!

use serde_json;
use serde::{Serialize, Deserialize};

/// Act
#[derive(Serialize, Deserialize, Debug)]
pub struct Act {
  /// act
  #[serde(alias = "id")]
  pub act: String,
  /// lst
  #[serde(alias = "list")]
  pub lst: Vec<String>
}

/// impl fmt::Display for Act
impl std::fmt::Display for Act {
  /// fmt
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", serde_json::to_string(self).map_or(format!("{{}}"), |s| s))
  }
}
