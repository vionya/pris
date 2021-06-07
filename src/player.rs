#![allow(clippy::needless_lifetimes)]

use crate::util;
use dbus::nonblock::SyncConnection;
use std::error::Error;

/// A struct used to control an MPRIS player.
#[derive(Debug)]
pub struct Player<'a> {
    pub name: &'a str,
}

impl Player<'_> {
    /// Tries to create a new `Player` instance from a given name.
    ///
    /// # Errors
    /// Returns an `Err` if the provided player is invalid.
    pub async fn try_new<'a>(
        name: &'a str,
        conn: &SyncConnection,
    ) -> Result<Player<'a>, Box<dyn Error>> {
        if !util::check_validity(name, conn).await? {
            return Err(Box::from("The provided player was invalid."));
        }

        let player = Player { name };
        Ok(player)
    }
}
