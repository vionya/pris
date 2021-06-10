use super::INTERFACE;
use crate::{prop_cast, Player, Result};
use dbus::nonblock::stdintf::org_freedesktop_dbus::Properties;
use dbus::{
    arg::{Append, Arg, Get, PropMap},
    strings::Path,
};
use std::time::Duration;

/// Retrieves track metadata from a `Player`.
/// The [`prop_cast`](crate::prop_cast) function may be used
/// to get specific values out of the resulting metadata.
///
/// # Errors
/// May `Err` if there is a failure in getting the metadata.
pub async fn get_metadata(player: &mut Player<'_>) -> Result<PropMap> {
    let proxy = player.get_proxy()?;
    let metadata: PropMap = proxy.get(INTERFACE, "Metadata").await?;
    Ok(metadata)
}

/// Retrieves the value of an MPRIS property.
/// Available properties can be found [here].
///
/// [here]: https://specifications.freedesktop.org/mpris-spec/latest/Player_Interface.html#Property:PlaybackStatus
///
/// # Errors
/// May return an `Err` variant if:
/// * An invalid type was provided for the property
/// * An invalid property was provided
pub async fn get_property<T>(player: &mut Player<'_>, property: &str) -> Result<T>
where
    T: for<'a> Get<'a> + 'static,
{
    let proxy = player.get_proxy()?;
    let value: T = proxy.get(INTERFACE, property).await?;

    Ok(value)
}

/// Sets the value of a writable MPRIS property.
/// Available properties can be found [here].
///
/// [here]: https://specifications.freedesktop.org/mpris-spec/latest/Player_Interface.html#Property:PlaybackStatus
///
/// # Errors
/// May return an `Err` variant if:
/// * An invalid type was provided for the property
/// * An invalid property was provided
pub async fn set_property<T>(player: &mut Player<'_>, property: &str, value: T) -> Result<()>
where
    T: Arg + Append,
{
    let proxy = player.get_proxy()?;
    proxy.set(INTERFACE, property, value).await?;

    Ok(())
}

/// Seeks the position of the active track.
pub async fn seek(player: &mut Player<'_>, offset: Duration) -> Result<()> {
    let proxy = player.get_proxy()?;
    let offset = offset.as_micros() as i64;
    proxy.method_call(INTERFACE, "Seek", (offset,)).await?;

    Ok(())
}

/// Same as `seek`, but in reverse.
pub async fn seek_reverse(player: &mut Player<'_>, offset: Duration) -> Result<()> {
    let proxy = player.get_proxy()?;
    let offset = offset.as_micros() as i64;
    proxy.method_call(INTERFACE, "Seek", (-offset,)).await?;

    Ok(())
}

/// Sets the position of the current track, by microseconds.
pub async fn set_position(player: &mut Player<'_>, position: i64) -> Result<()> {
    let mut player_clone = player.clone();

    let proxy = player.get_proxy()?;
    let metadata = get_metadata(&mut player_clone).await?;
    let track_id: &String = prop_cast(&metadata, "mpris:trackid").unwrap();

    proxy
        .method_call(INTERFACE, "SetPosition", (Path::from(track_id), position))
        .await?;

    Ok(())
}

/// Opens a track by its URI.
///
/// # Errors
/// May return an `Err` variant if the provided URI is invalid.
pub async fn open_uri(player: &mut Player<'_>, uri: &str) -> Result<()> {
    let proxy = player.get_proxy()?;
    proxy.method_call(INTERFACE, "OpenUri", (uri,)).await?;

    Ok(())
}
