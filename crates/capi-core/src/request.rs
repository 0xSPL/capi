//! Chat API request types.
//!
//! This module contains types related to chat server requests.

use serde::Deserialize;
use serde::Serialize;

use crate::payload::Payload;

// =============================================================================
// Request Type
// =============================================================================

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
pub enum RequestType {
  #[serde(rename = "Botapiauth.AuthenticateRequest")]
  Authenticate,
  #[serde(rename = "Botapichat.ConnectRequest")]
  Connect,
  #[serde(rename = "Botapichat.DisconnectRequest")]
  Disconnect,
  #[serde(rename = "Botapichat.SendMessageRequest")]
  SendMessage,
  #[serde(rename = "Botapichat.SendWhisperRequest")]
  SendWhisper,
  #[serde(rename = "Botapichat.BanUserRequest")]
  BanUser,
  #[serde(rename = "Botapichat.UnbanUserRequest")]
  UnbanUser,
  #[serde(rename = "Botapichat.SendEmoteRequest")]
  SendEmote,
  #[serde(rename = "Botapichat.KickUserRequest")]
  KickUser,
  #[serde(rename = "Botapichat.SetModeratorRequest")]
  SetModerator,
}

// =============================================================================
// Request ID
// =============================================================================

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
#[serde(transparent)]
pub struct RequestID(u64);

// =============================================================================
// Request Packet
// =============================================================================

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct RequestPacket {
  pub(crate) command: RequestType,
  #[serde(rename = "request_id")]
  pub(crate) request: RequestID,
  pub(crate) payload: Payload,
}

impl RequestPacket {
  /// Get the type identifier of the request.
  #[inline]
  pub const fn command(&self) -> RequestType {
    self.command
  }

  /// Get the request ID of the request.
  #[inline]
  pub const fn request(&self) -> RequestID {
    self.request
  }

  /// Get the payload of the request.
  #[inline]
  pub const fn payload(&self) -> &Payload {
    &self.payload
  }
}
