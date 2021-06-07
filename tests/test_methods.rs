use dbus_tokio::connection;
use empress::Player;

#[tokio::test]
async fn test_methods() -> Result<(), Box<dyn std::error::Error>> {
    let (resource, conn) = connection::new_session_sync()?;

    tokio::spawn(async {
        let err = resource.await;
        panic!("Lost connection to D-Bus: {}", err);
    });

    let mut player = Player::try_new("cmus", &conn).await?;
    player.play_pause().await;

    Ok(())
}
