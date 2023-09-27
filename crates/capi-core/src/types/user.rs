use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

#[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
#[serde(transparent)]
pub struct UserID(u64);

impl UserID {
  #[inline]
  pub const fn new(value: u64) -> Self {
    Self(value)
  }

  #[inline]
  pub const fn get(&self) -> u64 {
    self.0
  }
}

impl Debug for UserID {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    write!(f, "UserID({})", self.0)
  }
}

impl Display for UserID {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    write!(f, "UserID({})", self.0)
  }
}

impl From<UserID> for Value {
  #[inline]
  fn from(other: UserID) -> Self {
    Value::Number(other.get().into())
  }
}
