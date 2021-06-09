use empress::{get_connection, EventManager, EventType, Message};

// A callback can be a detached function...
fn callback(msg: Message) -> bool {
    println!("Seeked\nData: {:?}", msg);
    true
}

#[tokio::test]
async fn test_evt_mgr() -> Result<(), Box<dyn std::error::Error>> {
    let conn = get_connection();
    let mut manager = EventManager::new(&conn);

    let _incoming_props = manager
        .add_callback(EventType::PropertiesChanged, |msg| {
            // ... or a callback can be a closure
            println!("PropertiesChanged\nData: {:?}", msg);
            true
        })
        .await?;
    let _incoming_seeked = manager.add_callback(EventType::Seeked, callback).await?;

    tokio::signal::ctrl_c().await?;
    manager.clear_callbacks().await?;

    Ok(())
}
