//! This library provides a high-level interface to the [MPRIS] `DBus` specification.
//!
//! [MPRIS]: https://specifications.freedesktop.org/mpris-spec/latest/
//! 
//! It allows both controlling of a player, as well as listening
//! for events and executing callbacks.
//! 
//! # A basic player controller
//! ```
//! use empress::{self, Player};
//! 
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Create a connection to work with
//!     let conn = empress::get_connection();
//!     // Get a player under the name "vlc"
//!     let player = Player::try_new("vlc", &conn).await?;
//!     // Play/pause the player
//!     player.play_pause().await?;
//! }
//! ```
//! ---
//! This crate re-exports [`Message`](dbus::message::Message) for use
//! in typing non-closure callbacks.
mod event_manager;
mod player;
mod util;

pub mod methods;

#[doc(no_inline)]
pub use dbus::message::Message;
pub use event_manager::*;
pub use player::*;
pub use util::{get_all_players, get_connection, prop_cast};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
