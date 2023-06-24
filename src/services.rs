use std::{error::Error};
use serde_json::{Value, json};
use crate::{models::Message, messages};
use crate::api_key;
use std::mem::size_of_val;


// 发请求和AI聊天
pub async fn chat(content: &str) -> Result<String, Box<dyn Error>> {
    // 请求头
    let headers = headermap![
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

    messages::insert_message(
        Message::new("user", content)
    );

    // 计算消息数组总token_length
    let token_length = cal_total_token_length();

    let json = json!({
        "messages": messages::get_messages(),
        "model": "gpt-3.5-turbo",
        "tokensLength": token_length
    });


    // 发请求
    let client = reqwest::Client::new();

    let api_key = api_key::get_api_key();
    
    let resp = client.post(
        format!("https://api.aigcfun.com/api/v1/text?key={}",api_key)
    )
        .headers(headers)
        .json(&json)
        .send()
        .await?
        .json::<Value>()
        .await;

    let return_value: Result<String, Box<dyn Error>> = match resp {
        Ok(resp) => {
            let message_is_null = &resp["choices"][0]["message"].is_null();
            // 当前的API Key已经到达上限时，重新请求
            if *message_is_null {
                messages::clear_message();
                request_api_key().await?;
                Ok(String::from("该API Key达到了使用上限, 正在生成新的API Key, 请过一会儿重新提问~"))
            } else {
                let result = &resp["choices"][0]["message"];
                messages::insert_message(result.clone().into());  
                let messages_vec = messages::get_messages().to_vec();
                Ok(String::from(
                    format!("{}", messages_vec[messages_vec.len() - 1].content.as_str())
                ))
            }
        },
        Err(err) => Err(Box::new(err))
    };

    return_value
}

/// 计算总tokenLength, 用于构造请求体
/// 判断字符是全角还是半角，全角返回2，半角返回1
fn cal_token_length(content: &str) -> usize{
    let mut count: usize = 0;
    let mut content = String::from(content);
     // 适配python unicodedata.east_asian_width的规则
    // 这个规则十分奇怪，会把某些全角符号判断为1
    content = content.replace("？", "?")
                     .replace("：", ":")
                     .replace("，", ",")
                     .replace("？", "?")
                     .replace("（", "(")
                     .replace("）", ")")
                     .replace("……", ".")
                     .replace("“", "\"")
                     .replace("；", ".")
                     .replace("！", "!")
                     .replace("￥", ".");
    for i in content.chars() {
        match size_of_val(&i.to_string()[..]) {
            1 => {
                count += 1;
            },
            _ => count += 2
        }
    }
    count
}

// 遍历所有消息的content，计算tokenLength总值
fn cal_total_token_length() -> usize{
    let mut all_messages = String::from("");

    if messages::get_messages().len() > 0 {
        for msg in messages::get_messages(){
            all_messages.push_str(&msg.content);
        }
    } else {
        panic!("消息数组为空！")
    }
    cal_token_length(&all_messages)
}


// 请求新的api key
pub async fn request_api_key() -> Result<(), Box<dyn Error>>{
    let headers = headermap![
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

    let client = reqwest::Client::new();
    let resp = client.get("https://api.aigcfun.com/fc/key")
        .headers(headers)
        .send()
        .await?
        .json::<Value>()
        .await?;

    api_key::set_api_key(
        resp["data"].as_str().unwrap()
    );

    let new_key = api_key::get_api_key();

    println!("新API Key: {}", new_key);

    // 验证
    verify_key(new_key).await?;
    Ok(())
}

// 验证api key
pub async fn verify_key(api_key: String) -> Result<(), Box<dyn Error>>{
    let headers = headermap![
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

    let client = reqwest::Client::new();
    client.get(
        format!("https://api.aigcfun.com/fc/verify-key?key={}", api_key)
    )
    .headers(headers)
    .send()
    .await?;

    Ok(())
}

// 清空上下文
pub fn clear_context(){
    messages::clear_message();
}
