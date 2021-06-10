use crate::{Player, Result};
use dbus::{
    arg::{PropMap, RefArg},
    nonblock::{Proxy, SyncConnection},
};
use dbus_tokio::connection;
use std::{sync::Arc, time::Duration};

const MPRIS_PREFIX: &str = "org.mpris.MediaPlayer2.";

pub async fn validate(player_name: &str, conn: &SyncConnection) -> Result<bool> {
    Ok(get_all_names(&conn)
        .await?
        .contains(&player_name.to_string()))
}

async fn get_all_names(conn: &SyncConnection) -> Result<Vec<String>> {
    let proxy = Proxy::new("org.freedesktop.DBus", "/", Duration::from_secs(1), conn);
    let (services,): (Vec<String>,) = proxy
        .method_call("org.freedesktop.DBus", "ListNames", ())
        .await?;

    let active_players: Vec<String> = services
        .into_iter()
        .filter_map(|name| {
            name.strip_prefix(&MPRIS_PREFIX)
                .map_or_else(|| None, |s| Some(s.to_string()))
        })
        .collect();
    Ok(active_players)
}

/// Establishes a connection to the `DBus`.
/// Use this to create a connection to pass into `Player`.
pub fn get_connection() -> Arc<SyncConnection> {
    let (resource, conn) = connection::new_session_sync().unwrap();

    tokio::spawn(async {
        let err = resource.await;
        panic!("Lost connection to D-Bus: {}", err);
    });

    conn
}

/// Gets a `Vec` of `Player`s from all active
/// MPRIS players found on the `DBus`.
///
/// # Errors
/// May return an `Err` variant if there was a failure in
/// getting a list of names from `DBus`.
pub async fn get_all_players<'a>(conn: &'a SyncConnection) -> Result<Vec<Player<'_>>> {
    let mut players: Vec<Player<'a>> = Vec::new();

    for name in get_all_names(&conn).await? {
        match Player::try_new(name, &conn).await {
            Ok(player) => players.push(player),
            Err(_) => continue,
        };
    }

    Ok(players)
}

/// Gets a value from a HashMap, and casts it to the
/// type provided.
///
/// # Example
/// ```
/// let metadata = player.get_metadata().await?;
/// let title = match prop_cast::<String>(&metadata, "xesam:title") {
///     Some(t) => t.to_string(),
///     None => "Unknown title".to_string()
/// };
/// ```
pub fn prop_cast<'a, T>(map: &'a PropMap, key: &str) -> Option<&'a T>
where
    T: 'static,
{
    map.get(key).and_then(|v| v.0.as_any().downcast_ref())
}
