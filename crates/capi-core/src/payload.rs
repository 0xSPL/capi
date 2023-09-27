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

  /// Create a new `Payload` from a single key-value pair.
  #[inline]
  pub fn from_kv<K, V>(k: K, v: V) -> Self
  where
    K: Into<String>,
    V: Into<Value>,
  {
    let mut this: Self = Self::new();
    this.inner.insert(k.into(), v.into());
    this
  }

  /// Add a new key-value pair to the payload.
  #[inline]
  pub fn insert<K, V>(&mut self, k: K, v: V)
  where
    K: Into<String>,
    V: Into<Value>,
  {
    self.inner.insert(k.into(), v.into());
  }
}

impl Default for Payload {
  #[inline]
  fn default() -> Self {
    Self::new()
  }
}
