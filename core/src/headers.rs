use std::sync::Mutex;
use lazy_static::lazy_static;
use reqwest::header::HeaderMap;
use crate::headermap;

// 全局请求头
lazy_static! {
    static ref HEADERS: Mutex<HeaderMap> = {
        let headermap = headermap![
            "Accept" => "application/json, text/plain, */*",
            "Accept-Encoding" => "gzip, deflate, br",
            "Accept-Language" => "zh-CN",
            "Access-Control-Allow-Origin" => "*",
            "Connection" => "keep-alive",
            "Content-Type" => "application/json",
            "Host" => "api.aigcfun.com",
            "sec-ch-ua" => "\"Not?A_Brand\";v=\"8\", \"Chromium\";v=\"108\"",
            "sec-ch-ua-mobile" => "?0",
            "sec-ch-ua-platform" => "\"Windows\"",
            "Sec-Fetch-Dest" => "empty",
            "Sec-Fetch-Mode" => "cors",
            "Sec-Fetch-Site" => "cross-site",
            "User-Agent" => "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) ai-edu/0.0.2 Chrome/108.0.5359.215 Electron/22.3.5 Safari/537.36",
            "x-f-platform" => "win32"
        ];
        Mutex::new(headermap)
    };
}

// getter
pub fn get_headers() -> HeaderMap {
    HEADERS.lock().unwrap().clone()
}