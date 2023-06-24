use std::collections::HashMap;

use actix_web::{get, post, Responder, HttpResponse, web};
use serde_json::json;
use crate::{services::{chat, clear_context}};

// 对话
#[post("/chat")]
pub async fn chat_with_ai(body: web::Json<HashMap<String, String>>) -> impl Responder {
    let json_body = json!(body);
    let content = json_body["content"].as_str().unwrap();
    let result = chat(content).await;
    let resp = match result {
        Ok(result) => result,
        Err(err) => format!("{}", err)
    };
   
    HttpResponse::Ok().json(resp)
}

// 清空上下文
#[get("/clearContext")]
pub async fn clear_ctx() -> impl Responder {
    clear_context();
    HttpResponse::Ok().json("已经成功清理上下文！")
}

