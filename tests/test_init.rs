use empress::{self, Player};

#[tokio::test]
async fn test_init() -> Result<(), Box<dyn std::error::Error>> {
    let conn = empress::get_connection();
    let _player = Player::try_new("cmus", &conn).await?;

    Ok(())
}
