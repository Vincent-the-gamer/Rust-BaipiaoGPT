use core::models::Message;

use yewdux::store::Store;

#[derive(PartialEq, Eq, Clone, Default, Store)]
pub struct DialogMessage {
    pub role: String,
    pub content: String
}

impl From<Message> for DialogMessage {
    fn from(msg: Message) -> Self {
        Self { role: msg.role, content: msg.content }
    }
}