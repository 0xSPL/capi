use capi_core::ResponseStatus;
use std::error::Error;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

// =============================================================================
// Channel Error
// =============================================================================

#[derive(Debug)]
pub struct ChannelError {
  kind: ErrorKind,
  source: ErrorSource,
}

impl ChannelError {
  #[inline]
  pub(crate) fn new(kind: ErrorKind, source: impl Error + Send + 'static) -> Self {
    Self {
      kind,
      source: ErrorSource::Source(Box::new(source)),
    }
  }

  #[inline]
  pub(crate) fn msg(kind: ErrorKind, message: impl Display) -> Self {
    Self {
      kind,
      source: ErrorSource::String(message.to_string()),
    }
  }

  #[inline]
  pub(crate) fn status(status: ResponseStatus) -> Self {
    Self {
      kind: ErrorKind::Status,
      source: ErrorSource::Status(status),
    }
  }
}

impl Display for ChannelError {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    write!(f, "[{}]: {}", self.kind, self.source)
  }
}

impl Error for ChannelError {
  fn source(&self) -> Option<&(dyn Error + 'static)> {
    match self.source {
      ErrorSource::Source(ref inner) => Some(&**inner),
      ErrorSource::Status(_) => None,
      ErrorSource::String(_) => None,
    }
  }
}

#[derive(Clone, Copy, Debug)]
pub enum ErrorKind {
  Socket,
  Encode,
  Decode,
  Status,
  ChannRecv,
  ChannSend,
  JoinError,
}

impl Display for ErrorKind {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    match self {
      Self::Socket => write!(f, "socket error"),
      Self::Encode => write!(f, "encode error"),
      Self::Decode => write!(f, "decode error"),
      Self::Status => write!(f, "status error"),
      Self::ChannRecv => write!(f, "recv error"),
      Self::ChannSend => write!(f, "send error"),
      Self::JoinError => write!(f, "join error"),
    }
  }
}

#[derive(Debug)]
enum ErrorSource {
  Source(Box<dyn Error + Send + 'static>),
  Status(ResponseStatus),
  String(String),
}

impl Display for ErrorSource {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    match self {
      Self::Source(inner) => Display::fmt(inner, f),
      Self::Status(inner) => Display::fmt(inner.as_str(), f),
      Self::String(inner) => Display::fmt(inner, f),
    }
  }
}
