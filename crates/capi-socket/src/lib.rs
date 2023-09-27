//! WebSocket extensions for Blizzard Classic Chat API (CAPI).

#![feature(async_fn_in_trait)]

pub mod socket;

pub use self::socket::Socket;
pub use self::socket::SocketExt;
pub use self::socket::SocketResponse;
