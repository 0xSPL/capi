use capi_core::EventPacket;
use capi_core::IntoPayload;
use capi_core::Packet;
use capi_core::RequestID;
use capi_core::RequestPacket;
use capi_core::ResponsePacket;
use futures_util::stream::SplitSink;
use futures_util::stream::SplitStream;
use futures_util::SinkExt;
use futures_util::Stream;
use futures_util::StreamExt;
use serde::de::DeserializeOwned;
use serde::Deserialize;
use serde::Serialize;
use serde_json::from_str;
use serde_json::to_string;
use std::collections::BTreeMap;
use std::error::Error;
use std::marker::PhantomData;
use tokio::sync::mpsc;
use tokio::sync::mpsc::error::TryRecvError;
use tokio::sync::oneshot;
use tokio::task;
use tokio::task::JoinHandle;
use tokio_stream::wrappers::ReceiverStream;

use crate::channel::ChannelError;
use crate::channel::ErrorKind;
use crate::channel::Transport;
use crate::channel::Message;
use crate::socket::Socket;
use crate::socket::SocketResponse;

// =============================================================================
// Misc. Types
// =============================================================================

type ClientRecv<T> = mpsc::Receiver<Command<T>>;
type ClientSend<T> = mpsc::Sender<Command<T>>;

type ServerRecv<T> = ReceiverStream<T>;
type ServerSend<T> = mpsc::Sender<T>;

type SoloRecv<T> = oneshot::Receiver<T>;
type SoloSend<T> = oneshot::Sender<T>;

// =============================================================================
// Channel Command
// =============================================================================

enum Command<T> {
  SendRequest(Request<T>),
  ShutdownExit,
  ShutdownKill,
}

// =============================================================================
// Command Request
// =============================================================================

struct Request<T> {
  message: T,
  request: RequestID,
  oneshot: SoloSend<T>,
}

// =============================================================================
// Command Response (ID)
// =============================================================================

#[derive(Deserialize)]
struct Response {
  request_id: RequestID,
}

// =============================================================================
// Request/Response MailBox
// =============================================================================

struct MailBox<T> {
  channel: ServerSend<T>,
  tracker: BTreeMap<RequestID, SoloSend<T>>,
}

impl<T> MailBox<T> {
  #[inline]
  fn new(channel: ServerSend<T>) -> Self {
    Self {
      channel,
      tracker: BTreeMap::new(),
    }
  }

  fn track(&mut self, request: RequestID, oneshot: SoloSend<T>) {
    assert!(!self.tracker.contains_key(&request));
    self.tracker.insert(request, oneshot);
  }

  async fn send(&mut self, response: Response, message: T) -> Result<(), ChannelError> {
    // TODO: Probably should track by request type
    if let Some(oneshot) = self.tracker.remove(&response.request_id) {
      static ERR: &str = "failed to broadcast response (oneshot)";

      oneshot
        .send(message)
        .map_err(|_| ChannelError::msg(ErrorKind::ChannSend, ERR))
    } else {
      static ERR: &str = "failed to broadcast response (mailbox)";

      self
        .channel
        .send(message)
        .await
        .map_err(|_| ChannelError::msg(ErrorKind::ChannSend, ERR))
    }
  }
}

// =============================================================================
// Generic Channel
// =============================================================================

#[derive(Debug)]
pub struct Channel<T: Transport> {
  recv: ServerRecv<T::Message>,
  send: ClientSend<T::Message>,
  task: JoinHandle<Result<(), ChannelError>>,
  phantom: PhantomData<T>,
}

impl<T: Transport> Channel<T> {
  const BUFFER_CLIENT: usize = 0x20;
  const BUFFER_SERVER: usize = 0x20;

  /// Create a new `Channel` from the given [`transport`][Transport].
  pub fn new(transport: T) -> Self
  where
    T::Error: Error + Send,
  {
    let (client_send, client_recv) = mpsc::channel(Self::BUFFER_CLIENT);
    let (server_send, server_recv) = mpsc::channel(Self::BUFFER_SERVER);

    Self {
      recv: ReceiverStream::new(server_recv),
      send: client_send,
      task: task::spawn(process(transport, server_send, client_recv)),
      phantom: PhantomData,
    }
  }

  /// Returns a reference to the async task handle.
  #[inline]
  pub const fn task(&self) -> &JoinHandle<Result<(), ChannelError>> {
    &self.task
  }

  /// Waits for the WebSocket server task to finish.
  ///
  /// # Note
  ///
  /// [`stop`][Self::stop] or [`kill`][Self::kill] should be called prior,
  /// otherwise this may block indefinitely.
  #[inline]
  pub async fn join(self) -> Result<(), ChannelError> {
    match self.task.await {
      Ok(output) => output,
      Err(error) => Err(ChannelError::new(ErrorKind::JoinError, error)),
    }
  }

  /// Gracefully stops the WebSocket server.
  #[inline]
  pub async fn stop(&self) -> Result<(), ChannelError> {
    self.push(Command::ShutdownExit).await
  }

  /// Forcibly stops the WebSocket server.
  #[inline]
  pub async fn kill(&self) -> Result<(), ChannelError> {
    self.push(Command::ShutdownKill).await
  }

