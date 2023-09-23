use core::{models::Message, services::chat, messages};

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


#[derive(PartialEq, Eq, Default, Store, Clone)]
pub struct InputContent{
    pub text: String
}

impl InputContent {
    pub fn clear(&mut self) {
        self.text = String::from("");
        messages::clear_message();
    }
}

#[derive(PartialEq, Eq, Default, Store, Clone)]
pub struct DialogStore {
    pub messages: Vec<DialogMessage>,
}

impl DialogStore {
    pub fn insert_dialog(&mut self, dialog: DialogMessage){
        messages::insert_message(
            Message {
                role: dialog.role.to_owned(),
                content: dialog.content.to_owned()
            }
        );
        self.messages.push(dialog);
    }

    pub fn init_dialog(&mut self){
        self.messages = messages::get_messages().iter()
                            .map(|msg| DialogMessage::from(msg.to_owned()))
                            .collect();
    }

    pub async fn request_dialog(&mut self, text: String) -> Result<(), String>{
        // chat(text.as_str()).await.unwrap();

        messages::insert_message(
            Message::new("assistant", text.as_str())
        );
        Ok(())
    }
}