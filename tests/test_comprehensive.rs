use pris::{self, EventManager, EventType, Player};

#[tokio::test]
async fn test_comprehensive() -> Result<(), Box<dyn std::error::Error>> {
    let conn = pris::get_connection();
    let mut player = Player::try_new("cmus", &conn).await?;
    let mut manager = EventManager::new(&conn);

    let _incoming = manager
        .add_callback(EventType::PropertiesChanged, |msg| {
            println!("Data: {:?}", msg);
            true
        })
        .await?;

    player.set_position(456).await?;
    let metadata = player.get_metadata().await?;

    println!(
        "{}",
        pris::prop_cast::<String>(&metadata, "xesam:title").unwrap()
    );
    tokio::signal::ctrl_c().await?;
    manager.clear_callbacks().await?;

    Ok(())
}