  /// Returns a [`stream`][Stream] of [`events`][EventPacket].
  pub fn event_stream(&mut self) -> impl Stream<Item = Result<EventPacket, ChannelError>> + '_ {
    (&mut self.recv).map(decode_message)
  }

  /// Returns a vector of [`events`][EventPacket].
  pub fn events(&mut self) -> Result<Vec<EventPacket>, ChannelError> {
    let mut events: Vec<EventPacket> = Vec::new();

    loop {
      match self.recv.as_mut().try_recv() {
        Ok(message) => {
          events.push(decode_message(message)?);
        }
        Err(TryRecvError::Empty) => {
          break;
        }
        Err(error @ TryRecvError::Disconnected) => {
          return Err(ChannelError::new(ErrorKind::ChannRecv, error));
        }
      }
    }

    Ok(events)
  }

  // ===========================================================================
  // Private API
  // ===========================================================================

  async fn push(&self, command: Command<T::Message>) -> Result<(), ChannelError> {
    self
      .send
      .send(command)
      .await
      .map_err(|error| ChannelError::new(ErrorKind::ChannSend, error))
  }

  async fn send<P>(&self, payload: P) -> Result<ResponsePacket, ChannelError>
  where
    P: Packet + IntoPayload,
  {
    let (send, recv): (SoloSend<T::Message>, SoloRecv<T::Message>) = oneshot::channel();

    let request: RequestPacket = RequestPacket::new(payload);
    let command: Command<T::Message> = Self::request(&request, send)?;

    self.push(command).await?;

    let response: ResponsePacket = recv
      .await
      .map_err(|error| ChannelError::new(ErrorKind::ChannRecv, error))
      .and_then(decode_message)?;

    assert_eq!(response.command(), P::RES_TYPE);
    assert_eq!(response.request(), request.request());

    if let Some(status) = response.status() {
      return Err(ChannelError::status(status));
    }

    Ok(response)
  }

  #[inline]
  fn request(
    packet: &RequestPacket,
    sender: SoloSend<T::Message>,
  ) -> Result<Command<T::Message>, ChannelError> {
    Ok(Command::SendRequest(Request {
      message: encode(packet).map(T::Message::from_string)?,
      request: packet.request(),
      oneshot: sender,
    }))
  }
}

#[inline]
fn decode_message<T: Message, U: DeserializeOwned>(message: T) -> Result<U, ChannelError> {
  let text: String = into_string(message)?;
  let data: U = decode(text.as_str())?;

  Ok(data)
}

#[inline]
fn into_string<T: Message>(message: T) -> Result<String, ChannelError> {
  message
    .into_string()
    .map_err(|error| ChannelError::new(ErrorKind::Socket, error))
}

#[inline]
fn encode<T: Serialize>(data: &T) -> Result<String, ChannelError> {
  to_string(data).map_err(|error| ChannelError::new(ErrorKind::Encode, error))
}

#[inline]
fn decode<'de, T: Deserialize<'de>>(data: &'de str) -> Result<T, ChannelError> {
  from_str(data).map_err(|error| ChannelError::new(ErrorKind::Decode, error))
}

// =============================================================================
// Socket Implementation
// =============================================================================

impl<T: Transport> Socket for Channel<T> {
  type Error = ChannelError;

  #[inline]
  async fn send<P>(&self, payload: P) -> SocketResponse<Self>
  where
    P: Packet + IntoPayload,
  {
    Channel::send(self, payload).await
  }
}

// =============================================================================
// Internal Message Handler
// =============================================================================

async fn process<T>(
  transport: T,
  send: ServerSend<T::Message>,
  mut recv: ClientRecv<T::Message>,
) -> Result<(), ChannelError>
where
  T: Transport,
  T::Error: Error + Send,
{
  // Split the socket into writer/reader
  let (mut ssend, mut srecv): (SplitSink<T, T::Message>, SplitStream<T>) = transport.split();

  // Tracker for request/response channels
  let mut mailbox: MailBox<T::Message> = MailBox::new(send);

  'runloop: loop {
    tokio::select! {
      // Process client commands
      Some(command) = recv.recv() => {
        match command {
          Command::SendRequest(Request { message, request, oneshot }) => {
            mailbox.track(request, oneshot);

            if let Err(error) = ssend.send(message).await {
              return Err(ChannelError::new(ErrorKind::ChannSend, error));
            }
          }
          Command::ShutdownExit => {
            let Ok(mut transport) = ssend.reunite(srecv) else {
              static ERR: &str = "failed to reunite split `stream + sink`";
              return Err(ChannelError::msg(ErrorKind::Socket, ERR));
            };

            transport
              .shutdown()
              .await
              .map_err(|error| ChannelError::new(ErrorKind::Socket, error))?;

            break 'runloop;
          }
          Command::ShutdownKill => {
            break 'runloop;
          }
        }
      }
      // Process WebSocket messages
      Some(message) = srecv.next() => {
        let Ok(message) = message else {
          panic!("TODO");
        };

        let text: String = into_string(message)?;
        let data: Response = decode(text.as_str())?;

        mailbox.send(data, T::Message::from_string(text)).await?;
      }
    }
  }

  Ok(())
}
