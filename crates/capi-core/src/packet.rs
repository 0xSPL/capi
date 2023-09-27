use crate::event::EventType;
use crate::payload::Payload;
use crate::request::RequestType;
use crate::response::ResponseType;
use crate::types::UserID;

// =============================================================================
// Packet
// =============================================================================

mod private {
  pub trait Sealed {}
}

pub trait Packet: private::Sealed {
  const REQ_TYPE: RequestType;
  const RES_TYPE: ResponseType;
  const EVT_TYPE: Option<EventType>;
}

pub trait IntoPayload: private::Sealed {
  #[inline]
  fn into_payload(&self) -> Payload {
    Payload::new()
  }
}

// =============================================================================
// Authenticate
// =============================================================================

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct Authenticate<'a> {
  pub api_key: &'a str,
}

impl private::Sealed for Authenticate<'_> {}

impl Packet for Authenticate<'_> {
  const REQ_TYPE: RequestType = RequestType::Authenticate;
  const RES_TYPE: ResponseType = ResponseType::Authenticate;
  const EVT_TYPE: Option<EventType> = None;
}

impl IntoPayload for Authenticate<'_> {
  fn into_payload(&self) -> Payload {
    Payload::from_kv("api_key", self.api_key)
  }
}

// =============================================================================
// Connect
// =============================================================================

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct ChatConnect;

impl private::Sealed for ChatConnect {}

impl Packet for ChatConnect {
  const REQ_TYPE: RequestType = RequestType::Connect;
  const RES_TYPE: ResponseType = ResponseType::Connect;
  const EVT_TYPE: Option<EventType> = Some(EventType::Connect);
}

// =============================================================================
// ChatDisconnect
// =============================================================================

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct ChatDisconnect;

impl private::Sealed for ChatDisconnect {}

impl Packet for ChatDisconnect {
  const REQ_TYPE: RequestType = RequestType::Disconnect;
  const RES_TYPE: ResponseType = ResponseType::Disconnect;
  const EVT_TYPE: Option<EventType> = Some(EventType::Disconnect);
}

// =============================================================================
// ChatSendMessage
// =============================================================================

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct ChatSendMessage<'a> {
  pub message: &'a str,
}

impl private::Sealed for ChatSendMessage<'_> {}

impl Packet for ChatSendMessage<'_> {
  const REQ_TYPE: RequestType = RequestType::SendMessage;
  const RES_TYPE: ResponseType = ResponseType::SendMessage;
  const EVT_TYPE: Option<EventType> = None;
}

impl IntoPayload for ChatSendMessage<'_> {
  #[inline]
  fn into_payload(&self) -> Payload {
    Payload::from_kv("message", self.message)
  }
}

// =============================================================================
// ChatSendWhisper
// =============================================================================

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct ChatSendWhisper<'a> {
  pub message: &'a str,
  pub user_id: UserID,
}

impl private::Sealed for ChatSendWhisper<'_> {}

impl Packet for ChatSendWhisper<'_> {
  const REQ_TYPE: RequestType = RequestType::SendWhisper;
  const RES_TYPE: ResponseType = ResponseType::SendWhisper;
  const EVT_TYPE: Option<EventType> = None;
}

impl IntoPayload for ChatSendWhisper<'_> {
  #[inline]
  fn into_payload(&self) -> Payload {
    let mut payload: Payload = Payload::new();
    payload.insert("message", self.message);
    payload.insert("user_id", self.user_id);
    payload
  }
}

// =============================================================================
// ChatBanUser
// =============================================================================

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct ChatBanUser {
  pub user_id: UserID,
}

impl private::Sealed for ChatBanUser {}

impl Packet for ChatBanUser {
  const REQ_TYPE: RequestType = RequestType::BanUser;
  const RES_TYPE: ResponseType = ResponseType::BanUser;
  const EVT_TYPE: Option<EventType> = None;
}

impl IntoPayload for ChatBanUser {
  #[inline]
  fn into_payload(&self) -> Payload {
    Payload::from_kv("user_id", self.user_id)
  }
}

// =============================================================================
// ChatUnbanUser
// =============================================================================

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct ChatUnbanUser<'a> {
  pub toon_name: &'a str,
}

impl private::Sealed for ChatUnbanUser<'_> {}

impl Packet for ChatUnbanUser<'_> {
  const REQ_TYPE: RequestType = RequestType::UnbanUser;
  const RES_TYPE: ResponseType = ResponseType::UnbanUser;
  const EVT_TYPE: Option<EventType> = None;
}

impl IntoPayload for ChatUnbanUser<'_> {
  #[inline]
  fn into_payload(&self) -> Payload {
    Payload::from_kv("toon_name", self.toon_name)
  }
}

// =============================================================================
// ChatSendEmote
// =============================================================================

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct ChatSendEmote<'a> {
  pub message: &'a str,
}

impl private::Sealed for ChatSendEmote<'_> {}

impl Packet for ChatSendEmote<'_> {
  const REQ_TYPE: RequestType = RequestType::SendEmote;
  const RES_TYPE: ResponseType = ResponseType::SendEmote;
  const EVT_TYPE: Option<EventType> = None;
}

impl IntoPayload for ChatSendEmote<'_> {
  #[inline]
  fn into_payload(&self) -> Payload {
    Payload::from_kv("message", self.message)
  }
}

// =============================================================================
// ChatKickUser
// =============================================================================

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct ChatKickUser {
  pub user_id: UserID,
}

impl private::Sealed for ChatKickUser {}

impl Packet for ChatKickUser {
  const REQ_TYPE: RequestType = RequestType::KickUser;
  const RES_TYPE: ResponseType = ResponseType::KickUser;
  const EVT_TYPE: Option<EventType> = None;
}

impl IntoPayload for ChatKickUser {
  #[inline]
  fn into_payload(&self) -> Payload {
    Payload::from_kv("user_id", self.user_id)
  }
}

// =============================================================================
// ChatSetModerator
// =============================================================================

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct ChatSetModerator {
  pub user_id: UserID,
}

impl private::Sealed for ChatSetModerator {}

impl Packet for ChatSetModerator {
  const REQ_TYPE: RequestType = RequestType::SetModerator;
  const RES_TYPE: ResponseType = ResponseType::SetModerator;
  const EVT_TYPE: Option<EventType> = None;
}

impl IntoPayload for ChatSetModerator {
  #[inline]
  fn into_payload(&self) -> Payload {
    Payload::from_kv("user_id", self.user_id)
  }
}
