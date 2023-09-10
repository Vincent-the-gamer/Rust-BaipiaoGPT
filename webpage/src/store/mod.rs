use core::models::Message;

use yewdux::store::Store;

#[derive(PartialEq, Eq, Default, Store, Clone)]
pub struct InputContent{
    pub text: String
}

#[derive(PartialEq, Eq, Default, Store, Clone)]
pub struct DialogStore {
    pub messages: Vec<Message>,
    pub len: usize
}