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

// 删除最后的两条消息
pub fn remove_last_two_messages() {
    let mut messages = MESSAGES.lock().unwrap();
    messages.resize(get_messages().len() - 2, Message::new("user", ""))
}

// 获取消息列表长度
pub fn len() -> usize {
    let messages = MESSAGES.lock().unwrap();
    messages.len()
}