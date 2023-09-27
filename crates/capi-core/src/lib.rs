//! Tools for working with Blizzard Classic Chat API (CAPI).

pub mod event;
pub mod packet;
pub mod payload;
pub mod request;
pub mod response;
pub mod types;

pub use self::event::EventPacket;
pub use self::event::EventType;
pub use self::packet::IntoPayload;
pub use self::packet::Packet;
pub use self::payload::Payload;
pub use self::request::RequestID;
pub use self::request::RequestPacket;
pub use self::request::RequestType;
pub use self::response::ResponsePacket;
pub use self::response::ResponseStatus;
pub use self::response::ResponseType;
