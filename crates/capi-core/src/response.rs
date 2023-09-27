//! Chat API response types.
//!
//! This module contains types related to chat server responses.

use serde::Deserialize;
use serde::Serialize;

use crate::payload::Payload;
use crate::request::RequestID;

// =============================================================================
// Response Type
// =============================================================================

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
pub enum ResponseType {
  #[serde(rename = "Botapiauth.AuthenticateResponse")]
  Authenticate,
  #[serde(rename = "Botapichat.ConnectResponse")]
  Connect,
  #[serde(rename = "Botapichat.DisconnectResponse")]
  Disconnect,
  #[serde(rename = "Botapichat.SendMessageResponse")]
  SendMessage,
  #[serde(rename = "Botapichat.SendWhisperResponse")]
  SendWhisper,
  #[serde(rename = "Botapichat.BanUserResponse")]
  BanUser,
  #[serde(rename = "Botapichat.UnbanUserResponse")]
  UnbanUser,
  #[serde(rename = "Botapichat.SendEmoteResponse")]
  SendEmote,
  #[serde(rename = "Botapichat.KickUserResponse")]
  KickUser,
  #[serde(rename = "Botapichat.SetModeratorResponse")]
  SetModerator,
}

// =============================================================================
// Response Status
// =============================================================================

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
pub struct ResponseStatus {
  area: u8,
  code: u8,
}

impl ResponseStatus {
  #[inline]
  pub const fn area(&self) -> u8 {
    self.area
  }

  #[inline]
  pub const fn code(&self) -> u8 {
    self.code
  }

  #[inline]
  pub const fn as_str(&self) -> &'static str {
    match (self.area, self.code) {
      (0, 0) => "Ok",
      (8, 1) => "Not Connected to chat",
      (8, 2) => "Bad request",
      (6, 5) => "Reqeust timed out",
      (6, 8) => "Hit rate limit",
      (_, _) => "Unknown",
    }
  }
}

// =============================================================================
// Response Packet
// =============================================================================

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct ResponsePacket {
  pub(crate) command: ResponseType,
  #[serde(rename = "request_id")]
  pub(crate) request: RequestID,
  pub(crate) payload: Payload,
  pub(crate) status: Option<ResponseStatus>,
}

impl ResponsePacket {
  /// Get the type identifier of the response.
  #[inline]
  pub const fn command(&self) -> ResponseType {
    self.command
  }

  /// Get the request ID of the response.
  #[inline]
  pub const fn request(&self) -> RequestID {
    self.request
  }

  /// Get the payload of the response.
  #[inline]
  pub const fn payload(&self) -> &Payload {
    &self.payload
  }

  /// Get the status of the response.
  #[inline]
  pub const fn status(&self) -> Option<ResponseStatus> {
    self.status
  }
}
