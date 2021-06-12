use pris::{self, Player};

#[tokio::test]
async fn test_init() -> Result<(), Box<dyn std::error::Error>> {
    let conn = pris::get_connection();
    let _player = Player::try_new("cmus", &conn).await?;

    Ok(())
}
