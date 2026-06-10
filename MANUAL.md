# Axum Api Core 使用手册

## 1. 项目定位

`axum-api-core` 是一个基于 `Axum 0.8.8`、采用 Laravel 风格分层的 Rust Web API 基础工程。

核心目标：

- 提供可直接启动的 API 服务骨架
- 保持清晰的分层：`Router -> Controller -> Service -> Model -> Resource`
- 支持数据库可选、Redis 可选、WebSocket 可选、定时任务可选
- 统一业务响应结构，便于前后端对接

---

## 2. 环境准备

建议准备以下环境：

- Rust stable 工具链
- Cargo
- 可选数据库：MySQL / PostgreSQL / SQLite
- 可选 Redis

如果您只想先把服务跑起来，不配置数据库和 Redis 也可以。

---

## 3. 快速开始

### 3.1 复制配置

```bash
cp .env.example .env
```

至少建议先确认这些变量：

```env
SERVER_HOST=0.0.0.0
SERVER_PORT=3000
JWT_SECRET=change-me
DATABASE_URL=
REDIS_URL=
```

### 3.2 启动服务

```bash
cargo run -p bin
```

启动成功后，默认可访问：

- `http://127.0.0.1:3000/`
- `http://127.0.0.1:3000/health`
- `http://127.0.0.1:3000/ready`

### 3.3 快速验证

```bash
curl http://127.0.0.1:3000/
curl http://127.0.0.1:3000/health
curl http://127.0.0.1:3000/ready
```

---

## 4. 配置说明

完整配置请看 [CONFIG.md](./CONFIG.md)。

这里先给出最常用的几项：

| 变量 | 说明 |
|------|------|
| `APP_ENV` | 环境名，会自动尝试加载 `.env.{APP_ENV}` |
| `DEBUG` | 是否挂载 `/test` 测试路由 |
| `SERVER_HOST` | 服务监听地址 |
| `SERVER_PORT` | 服务端口 |
| `SERVER_CONTENT_GZIP` | 是否开启 Gzip |
| `SERVER_CRON` | 是否开启定时任务 |
| `SERVER_WS_OPEN` | 是否开启 WebSocket |
| `SERVER_WS_PATH` | WebSocket 路径，默认 `/ws` |
| `SERVER_RATE_LIMIT_ENABLED` | 是否开启限流 |
| `CORS_ALLOWED_ORIGINS` | 允许的跨域来源，支持 `*` 或逗号分隔 |
| `LOG_LEVEL` | 日志级别 |
| `DATABASE_URL` | 数据库连接串，留空可禁用数据库 |
| `REDIS_URL` | Redis 连接串，留空可禁用 Redis |

---

## 5. 运行行为说明

### 5.1 数据库与 Redis 都是可选的

- `DATABASE_URL` 为空时，服务会跳过数据库初始化
- `REDIS_URL` 为空时，服务会跳过 Redis 初始化
- 即使数据库或 Redis 初始化失败，服务仍会继续尝试启动

### 5.2 `/health` 与 `/ready` 的区别

- `/health`：只表示服务进程存活，通常返回 `200`
- `/ready`：表示服务是否具备对外提供能力

当前 `/ready` 的行为：

- 数据库未启用：返回 `{"status":"ready","database":"disabled"}`
- 数据库已启用且连接可用：`database = "connected"`
- 数据库已启用但连接不可用：返回 HTTP `503`，并标记 `not_ready`
- Redis 未启用：`redis = "disabled"`
- Redis 已启用但连接不可用：`redis = "disconnected"`

注意：

- 当前实现里，Redis 断开不会阻断 `/ready`
- 数据库断开会直接让 `/ready` 返回 `503`

### 5.3 定时任务启动策略

- 当 `SERVER_CRON=true` 时，服务会尝试启动调度器
- 如果调度器启动失败，当前实现会记录告警，但服务本身继续运行

### 5.4 请求日志格式

`logging_middleware` 会把请求和响应拆成两个块状段落输出：

- `HTTP REQUEST`：包含 `trace_id`、`method`、`uri`、核心 headers
- `HTTP RESPONSE`：包含 `trace_id`、`status`、耗时、body 摘要

这套格式在终端和日志文件中保持一致，方便肉眼快速定位一次请求的完整链路。

---

## 6. 响应结构与状态语义

### 6.1 统一 JSON 结构

业务 API 统一返回：

```json
{
  "code": 200,
  "data": {},
  "message": "success",
  "errors": null
}
```

### 6.2 结果判断规则

这个项目里，业务结果请优先看响应体中的 `code`：

- `code = 200` 表示业务成功
- `code = 400/401/403/404/409/500` 表示业务失败或校验失败

HTTP 状态在这个项目中的角色更偏向于：

- 连接层是否通
- 基础设施路由是否可用
- 是否是框架级异常情况

因此：

- 业务接口即使失败，也可能仍返回 HTTP `200`
- 具体业务是否成功，应以 `body.code` 为准
- 像 `/ready`、未知路由 `404` 这类基础设施路由，仍可能使用语义化 HTTP 状态

### 6.3 字段级错误

当参数校验失败时，`errors` 字段会带出字段级错误列表，例如：

```json
{
  "code": 400,
  "data": null,
  "message": "Query 参数校验失败",
  "errors": [
    {
      "field": "page",
      "message": "range"
    }
  ]
}
```

