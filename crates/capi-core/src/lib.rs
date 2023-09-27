//! Tools for working with Blizzard Classic Chat API (CAPI).

pub mod event;
pub mod payload;
pub mod request;
pub mod response;

pub use self::event::EventPacket;
pub use self::event::EventType;
pub use self::payload::Payload;
pub use self::request::RequestID;
pub use self::request::RequestPacket;
pub use self::request::RequestType;
pub use self::response::ResponsePacket;
pub use self::response::ResponseStatus;
pub use self::response::ResponseType;
