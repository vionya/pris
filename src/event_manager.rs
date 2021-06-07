use dbus::{
    arg::ReadAll,
    channel::Token,
    message::{MatchRule, Message},
    nonblock::{MsgMatch, SyncConnection},
    strings::{Member, Path},
};
use std::error::Error;

type EmptyResult = std::result::Result<(), Box<dyn Error>>;

/// Enum for indicating which type of MPRIS event to listen
/// for.
pub enum EventType {
    /// Emitted whenever properties change.
    /// A list of all properties that will
    /// cause this to be emitted can be found
    /// [here]
    /// 
    /// [here]: https://specifications.freedesktop.org/mpris-spec/latest/Player_Interface.html#Property:PlaybackStatus
    PropertiesChanged,
    /// Emitted whenever the active track is seeked.
    Seeked,
}

/// A struct that simplifies the process of adding
/// and removing listeners and callbacks to/from MPRIS's
/// `DBus` signals.
pub struct EventManager<'a> {
    conn: &'a SyncConnection,
    callback_tokens: Vec<Token>,
}

impl EventManager<'_> {
    /// Creates a new event manager.
    pub fn new(conn: &SyncConnection) -> EventManager {
        EventManager {
            conn,
            callback_tokens: Vec::new(),
        }
    }

    /// Adds a new callback to the event manager.
    ///
    /// # Errors
    /// Returns an `Err` if there is a failure in adding
    /// a match rule to the connection.
    /// 
    /// # Example
    /// ```
    /// let mut manager = EventManager::new(&connection);
    /// // Be advised that it is important that this is assigned to a variable
    /// let _incoming = manager
    ///     .add_callback(EventType::PropertiesChanged, |msg, (source,): (String,)| {
    ///         println!("From: {:?}\nData: {:?}", source, msg);
    ///         true
    ///     })
    ///     .await?;
    /// ```
    pub async fn add_callback<R: 'static>(
        &mut self,
        event_type: EventType,
        callback: fn(Message, R) -> bool,
    ) -> Result<MsgMatch, Box<dyn Error>>
    where
        R: ReadAll,
    {
        let mut rule = MatchRule::new();
        rule.member = Some(Member::new(match event_type {
            EventType::PropertiesChanged => "PropertiesChanged",
            EventType::Seeked => "Seeked",
        })?);
        rule.path = Some(Path::new("/org/mpris/MediaPlayer2")?);

        let msg_match = self.conn.add_match(rule).await?;
        let registered_callback = msg_match.cb(callback);
        self.callback_tokens.push(registered_callback.token());

        Ok(registered_callback)
    }

    /// Clears all registered callbacks from the manager.
    /// 
    /// # Errors
    /// Returns an `Err` if there is a failure in removing
    /// a match from the connection.
    pub async fn clear_callbacks(&mut self) -> EmptyResult {
        for token in &self.callback_tokens {
            self.conn.remove_match(*token).await?;
        }

        Ok(())
    }
}
