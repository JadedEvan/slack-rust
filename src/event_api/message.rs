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
        hidden: bool,
        channel: String,
        ts: String,
        message: ChangedMessage,
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
pub struct ChangedMessage {
    #[serde(rename = "type")]
    _type: String,
    user: String,
    text: String,
    ts: String,
    edited: MessageChangedEdited,
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct MessageChangedEdited {
    user: String,
    ts: String,
}

#[cfg(test)]
mod test {
    use crate::event_api::event::*;
    use crate::event_api::message::*;

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

    #[test]
    fn deserializes_message_subtype_me_message() {
        let json = r##"
{
    "token": "tczRggnKN5seG9m70IPDIwkq",
    "team_id": "T1234567890",
    "api_app_id": "A0454AVL9L0",
    "event": {
	"type": "message",
	"subtype": "me_message",
	"channel": "C2147483705",
	"user": "U2147483697",
	"text": "is doing that thing",
	"ts": "1355517523.000005"
    },
    "type": "event_callback",
    "event_id": "Ev0464MAU18R",
    "event_time": 1665341482
}
"##;
        let event = serde_json::from_str::<Event>(json);
        assert_eq!(event.is_ok(), true);
    }

    #[test]
    fn deserializes_message_subtype_message_changed() {
        let json = r##"
{
    "token": "tczRggnKN5seG9m70IPDIwkq",
    "team_id": "T1234567890",
    "api_app_id": "A0454AVL9L0",
    "event": {
        "type": "message",
        "subtype": "message_changed",
        "hidden": true,
        "channel": "C2147483705",
        "ts": "1358878755.000001",
        "message": {
            "type": "message",
            "user": "U2147483697",
            "text": "Hello, world!",
            "ts": "1355517523.000005",
            "edited": {
                "user": "U2147483697",
                "ts": "1358878755.000001"
            }
        }
    },
    "type": "event_callback",
    "event_id": "Ev0464MAU18R",
    "event_time": 1665341482
}
"##;
        let event = serde_json::from_str::<Event>(json).unwrap();
        match event {
            Event::EventCallback(event_callback) => { 
                match event_callback.event {
                    EventCallbackType::Message(message) => {
                        match message {
                            MessageSubtype::MessageChanged{..} => assert!(true),
                            _ => assert!(false)
                        }
                    },
                    _ => panic!("Event callback deserialize into incorrect variant"),
                }
            },
        }
    }
}
