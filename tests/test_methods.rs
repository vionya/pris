use empress::{get_connection, prop_cast, Player};

#[tokio::test]
async fn test_methods() -> Result<(), Box<dyn std::error::Error>> {
    let conn = get_connection();
    let mut player = Player::try_new("cmus", &conn).await?;
    // player
    //     .seek_reverse(std::time::Duration::from_secs(15))
    //     .await?;

    let metadata = player.get_metadata().await?;
    let title: &String = prop_cast(&metadata, "xesam:title").unwrap();
    println!("{}", title);

    Ok(())
}
