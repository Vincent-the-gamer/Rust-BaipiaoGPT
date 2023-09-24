use core::{messages, models::Message, services::chat};

use yewdux::store::Store;

use super::dialog_message::DialogMessage;

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
                            .collect::<Vec<DialogMessage>>();
    }

    pub async fn request_dialog(&mut self, text: String) -> Result<String, String>{
        match chat(text.as_str()).await {
            Ok(result) => Ok(result),
            Err(err) => Err(err.to_string())
        }
    }
}