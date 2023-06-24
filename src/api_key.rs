use std::sync::Mutex;
use lazy_static::lazy_static;

// 全局api_key
lazy_static! {
    static ref API_KEY: Mutex<String> = {
        let api_key = String::from("FCM0PV5F2NR72JQTB3");
        Mutex::new(api_key)
    };
}

pub fn get_api_key() -> String {
    API_KEY.lock().unwrap().to_string()
}

pub fn set_api_key(api_key: &str) {
    let mut key = API_KEY.lock().unwrap();
    *key = api_key.to_string();
}