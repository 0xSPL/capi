use std::error::Error;
use std::future::Future;

use self::private::Connection;

// =============================================================================
// Connection ("Sealed")
// =============================================================================

mod private {
  use futures_util::Sink;
  use futures_util::Stream;

  pub trait Connection<T, U>: Sink<T> + Stream<Item = Result<T, U>> {}

  impl<T, U, V> Connection<T, U> for V
  where
    V: Sink<T>,
    V: Stream<Item = Result<T, U>>,
  {}
}

// =============================================================================
// Transport
// =============================================================================

pub trait Transport: Connection<Self::Message, Self::Invalid> + Unpin + Send + 'static {
  type Message: Message;
  type Invalid: Error + Send;

  // Note: async fn in trait does not currently apply `Send` bound.
  //
  // https://blog.rust-lang.org/inside-rust/2022/11/17/async-fn-in-trait-nightly.html
  fn shutdown(&mut self) -> impl Future<Output = Result<(), Self::Error>> + Send + '_;
}

// =============================================================================
// Transport Message
// =============================================================================

pub trait Message: Send + 'static {
  type Error: Error + Send;

  fn from_string(string: String) -> Self;
  fn into_string(self) -> Result<String, Self::Error>;
}
