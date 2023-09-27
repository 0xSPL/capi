//! Chat API event types.
//!
//! This module contains types related to async chat server events.

use serde::Deserialize;
use serde::Serialize;

use crate::payload::Payload;
use crate::request::RequestID;

// =============================================================================
// Event Type
// =============================================================================

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
pub enum EventType {
  #[serde(rename = "Botapichat.ConnectEventRequest")]
  Connect,
  #[serde(rename = "Botapichat.DisconnectEventRequest")]
  Disconnect,
  #[serde(rename = "Botapichat.MessageEventRequest")]
  Message,
  #[serde(rename = "Botapichat.UserUpdateEventRequest")]
  UserUpdate,
  #[serde(rename = "Botapichat.UserLeaveEventRequest")]
  UserLeave,
}

// =============================================================================
// Event Packet
// =============================================================================

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct EventPacket {
  pub(crate) command: EventType,
  #[serde(rename = "request_id")]
  pub(crate) request: RequestID,
  pub(crate) payload: Payload,
}

impl EventPacket {
  /// Get the type identifier of the event.
  #[inline]
  pub const fn command(&self) -> EventType {
    self.command
  }

  /// Get the request ID of the event.
  #[inline]
  pub const fn request(&self) -> RequestID {
    self.request
  }

  /// Get the payload of the event.
  #[inline]
  pub const fn payload(&self) -> &Payload {
    &self.payload
  }
}
