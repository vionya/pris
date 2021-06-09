mod event_manager;
mod player;
mod util;

pub mod methods;

#[doc(no_inline)]
pub use dbus::message::Message;
pub use event_manager::*;
pub use player::*;
pub use util::{get_all_players, get_connection};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
