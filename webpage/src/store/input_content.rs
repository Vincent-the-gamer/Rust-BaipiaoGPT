use core::messages;

use yewdux::store::Store;

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
