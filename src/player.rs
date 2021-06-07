use crate::util;
use dbus::nonblock::SyncConnection;
use std::error::Error;

pub struct Player<'a> {
    pub name: &'a str,
}

impl Player<'_> {
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
