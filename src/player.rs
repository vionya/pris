#![allow(clippy::needless_lifetimes)]

use crate::{methods, util};
use dbus::nonblock::{Proxy, SyncConnection};
use std::{error::Error, time::Duration};

type Result<T> = std::result::Result<T, Box<dyn Error>>;

/// A struct used to control an MPRIS player.
#[derive(Clone)]
pub struct Player<'a, 'b> {
    pub name: &'a str,
    pub conn: &'b SyncConnection,
}

impl<'a, 'b> Player<'a, 'b> {
    /// Tries to create a new `Player` instance from a given name.
    ///
    /// # Errors
    /// Returns an `Err` if the provided player is invalid.
    pub async fn try_new(name: &'a str, conn: &'b SyncConnection) -> Result<Player<'a, 'b>> {
        if !util::check_validity(name, conn).await? {
            return Err(Box::from("The provided player was invalid."));
        }

        let player = Player { name, conn };
        Ok(player)
    }

    #[doc(hidden)]
    pub fn get_proxy(&mut self) -> Result<Proxy<&'b SyncConnection>> {
        let proxy = Proxy::new(
            format!("org.mpris.MediaPlayer2.{}", self.name),
            "/org/mpris/MediaPlayer2",
            Duration::from_millis(5000),
            self.conn,
        );

        Ok(proxy)
    }

    /// Skips to the next track
    pub async fn next(&mut self) {
        methods::next(self).await.unwrap();
    }

    /// Skips to the previous track
    pub async fn previous(&mut self) {
        methods::previous(self).await.unwrap();
    }

    /// Pauses the current track
    pub async fn pause(&mut self) {
        methods::pause(self).await.unwrap();
    }

    /// Starts or resumes the current track
    pub async fn play(&mut self) {
        methods::play(self).await.unwrap();
    }

    /// Resumes/starts or pauses the current track
    pub async fn play_pause(&mut self) {
        methods::play_pause(self).await.unwrap();
    }

    /// Stops playback
    pub async fn stop(&mut self) {
        methods::stop(self).await.unwrap();
    }
}
