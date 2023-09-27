use std::future::Future;
use tokio::net::TcpStream;
use tokio_tungstenite::connect_async_with_config;
use tokio_tungstenite::MaybeTlsStream;
use tokio_tungstenite::WebSocketStream;
use tokio_tungstenite::tungstenite::error::Error;
use tokio_tungstenite::tungstenite::http::StatusCode;
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::tungstenite::protocol::WebSocketConfig;

use crate::ENDPOINT;
use crate::channel;
use crate::channel::Channel;
use crate::channel::Transport;

// =============================================================================
// WebSocket Config
// =============================================================================

#[derive(Debug)]
pub struct Config {
  pub(crate) endpoint: &'static str,
  pub(crate) socket: WebSocketConfig,
}

impl Config {
  #[inline]
  pub const fn new() -> Self {
    Self {
      endpoint: ENDPOINT,
      socket: Self::default_socket_config(),
    }
  }

  #[inline]
  pub const fn endpoint(mut self, endpoint: &'static str) -> Self {
    self.endpoint = endpoint;
    self
  }

  #[allow(deprecated)]
  #[inline]
  const fn default_socket_config() -> WebSocketConfig {
    WebSocketConfig {
      max_send_queue: None,                    // deprecated
      write_buffer_size: 1024 * 64,            // 64 KiB
      max_write_buffer_size: 1024 * 1024 * 2,  // 2 MiB
      max_message_size: Some(1024 * 1024 * 4), // 4 MiB
      max_frame_size: Some(1024 * 1024 * 2),   // 2 MiB
      accept_unmasked_frames: false,
    }
  }
}

// =============================================================================
// WebSocket Handler
// =============================================================================

type Socket = WebSocketStream<MaybeTlsStream<TcpStream>>;

/// Create a new socket connection.
#[inline]
pub async fn connect() -> Result<Channel<Socket>, Error> {
  connect_with_config(Config::new()).await
}

/// Create a new socket connection with the given `config`.
pub async fn connect_with_config(config: Config) -> Result<Channel<Socket>, Error> {
  let request: &'static str = config.endpoint;
  let config: Option<WebSocketConfig> = Some(config.socket);

  match connect_async_with_config(request, config, false).await {
    Ok((socket, response)) => {
      // Sanity Check
      assert_eq!(response.status(), StatusCode::SWITCHING_PROTOCOLS);

      Ok(Channel::new(socket))
    }
    Err(error) => {
      Err(error)
    }
  }
}

impl Transport for Socket {
  type Message = Message;
  type Invalid = Error;

  #[inline]
  fn shutdown(&mut self) -> impl Future<Output = Result<(), Self::Error>> + Send + '_ {
    self.close(None)
  }
}

impl channel::Message for Message {
  type Error = Error;

  #[inline]
  fn from_string(string: String) -> Self {
    Self::Text(string)
  }

  #[inline]
  fn into_string(self) -> Result<String, Self::Error> {
    match self {
      Message::Text(data) => Ok(data),
      Message::Binary(data) => String::from_utf8(data).map_err(Into::into),
      Message::Ping(_data) => panic!("[tungstenite][message::ping]"),
      Message::Pong(_data) => panic!("[tungstenite][message::pong]"),
      Message::Close(_data) => panic!("[tungstenite][message::close]"),
      Message::Frame(data) => data.into_string().map_err(Into::into),
    }
  }
}
