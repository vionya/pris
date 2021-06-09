use super::INTERFACE;
use crate::{Player, Result};
use dbus::nonblock::stdintf::org_freedesktop_dbus::Properties;
use dbus::{
    arg::{Append, Arg, Get, PropMap, RefArg},
    strings::Path,
};
use std::time::Duration;

/// Retrieves a metadata property from the given player.
pub async fn get_metadata_property(
    player: &mut Player<'_, '_>,
    property: &str,
) -> Result<Box<dyn RefArg>> {
    let proxy = player.get_proxy()?;
    let mut metadata: PropMap = proxy.get(INTERFACE, "Metadata").await?;

    let prop = metadata.remove(property).unwrap();

    Ok(prop.0)
}

/// Retrieves the value of an MPRIS property.
/// Available properties can be seen [here].
///
/// [here]: https://specifications.freedesktop.org/mpris-spec/latest/Player_Interface.html#Property:PlaybackStatus
pub async fn get_property<T>(player: &mut Player<'_, '_>, property: &str) -> Result<T>
where
    T: for<'a> Get<'a> + 'static,
{
    let proxy = player.get_proxy()?;
    let value: T = proxy.get(INTERFACE, property).await?;

    Ok(value)
}

/// Sets the value of a writable MPRIS property.
/// Available properties can be seen [here].
///
/// [here]: https://specifications.freedesktop.org/mpris-spec/latest/Player_Interface.html#Property:PlaybackStatus
pub async fn set_property<T>(player: &mut Player<'_, '_>, property: &str, value: T) -> Result<()>
where
    T: Arg + Append,
{
    let proxy = player.get_proxy()?;
    proxy.set(INTERFACE, property, value).await?;

    Ok(())
}

/// Seeks the position of the active track.
pub async fn seek(player: &mut Player<'_, '_>, offset: Duration) -> Result<()> {
    let proxy = player.get_proxy()?;
    let offset = offset.as_micros() as i64;
    proxy.method_call(INTERFACE, "Seek", (offset,)).await?;

    Ok(())
}

/// Same as `seek`, but in reverse.
pub async fn seek_reverse(player: &mut Player<'_, '_>, offset: Duration) -> Result<()> {
    let proxy = player.get_proxy()?;
    let offset = offset.as_micros() as i64;
    proxy.method_call(INTERFACE, "Seek", (-offset,)).await?;

    Ok(())
}

/// Sets the position of the current track, by microseconds.
pub async fn set_position(player: &mut Player<'_, '_>, position: i64) -> Result<()> {
    let mut player_clone = player.clone();

    let proxy = player.get_proxy()?;
    let track_id = get_metadata_property(&mut player_clone, "mpris:trackid").await?;

    proxy
        .method_call(
            INTERFACE,
            "SetPosition",
            (Path::from(track_id.as_str().unwrap()), position),
        )
        .await?;

    Ok(())
}

/// Opens a track by its URI.
pub async fn open_uri(player: &mut Player<'_, '_>, uri: &str) -> Result<()> {
    let proxy = player.get_proxy()?;
    proxy.method_call(INTERFACE, "OpenUri", (uri,)).await?;

    Ok(())
}
