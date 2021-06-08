use empress::{get_connection, EventManager, EventType};

#[tokio::test]
async fn test_evt_mgr() -> Result<(), Box<dyn std::error::Error>> {
    let conn = get_connection();
    let mut manager = EventManager::new(&conn);
    let _incoming = manager
        .add_callback(EventType::PropertiesChanged, |msg, (source,): (String,)| {
            println!("From: {:?}\nData: {:?}", source, msg);
            true
        })
        .await?;

    tokio::signal::ctrl_c().await?;
    manager.clear_callbacks().await?;

    Ok(())
}
