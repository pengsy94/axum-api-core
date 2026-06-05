use axum::Json;
use serde_json::{Value, json};

/// 用户列表
pub async fn index() -> Json<Value> {
    Json(json!([
        { "id": 1, "name": "李寻欢" },
        { "id": 2, "name": "阿飞" },
    ]))
}

/// 创建用户
pub async fn create() -> Json<Value> {
    Json(json!({ "id": 3, "name": "林诗音" }))
}

/// 用户详情
pub async fn show() -> Json<Value> {
    Json(json!({ "id": 1, "name": "李寻欢", "age": 30 }))
}

/// 更新用户
pub async fn update() -> Json<Value> {
    Json(json!({ "success": true }))
}

/// 删除用户
pub async fn delete() -> Json<Value> {
    Json(json!({ "success": true }))
}
