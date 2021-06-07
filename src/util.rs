use dbus::nonblock::{Proxy, SyncConnection};
use std::{error::Error, time::Duration};

const MPRIS_PREFIX: &str = "org.mpris.MediaPlayer2.";

pub async fn check_validity(
    player_name: &str,
    conn: &SyncConnection,
) -> Result<bool, Box<dyn Error>> {
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

    Ok(active_players.contains(&player_name.to_string()))
}
