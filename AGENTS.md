# Axum Api Core - AGENTS

## 项目目标
使用 axum 0.8.8 搭建内存占用极少的 `web api` 基础服务。

## 🔴 硬规则 [CRITICAL]

- **默认使用中文沟通、说明和评审。**
- 修改代码前，先给出可选方案并等待确认。
- 未经确认，不进行实质性代码写入、删除、迁移、重构。
- 不得覆盖、回滚或污染用户已有未提交改动。
- 新发现的稳定约定，需要提示是否沉淀到公共规范。

## 技术约束
- 语言：Rust（Edition 2024）
- 框架：Axum 0.8.8
- ORM：SeaORM 2.0.0-rc.24

## 🏗️ 项目结构

```
bin/               入口，只组装不实现
  src/main.rs      启动入口，信号处理 + 优雅关闭
  src/bootstrap/   启动编排（配置→路由→DB→调度器→系统信息）

app/               路由注册 + API handler
  src/route.rs     全局路由定义（中间件/健康检查/404）
  src/api/         业务 handler（system/login, case/test）
  src/websocket/   WebSocket 连接管理
  src/docs.rs      OpenAPI ApiDoc 定义（feature = "openapi"）

common/            共享类型
  src/error.rs     AppError 统一错误枚举
  src/auth.rs      AuthUser 结构体
  src/request/     请求体类型（LoginRequest）
  src/response/    响应体类型（LoginResponse）
  src/validator/   参数校验 extractor（ValidatedJson/Form/Path/Query）
  src/utils/       ApiResponse 响应封装

middleware/        中间件（原名 middleware-fn）
  src/request.rs   logging / rate_limiter / trace_middleware
  src/auth.rs      auth_middleware（骨架，替换 validate_token 即可投产）

kernel/            基础设施
  src/config/      配置加载 + 校验（OnceLock + 环境变量）
  src/tasks/       定时任务调度器（tokio-cron-scheduler）
  src/system.rs    系统信息打印

database/          数据库层
  src/entity/      SeaORM 自动生成实体
  src/repository/  仓库层（sys_user_repository）
```

## 📐 核心约定

### 1. JSON 响应格式

所有 API 统一返回此结构：

```json
{
  "code": 200,
  "data": { ... },       // 成功时有值，失败时为 null（始终存在）
  "message": "success",
  "errors": null         // 仅字段级校验失败时携带
}
```

成功：`ApiResponse::success(data)` — code=200, message="success"
错误：handler 返回 `Result<_, AppError>`，自动转换

### 2. 错误处理

使用 `common::error::AppError` 枚举：

| 变体 | HTTP 状态 | 用途 |
|------|-----------|------|
| `Unauthorized(msg)` | 401 | 未登录/token 无效 |
| `Forbidden(msg)` | 403 | 无权限 |
| `NotFound(msg)` | 404 | 资源不存在 |
| `BadRequest(msg)` | 400 | 参数错误 |
| `Conflict(msg)` | 409 | 冲突 |
| `Internal(msg)` | 500 | 内部错误 |

Handler 中 `?` 自动转换 `anyhow::Error` → `AppError::Internal`。

### 3. 配置管理

所有配置通过**环境变量**加载（支持 `.env` 文件），启动时自动校验。

- 全局单例 `OnceLock<AppConfig>`，通过 `server_config()` / `database_config()` 获取
- 新增配置项需要在 `ServerConfig` / `DatabaseConfig` 中同时添加字段 + `from_env()` + `validate()`
- 完整文档见 `CONFIG.md`

### 4. 数据库（可选）

`DATABASE_URL` 为空时跳过数据库初始化，服务仍可正常启动。

- 未配置时：`/ready` → `{"status":"ready","database":"disabled"}`
- 配置时：`/ready` 校验连接池存活

### 5. 中间件顺序（从外到内）

```
trace_middleware（请求追踪, 最外层）
  → logging_middleware（请求日志, 条件启用）
    → rate_limiter（限流, 条件启用）
```

### 6. 可选的 openapi 功能

编译时通过 `--features openapi` 启用：
- `GET /api/openapi.json` — OpenAPI 规范
- `GET /docs` — Swagger UI 可视化页面

feature gate 通过 `#[cfg(feature = "openapi")]` 控制，不影响核心编译。

### 7. 日志规范

- 使用 `tracing::info/debug/warn/error`，禁止 `println!`（已修复）
- 日志级别：`TRACE` / `DEBUG` / `INFO` / `WARN` / `ERROR`
- 每个请求自动携带 `trace_id`（UUID），支持日志串联

## 📁 关键文件定位

| 需求 | 文件 |
|------|------|
| 添加新 API 路由 | `app/src/route.rs` → `add_api_routes()` |
| 添加新 handler | `app/src/api/` 下新建模块 |
| 添加新中间件 | `middleware/src/request.rs` 或 `auth.rs` |
| 添加配置项 | `kernel/src/config/server_config.rs` + `.env.example` + `CONFIG.md` |
| 添加数据库实体 | 用 `sea-orm-cli generate entity` 生成到 `database/src/entity/` |
| 添加 repository | `database/src/repository/` 下新建 |
| 添加 OpenAPI 文档 | 为 handler 加 `#[cfg_attr(feature = "openapi", utoipa::path(...))]` |
