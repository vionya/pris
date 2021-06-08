use crate::Player;

const INTERFACE: &str = "org.mpris.MediaPlayer2.Player";

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

/// Skips to the next track
pub async fn next(player: &mut Player<'_, '_>) -> Result<()> {
    let proxy = player.get_proxy()?;
    proxy.method_call(INTERFACE, "Next", ()).await?;

    Ok(())
}

/// Skips to the previous track
pub async fn previous(player: &mut Player<'_, '_>) -> Result<()> {
    let proxy = player.get_proxy()?;
    proxy.method_call(INTERFACE, "Previous", ()).await?;

    Ok(())
}

/// Pauses the current track
pub async fn pause(player: &mut Player<'_, '_>) -> Result<()> {
    let proxy = player.get_proxy()?;
    proxy.method_call(INTERFACE, "Pause", ()).await?;

    Ok(())
}

/// Starts or resumes the current track
pub async fn play(player: &mut Player<'_, '_>) -> Result<()> {
    let proxy = player.get_proxy()?;
    proxy.method_call(INTERFACE, "Play", ()).await?;

    Ok(())
}

/// Resumes/starts or pauses the current track
pub async fn play_pause(player: &mut Player<'_, '_>) -> Result<()> {
    let proxy = player.get_proxy()?;
    proxy.method_call(INTERFACE, "PlayPause", ()).await?;

    Ok(())
}

/// Stops playback
pub async fn stop(player: &mut Player<'_, '_>) -> Result<()> {
    let proxy = player.get_proxy()?;
    proxy.method_call(INTERFACE, "Stop", ()).await?;

    Ok(())
}
