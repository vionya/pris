use empress::{get_connection, prop_cast, EventManager, EventType, Player};

#[tokio::test]
async fn test_comprehensive() -> Result<(), Box<dyn std::error::Error>> {
    let conn = get_connection();
    let mut player = Player::try_new("cmus", &conn).await?;
    let mut manager = EventManager::new(&conn);

    let _incoming = manager
        .add_callback(EventType::PropertiesChanged, |msg| {
            println!("Data: {:?}", msg);
            true
        })
        .await?;

    player.play_pause().await?;
    let metadata = player.get_metadata().await?;

    println!("{}", prop_cast::<String>(&metadata, "xesam:title").unwrap());
    tokio::signal::ctrl_c().await?;
    manager.clear_callbacks().await?;

    Ok(())
}
