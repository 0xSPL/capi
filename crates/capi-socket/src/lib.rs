//! WebSocket extensions for Blizzard Classic Chat API (CAPI).

#![allow(async_fn_in_trait)]
#![cfg_attr(docsrs, feature(doc_cfg))]

#[macro_use]
mod macros;

feature! {
  #[feature = "channel"]
  pub mod channel;
}

pub mod socket;
pub mod transport;

pub use self::socket::Socket;
pub use self::socket::SocketExt;
pub use self::socket::SocketResponse;

/// The chat API connection endpoint.
pub const ENDPOINT: &str = "wss://connect-bot.classic.blizzard.com/v1/rpc/chat";
