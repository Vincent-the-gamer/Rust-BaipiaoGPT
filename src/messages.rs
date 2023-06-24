use std::sync::Mutex;
use lazy_static::lazy_static;
use crate::models::Message;

// 全局消息
lazy_static! {
    static ref MESSAGES: Mutex<Vec<Message>> = {
        let messages: Vec<Message> = vec![
            Message::new("system", "请以markdown的形式返回答案")
        ];
        Mutex::new(messages)
    };
}

pub fn get_messages() -> Vec<Message>{
    MESSAGES.lock()
            .unwrap()
            .to_vec()
}

pub fn insert_message(message: Message) -> () {
    let mut messages = MESSAGES.lock().unwrap();
    messages.push(message);
}

pub fn clear_message() {
    let mut messages = MESSAGES.lock().unwrap();
    messages.clear();
    messages.push(Message::new("system", "请以markdown的形式返回答案"));
}