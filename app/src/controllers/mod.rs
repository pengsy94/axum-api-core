//! Controller 层 — 类似 Laravel 的 Controller
//!
//! 处理 HTTP 请求，调用 Service 层，返回 API 响应。
//!
//! # 目录约定
//!
//! ```text
//! app/src/controllers/
//!   mod.rs                   ← 本文件（模块声明 + Controller trait）
//!   user_controller.rs       ← 用户控制器
//! ```

pub mod user_controller;

/// Controller trait — 所有 Controller 的基础 trait
///
/// Controller 负责：
/// 1. 接收并校验请求参数
/// 2. 调用 Service 处理业务逻辑
/// 3. 使用 Resource 转换响应
/// 4. 返回 ApiResponse
pub trait Controller {
    /// 控制器名称（用于日志）
    fn name() -> &'static str;
}

/// 创建资源路由组
///
/// 类似 Laravel 的 `Route::prefix('/api/users')->group(...)`
#[macro_export]
macro_rules! controller_routes {
    // 入口：路由组带 prefix
    ($router:expr, $prefix:expr, $ctrl:ident, [$($method:ident),* $(,)?] $(, $middleware:expr)?) => {{
        use $ctrl as __ctrl;
        let __resource = controller_routes!(@build __ctrl $($method)*);
        $router.nest($prefix, __resource)
    }};

    // 构建 MethodRouter
    (@build $mod:ident) => { ::axum::Router::new() };
    (@build $mod:ident index $($rest:tt)*) => {{
        let __mr = controller_routes!(@build $mod $($rest)*);
        __mr.route("/", ::axum::routing::get($mod::index))
    }};
    (@build $mod:ident show $($rest:tt)*) => {{
        let __mr = controller_routes!(@build $mod $($rest)*);
        __mr.route("/{id}", ::axum::routing::get($mod::show))
    }};
    (@build $mod:ident create $($rest:tt)*) => {{
        let __mr = controller_routes!(@build $mod $($rest)*);
        __mr.route("/", ::axum::routing::post($mod::create))
    }};
    (@build $mod:ident update $($rest:tt)*) => {{
        let __mr = controller_routes!(@build $mod $($rest)*);
        __mr.route("/{id}", ::axum::routing::put($mod::update))
    }};
    (@build $mod:ident delete $($rest:tt)*) => {{
        let __mr = controller_routes!(@build $mod $($rest)*);
        __mr.route("/{id}", ::axum::routing::delete($mod::delete))
    }};
}

