use std::collections::HashMap;
use actix_web::{get, post, Responder, HttpResponse, web};
use serde_json::json;
use crate::services::{chat, clear_context};
use crate::messages::{ get_messages, remove_last_two_messages };

// 对话
#[post("/chat")]
pub async fn chat_with_ai(body: web::Json<HashMap<String, String>>) -> impl Responder {
    let json_body = json!(body);
    let content = json_body["content"].as_str().unwrap();
    let result = send_message(content).await;

    HttpResponse::Ok().body(result)
}

// 清空上下文
#[get("/clearContext")]
pub async fn clear_ctx() -> impl Responder {
    clear_context();
    HttpResponse::Ok().body("已经成功清理上下文！")
}

// 获得当前上下文条数
#[get("/showContextCount")]
pub async fn show_context_count() -> impl Responder {
    HttpResponse::Ok().body(
        format!("当前上下文条数：{}",get_messages().len())
    )
}

// 重新生成答案
#[get("/regenerate")]
pub async fn regenerate() -> impl Responder {
    let messages = get_messages();
    if messages.len() <= 1 {
        HttpResponse::Ok().body("消息为空")
    } else {
        // 拿到最后一个问题
        let last_question = &messages[messages.len() - 2].content;
        // 删除最后两条问答
        remove_last_two_messages();
        // 重新提问
        let result = send_message(&last_question[..]).await;
        HttpResponse::Ok().body(result)
    }
}

// 发送消息
async fn send_message(content: &str) -> String{
   chat(content).await.expect("AI对话请求出幺蛾子啦！")
}