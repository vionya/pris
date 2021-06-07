use dbus::{
    message::{MatchRule, Message},
    strings::{Member, Path},
};
use dbus_tokio::connection;
use futures::StreamExt;
use std::pin::Pin;

fn callback(msg: Message) -> Pin<Box<dyn futures::Future<Output = ()>>> {
    println!("{:?}", msg);
    Box::pin(async {})
}

#[tokio::test]
async fn test_monitor() -> Result<(), Box<dyn std::error::Error>> {
    let (resource, conn) = connection::new_session_sync()?;

    tokio::spawn(async {
        let err = resource.await;
        panic!("Lost connection to D-Bus: {}", err);
    });

    let mut rule = MatchRule::new();
    rule.member = Some(Member::new("PropertiesChanged")?);
    rule.path = Some(Path::new("/org/mpris/MediaPlayer2")?);

    let (_incoming_signal, stream) = conn.add_match(rule).await?.msg_stream();
    let stream = stream.for_each(callback);
    futures::join!(stream);

    unreachable!()
    // Ok(())
}
