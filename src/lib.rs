mod event_manager;
mod player;
mod util;

pub mod methods;

#[doc(no_inline)]
pub use dbus::message::Message;
pub use event_manager::*;
pub use player::*;

use dbus::nonblock::SyncConnection;
use dbus_tokio::connection;
use std::sync::Arc;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

/// Establishes a connection to the DBus.
/// Use this to create a connection to pass into `Player`
pub fn get_connection() -> Arc<SyncConnection> {
    let (resource, conn) = connection::new_session_sync().unwrap();

    tokio::spawn(async {
        let err = resource.await;
        panic!("Lost connection to D-Bus: {}", err);
    });

    conn
}
