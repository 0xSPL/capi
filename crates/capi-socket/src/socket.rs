use capi_core::packet::Authenticate;
use capi_core::packet::ChatBanUser;
use capi_core::packet::ChatConnect;
use capi_core::packet::ChatDisconnect;
use capi_core::packet::ChatKickUser;
use capi_core::packet::ChatSendEmote;
use capi_core::packet::ChatSendMessage;
use capi_core::packet::ChatSendWhisper;
use capi_core::packet::ChatSetModerator;
use capi_core::packet::ChatUnbanUser;
use capi_core::types::UserID;
use capi_core::IntoPayload;
use capi_core::Packet;
use capi_core::ResponsePacket;

/// Alias for [`Result<ResponsePacket, T::Error>`]
pub type SocketResponse<T> = Result<ResponsePacket, <T as Socket>::Error>;

// =============================================================================
// Socket
// =============================================================================

pub trait Socket {
  type Error;

  async fn send<T>(&self, payload: T) -> SocketResponse<Self>
  where
    T: Packet + IntoPayload;
}

// =============================================================================
// Socket Extension
// =============================================================================

mod private {
  pub trait Sealed {}
}

pub trait SocketExt: Socket + private::Sealed {
  /// Send an authentication request with the API key.
  #[inline]
  async fn send_authenticate(&self, api_key: &str) -> SocketResponse<Self> {
    self.send(Authenticate { api_key }).await
  }

  /// Connect the bot to the gateway and chat channel.
  ///
  /// @event: { channel: String }
  #[inline]
  async fn send_connect(&self) -> SocketResponse<Self> {
    self.send(ChatConnect).await
  }

  /// Disconnects the bot from the gateway and chat channel.
  #[inline]
  async fn send_disconnect(&self) -> SocketResponse<Self> {
    self.send(ChatDisconnect).await
  }

  /// Sends a chat message to the channel.
  #[inline]
  async fn send_message(&self, message: &str) -> SocketResponse<Self> {
    self.send(ChatSendMessage { message }).await
  }

  /// Sends a chat message to one user in the channel.
  #[inline]
  async fn send_whisper(&self, message: &str, user_id: UserID) -> SocketResponse<Self> {
    self.send(ChatSendWhisper { message, user_id }).await
  }

  /// Bans a user from the channel.
  #[inline]
  async fn ban_user(&self, user_id: UserID) -> SocketResponse<Self> {
    self.send(ChatBanUser { user_id }).await
  }

  /// Un-Bans a user from the channel.
  #[inline]
  async fn unban_user(&self, toon_name: &str) -> SocketResponse<Self> {
    self.send(ChatUnbanUser { toon_name }).await
  }

  /// Sends an emote on behalf of a bot.
  #[inline]
  async fn send_emote(&self, message: &str) -> SocketResponse<Self> {
    self.send(ChatSendEmote { message }).await
  }

  /// Kicks a user from the channel.
  #[inline]
  async fn kick_user(&self, user_id: UserID) -> SocketResponse<Self> {
    self.send(ChatKickUser { user_id }).await
  }

  /// Sets the current chat moderator to a member of the current chat.
  ///
  /// Note: Same as a normal user doing `/desginate` followed by `/resign`.
  #[inline]
  async fn set_moderator(&self, user_id: UserID) -> SocketResponse<Self> {
    self.send(ChatSetModerator { user_id }).await
  }
}

// Implement `SocketExt` for any types that implement `Socket`
impl<T: Socket> private::Sealed for T {}
impl<T: Socket> SocketExt for T {}
