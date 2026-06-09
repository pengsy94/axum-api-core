//! 用户控制器
//!
//! 处理 `/api/users` 下的所有请求。
//!
//! # RESTful 路由
//! | 方法 | 路径 | Handler |
//! |------|------|---------|
//! | GET  | /api/users     | index()   |
//! | GET  | /api/users/{id}| show()    |
//! | POST | /api/users     | create()  |
//! | PUT  | /api/users/{id}| update()  |
//! | DELETE| /api/users/{id}| delete() |

use axum::extract::Path;
use common::resources::JsonResource;
use common::utils::pagination::PageParams;
use common::utils::response::ApiResponse;
use common::validator::query::ValidatedQuery;
use database::entity::sys_user;
use serde_json::json;

use super::Controller;
use crate::services::user_service::UserService;

// ========================================
// UserResource
// ========================================

/// 用户资源 — 控制 Model 的 JSON 输出格式
pub struct UserResource {
    user: sys_user::Model,
}

impl JsonResource for UserResource {
    type Source = sys_user::Model;

    fn from_source(source: Self::Source) -> Self {
        Self { user: source }
    }

    fn to_array(&self) -> serde_json::Value {
        json!({
            "id": self.user.id,
            "name": self.user.name,
        })
    }
}

// ========================================
// UserController
// ========================================

/// 用户控制器
///
/// 每个 handler 按需创建 Service 实例（与 Laravel 容器按请求解析一致）。
pub struct UserController;

impl Controller for UserController {
    fn name() -> &'static str {
        "UserController"
    }
}

// ========================================
// Handler 方法
// ========================================

impl UserController {
    /// 用户列表（分页）
    ///
    /// `GET /api/users?page=1&page_size=20`
    pub async fn index(
        ValidatedQuery(params): ValidatedQuery<PageParams>,
    ) -> ApiResponse<serde_json::Value> {
        let service = UserService::new();
        match service.list(&params).await {
            Ok(paginated) => {
                // 用 Resource 转换每条记录
                let items: Vec<serde_json::Value> = paginated
                    .items
                    .into_iter()
                    .map(|m| UserResource::from_source(m).to_array())
                    .collect();
                ApiResponse::success(json!({
                    "items": items,
                    "total": paginated.total,
                    "page": paginated.page,
                    "page_size": paginated.page_size,
                    "total_pages": paginated.total_pages,
                }))
            }
            Err(e) => ApiResponse::error(500, &e.to_string()),
        }
    }

    /// 用户详情
    ///
    /// `GET /api/users/{id}`
    pub async fn show(Path(id): Path<i32>) -> ApiResponse<serde_json::Value> {
        let service = UserService::new();
        match service.show(id).await {
            Ok(Some(user)) => UserResource::make(user).respond(),
            Ok(None) => ApiResponse::error(404, "用户不存在"),
            Err(e) => ApiResponse::error(500, &e.to_string()),
        }
    }

    /// 创建用户（示例，需要 FormRequest 配合）
    ///
    /// `POST /api/users`
    pub async fn create() -> ApiResponse<serde_json::Value> {
        // TODO: 与 FormRequest 集成后实现
        ApiResponse::success(json!({ "message": "创建接口待实现" }))
    }

    /// 更新用户
    ///
    /// `PUT /api/users/{id}`
    pub async fn update(Path(id): Path<i32>) -> ApiResponse<serde_json::Value> {
        // TODO: 与 FormRequest 集成后实现
        ApiResponse::success(json!({ "message": format!("更新接口待实现, id={}", id) }))
    }

    /// 删除用户
    ///
    /// `DELETE /api/users/{id}`
    pub async fn delete(Path(id): Path<i32>) -> ApiResponse<serde_json::Value> {
        let service = UserService::new();
        match service.destroy(id).await {
            Ok(()) => ApiResponse::success(json!({ "deleted": true })),
            Err(e) => ApiResponse::error(500, &e.to_string()),
        }
    }
}
