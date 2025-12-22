use crate::routes;

use axum::response::Json;
use axum::routing::{get, post};
use axum::{middleware, Router};
use common::config::{server_config};
use database::entities::sys_user;
use database::repository::sys_user_repository;
use middleware_fn::request::{logging_middleware, rate_limiter};

pub fn build_router() -> Router {
    let mut router = Router::new();

    // 添加 Web 路由
    router = add_web_routes(router);

    router.layer(middleware::from_fn(rate_limiter)) // 整体限流
}

fn add_web_routes(mut router: Router) -> Router {
    let config = server_config();

    if config.ws_open {
        // ws服务和测试页面
        router = router.nest(&config.ws_path, routes::websocket::set_websocket_api());
    }

    if config.debug {
        //  测试模块
        router = router.nest(
            "/test",
            Router::new()
                // 获取参数 /{id}
                .route("/{id}", get(routes::case::args::sys_path_test))
                .route("/{name}/{age}", get(routes::case::args::sys_path_2_test))
                .route("/query", get(routes::case::args::sys_query_test))
                // header获取
                .route("/header", get(routes::case::args::sys_header_test))
                // 返回json
                .route("/json", get(routes::case::args::sys_response_json))
                // post json提交参数
                .route("/post-json", post(routes::case::args::sys_query_json))
                // post form提交参数
                .route("/post-form", post(routes::case::args::sys_query_form))
                // 整个组添加 中间件案例
                .layer(middleware::from_fn(logging_middleware)),
        );
    }

    router = router.route("/", get(index)).nest(
        "/api",
        Router::new().route("/", get(|| async { "Hello, Api!" })),
    );

    router
}

async fn index() -> Json<sys_user::Model> {
    let sys_user_data = sys_user_repository::get_by_id("2").await.unwrap();

    Json(sys_user_data)
}