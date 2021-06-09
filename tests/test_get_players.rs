use empress;

#[tokio::test]
async fn test_get_players() -> Result<(), Box<dyn std::error::Error>> {
    let conn = empress::get_connection();
    let players = empress::get_all_players(&conn).await?;
    
    for mut player in players {
        player.play_pause().await;
        println!("{}", player.name);
    }

    Ok(())
}