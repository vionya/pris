use empress::{get_connection, Player};

#[tokio::test]
async fn test_init() -> Result<(), Box<dyn std::error::Error>> {
    let conn = get_connection();
    let _player = Player::try_new("cmus", &conn).await?;

    Ok(())
}
