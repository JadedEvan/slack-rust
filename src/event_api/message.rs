use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[derive(Deserialize, Serialize, Debug, PartialEq)]
#[serde(rename_all = "snake_case", tag = "subtype")]
pub enum MessageSubtype {
    BotMessage {
        bot_id: String,
        text: String,
        ts: String,
        username: String,
    },
    MeMessage,
    MessageChanged {
        edited: MessageChangedEdited,
        text: String,
        ts: String,
        user: String,
    },
    Message {
        channel: String,
        channel_type: String,
        event_ts: String,
        hidden: Option<bool>,
        text: String,
        thread_ts: Option<String>,
        ts: String,
        user: Option<String>,
    },
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct MessageChangedEdited {
    user: String,
    ts: String,
}

#[cfg(test)]
mod test {
    use crate::event_api::event::Event;
    #[test]
    fn deserializes_message_subtype_bot_message() {
        let json = r##"
{
    "token": "tczRggnKN5seG9m70IPDIwkq",
    "team_id": "T1234567890",
    "api_app_id": "A0454AVL9L0",
    "event": {
        "type": "message",
        "subtype": "bot_message",
        "text": "this is a bot event",
        "ts": "1665341482.399349",
        "username": "Robot",
        "bot_id": "B04488SU0P8",
        "app_id": "A044TG2QKDF",
        "channel": "C044T73TB17",
        "event_ts": "1665341482.399349",
        "channel_type": "channel"
    },
    "type": "event_callback",
    "event_id": "Ev0464MAU18R",
    "event_time": 1665341482
}
"##;
        let event = serde_json::from_str::<Event>(json);
        assert_eq!(event.is_ok(), true);
    }
}
