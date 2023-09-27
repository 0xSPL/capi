use capi_core::EventPacket;
use capi_core::RequestID;
use capi_core::RequestPacket;
use capi_core::ResponsePacket;
use futures_util::Sink;
use futures_util::Stream;
use std::error::Error;
use std::future::Future;

// =============================================================================
// Transport
// =============================================================================

pub trait Transport:
  Sink<Self::Message> + Stream<Item = Self::Message> + Unpin + Send + 'static
{
  type Message: Send;
  type EncodeError: Error + Send;
  type DecodeError: Error + Send;

  fn encode(packet: &RequestPacket) -> Result<Self::Message, Self::EncodeError>;
  fn decode(packet: Self::Message) -> Result<ResponsePacket, Self::DecodeError>;

  fn event(packet: Self::Message) -> Result<EventPacket, Self::DecodeError>;
  fn ident(packet: &Self::Message) -> Result<RequestID, Self::DecodeError>;

  // https://blog.rust-lang.org/inside-rust/2022/11/17/async-fn-in-trait-nightly.html#workarounds-available-in-the-stable-compiler
  fn close(&mut self) -> impl Future<Output = Result<(), Self::Error>> + Send + '_;
}
