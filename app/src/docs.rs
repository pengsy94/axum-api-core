#[cfg(feature = "openapi")]
use utoipa::{OpenApi, Modify};

#[cfg(feature = "openapi")]
use crate::api;

/// 项目 API 文档（utoipa）
///
/// 访问 `/api/openapi.json` 获取 OpenAPI 规范。
#[derive(OpenApi)]
#[openapi(
    info(
        title = "Axum Api Core",
        description = "基于 Axum 0.8 的轻量级 Web 服务框架",
        version = "0.1.0",
        license(name = "MIT"),
    ),
    paths(
        api::system::login,
    ),
    components(
        schemas(
            common::request::system::LoginRequest,
            common::response::login::LoginResponse,
            common::utils::response::ApiResponse<common::response::login::LoginResponse>,
        ),
    ),
    modifiers(&SecurityAddon),
)]
pub struct ApiDoc;

/// 为所有 API 添加 Bearer 认证
struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "bearer_auth",
                utoipa::openapi::security::SecurityScheme::Http(
                    utoipa::openapi::security::HttpBuilder::new()
                        .scheme(utoipa::openapi::security::HttpAuthScheme::Bearer)
                        .bearer_format("JWT")
                        .build(),
                ),
            );
        }

        let requirement =
            utoipa::openapi::security::SecurityRequirement::new::<&str, [&str; 0], &str>(
                "bearer_auth",
                [],
            );
        openapi.security = Some(vec![requirement]);
    }
}