---

## 7. 常用接口

| 端点 | 方法 | 说明 |
|------|------|------|
| `/` | GET / POST | 欢迎页 |
| `/health` | GET | 存活检查 |
| `/ready` | GET | 就绪检查 |
| `/metrics` | GET | Prometheus 指标 |
| `/api/login` | POST | 演示登录 |
| `/api/users` | GET | 用户列表 |
| `/api/users` | POST | 创建用户占位接口 |
| `/api/users/{id}` | GET | 用户详情 |
| `/api/users/{id}` | PUT | 更新用户占位接口 |
| `/api/users/{id}` | DELETE | 删除用户 |
| `/api/openapi.json` | GET | OpenAPI JSON 规范，需启用 `app/openapi` |
| `/docs` | GET | RapiDoc 可视化文档，需启用 `app/openapi` |
| `/swagger` | GET | Swagger UI，需启用 `app/openapi` |
| `/test/*` | 多种 | 调试测试路由，需 `DEBUG=true` |
| `/ws` | WS | WebSocket，需 `SERVER_WS_OPEN=true` |

### 7.1 登录接口

演示账号固定为：

- 邮箱：`admin@example.com`
- 密码：`admin123`

示例：

```bash
curl -X POST http://127.0.0.1:3000/api/login \
  -H 'Content-Type: application/json' \
  -d '{
    "email": "admin@example.com",
    "password": "admin123"
  }'
```

### 7.2 用户接口说明

当前 `UserController` 已接入分层链路：

- `GET /api/users`
- `GET /api/users/{id}`
- `DELETE /api/users/{id}`

`POST /api/users` 和 `PUT /api/users/{id}` 目前仍是占位接口，返回 TODO 风格结果，用于保留资源路由结构。

---

## 8. 数据库与迁移

### 8.1 启用数据库

只要配置 `DATABASE_URL`，服务启动时就会尝试建立连接池。

示例：

```env
DATABASE_URL=mysql://root:password@127.0.0.1:3306/database
```

### 8.2 迁移命令

```bash
cargo run -p migration up
cargo run -p migration down
cargo run -p migration fresh
cargo run -p migration status
```

### 8.3 生成实体

如果您使用现有数据库表结构，可以用 `sea-orm-cli` 生成实体：

```bash
cargo install sea-orm-cli
sea-orm-cli generate entity \
  -u mysql://root:password@127.0.0.1:3306/database \
  --with-serde both \
  -o database/src/entity
```

---

## 9. Redis 使用

Redis 启用方式：

```env
REDIS_URL=redis://127.0.0.1:6379
REDIS_POOL_SIZE=4
```

代码中可直接使用 `database::Cache`：

```rust
use database::Cache;

Cache::set("key", "value").await?;
let value = Cache::get("key").await?;
```

---

## 10. Artisan 脚手架

### 10.1 创建 Controller

```bash
cargo run -p artisan -- make:controller User
```

### 10.2 创建 Service

```bash
cargo run -p artisan -- make:service Order
```

### 10.3 创建 Resource

```bash
cargo run -p artisan -- make:resource Order
```

### 10.4 创建 Model 占位

```bash
cargo run -p artisan -- make:model Category
```

生成器会自动把新文件注册到对应 `mod.rs`。

当前生成约定：

- Controller 优先使用 `Result<ApiResponse<_>, AppError>`
- 查询/详情/删除接口按分层方式调用 Service
- 资源输出走 `JsonResource`

---

## 11. 开发流程建议

一个标准新增资源的流程通常是：

1. 先建表并执行 migration
2. 生成或编写 `entity`
3. 编写 `Service`
4. 编写 `Controller`
5. 如有需要，定义 `Resource`
6. 在 `app/src/route.rs` 注册资源路由
7. 补测试并执行 `cargo check` / `cargo test`

常用命令：

```bash
cargo check
cargo test -p app --test api
```

---

## 12. 项目结构速览

```text
bin/          应用入口与启动编排
app/          路由、Controller、Service、WebSocket
common/       错误、鉴权、请求校验、统一响应、资源层
middleware/   请求日志、限流、认证等中间件
database/     Entity、Model 门面、Redis、旧仓库层
kernel/       配置、系统信息、定时任务
migration/    数据库迁移
artisan/      代码脚手架
```

---

## 13. 补充说明

### 13.1 OpenAPI / 可视化 API 文档

当前代码库里，启用 OpenAPI feature 的命令：

```bash
cargo run -p bin --features app/openapi
```

启用后可访问：

- `/api/openapi.json`：OpenAPI 原始规范
- `/docs`：默认的 RapiDoc 可视化文档页
- `/swagger`：备用 Swagger UI 页面

说明：

- `RapiDoc` 更现代，也更适合阅读和对外展示
- `Swagger UI` 更适合调试和直接试请求

### 13.2 认证中间件已提供，但默认未全局挂载

仓库里已经有 JWT 认证中间件实现，但当前并没有默认把它挂到全部业务路由上。

如果您要启用鉴权，需要在路由层显式挂载中间件。

---

## 14. 参考文档

- 项目入口说明：[README.md](./README.md)
- 配置文档：[CONFIG.md](./CONFIG.md)
- 请求示例：[test-request.http](./test-request.http)
