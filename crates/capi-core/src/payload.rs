use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;
use std::collections::BTreeMap;

// =============================================================================
// Payload
// =============================================================================

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
#[serde(transparent)]
pub struct Payload {
  inner: BTreeMap<String, Value>,
}

impl Payload {
  /// Create a new `Payload`.
  #[inline]
  pub const fn new() -> Self {
    Self {
      inner: BTreeMap::new(),
    }
  }
}

impl Default for Payload {
  #[inline]
  fn default() -> Self {
    Self::new()
  }
}
