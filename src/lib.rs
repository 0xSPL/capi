pub mod core {
  #[doc(inline)]
  pub use capi_core::*;
}

pub mod socket {
  #[doc(inline)]
  #[cfg(feature = "channel")]
  pub use capi_socket::channel;

  #[doc(inline)]
  #[cfg(feature = "tungstenite")]
  pub use capi_socket::transport::tungstenite;

  #[doc(inline)]
  pub use capi_socket::Socket;

  #[doc(inline)]
  pub use capi_socket::SocketExt;

  #[doc(inline)]
  pub use capi_socket::SocketResponse;

  #[doc(inline)]
  pub use capi_socket::ENDPOINT;
}
