# Axum Api Core

基于 **Axum 0.8.8** + **Laravel 风格分层**的轻量级 Web 服务基础框架。内存占用极少，适合低配云服务器部署。

## ✨ 特性

- ⚡ **Axum 0.8.8** — 高性能异步 HTTP 框架
- 🧱 **Laravel 风格分层** — Controller → Service → Model(ORM) → Resource
- 🧱 **模块化 workspace** — 8 个 crate 职责清晰
- 📦 **数据库可选** — 不配 `DATABASE_URL` 也能跑
- ⚡ **Redis 缓存** — 可选，一行代码读写缓存
- 🔐 **JWT + argon2** — 真实 JWT 签发验证 + 密码哈希
- 📋 **请求追踪** — 每个请求自动注入 `X-Trace-Id` 关联日志
- 🚦 **限流 / 超时 / CORS / Gzip** — 全部可配置
- 📊 **健康检查** — `/health`（存活）+ `/ready`（就绪）
- 📈 **Prometheus 指标** — `GET /metrics` 暴露请求计数与延迟
- 📚 **OpenAPI 文档** — `cargo run --features openapi` 一键生成 Swagger
- 🔌 **WebSocket** — 私聊 / 广播 / 在线列表
- ⏰ **定时任务** — tokio-cron-scheduler
- 🗂️ **分页工具** — 通用 `PageParams` + `Paginated<T>` 响应
- 📎 **文件上传** — Multipart 处理 + 自动命名存储
- 🗑️ **软删除 / 时间戳** — 可复用的 `SoftDelete` / `TimeStamp` trait
- 🛠️ **Artisan CLI** — `cargo run -p artisan make:controller User` 秒建 CRUD
- 📦 **数据库迁移** — 内置 SeaORM Migration CLI
- 🐳 **Docker 支持** — 多阶段构建 + docker-compose

## 🚀 快速开始

```bash
# 1. 复制环境变量
cp .env.example .env

# 2. 运行（数据库可选，不配 DATABASE_URL 也能启动）
cargo run

# 3. 访问
curl http://localhost:3000
# → Welcome to Axum Api Core!
```

## 📡 API 端点

| 端点 | 方法 | 说明 | 条件 |
|------|------|------|------|
| `/` | GET/POST | 欢迎页面 | |
| `/health` | GET | 存活检查 | |
| `/ready` | GET | 就绪检查（含 DB 状态） | |
| `/metrics` | GET | Prometheus 指标 | |
| `/api/login` | POST | JWT 登录（admin@example.com / admin123） | |
| `/api/users` | GET | 用户列表（Controller → Service → Model → Resource） | `debug_assertions` |
| `/api/users/{id}` | GET | 用户详情 | `debug_assertions` |
| `/api/users/{id}` | DELETE | 删除用户 | `debug_assertions` |
| `/api/openapi.json` | GET | OpenAPI 规范 | `--features openapi` |
| `/docs` | GET | Swagger UI | `--features openapi` |
| `/test/*` | - | 测试路由 | `DEBUG=true` |
| `/ws` | WS | WebSocket | `SERVER_WS_OPEN=true` |

## 🏗️ 项目结构

```
bin/                入口，只组装不实现
artisan/            Artisan CLI（cargo run -p artisan make:controller ...）
app/                路由注册 + Controller + Service + 宏
  src/route.rs           全局路由定义（中间件/健康检查/404）
  src/controllers/       Laravel 风格 Controller（user_controller.rs）
  src/services/          Service 层（user_service.rs）
  src/api/               旧版 handler（system/login, case/test）
  src/websocket/         WebSocket 连接管理
  src/macros.rs          controller_routes! / resources! 宏

common/             共享类型
  src/error.rs           AppError 统一错误枚举
  src/auth.rs            AuthUser + JWT + 密码哈希
  src/request/           请求体类型 + FormRequest trait
  src/response/          响应体类型
  src/resources/         API Resource 层（JsonResource trait）
  src/validator/         参数校验 extractor（ValidatedJson/Form/Path/Query）
  src/utils/             ApiResponse 响应封装 + 分页 + 软删除

middleware/          中间件
  src/request.rs        logging / rate_limiter / trace_middleware
  src/auth.rs           认证中间件

kernel/              基础设施
  src/config/           配置加载 + 校验（OnceLock + 环境变量）
  src/tasks/            定时任务调度器（tokio-cron-scheduler）

database/            数据库层
  src/entity/           SeaORM 自动生成实体
  src/model/            Eloquent 风格 Model 门面（find/create/update/delete/query/paginate）
  src/repository/       旧版仓库层
  src/redis/            Redis 缓存

migration/           数据库迁移（SeaORM Migration）
```

## ⚙️ 配置

通过环境变量加载（支持 `.env` + `.env.{APP_ENV}` 多环境），启动时自动校验。

```env
# 环境
APP_ENV=local                     # 自动加载 .env.local → .env

# 服务器
SERVER_HOST=0.0.0.0
SERVER_PORT=3000
SERVER_REQUEST_TIMEOUT=30         # 秒，0=不超时

# JWT
JWT_SECRET=change-me-to-random

# 日志
LOG_LEVEL=INFO
LOG_FORMAT=text                   # 或 json

# 数据库（可选，留空即不启用）
DATABASE_URL=
```

完整配置清单见 [`CONFIG.md`](CONFIG.md)。

## 💾 数据库

使用 **SeaORM 2.0**，支持 MySQL / PostgreSQL / SQLite。**数据库为可选。**

```bash
# 运行迁移
cargo run -p migration up

# 回滚
cargo run -p migration down

# 用 sea-orm-cli 从现有数据库生成实体
cargo install sea-orm-cli
sea-orm-cli generate entity \
  -u mysql://root:password@localhost:3306/database \
  --with-serde both \
  -o database/src/entity
```

## 🏗️ Laravel 风格分层

### Model（Eloquent 风格）

```rust
use database::model::Model;
use database::entity::sys_user;

type User = Model<sys_user::Entity>;

// 静态查询
let user = User::find(1).await?;
let all = User::all().await?;
let paginated = User::paginate(&params).await?;
let count = User::count().await?;

// 流式查询（Query Builder）
let users = User::query()
    .filter_eq(sys_user::Column::Name, "张三")
    .filter_like(sys_user::Column::Name, "%李%")
    .limit(10)
    .all()
    .await?;

// 写入
let new_user = User::create(|m| {
    m.name = Set("李寻欢".to_owned().into());
}).await?;

User::update_by_id(1, |m| {
    m.name = Set("新名字".to_owned().into());
}).await?;

User::delete_by_id(1).await?;
```

### Service 层

```rust
// app/src/services/user_service.rs
pub struct UserService;

impl UserService {
    pub async fn list(&self, params: &PageParams) -> Result<Paginated<sys_user::Model>, AppError> {
        UserModel::paginate(params).await
    }

    pub async fn show(&self, id: i32) -> Result<Option<sys_user::Model>, AppError> {
        UserModel::find(id).await
    }
}
```

### Controller + Resource

```rust
// app/src/controllers/user_controller.rs
impl UserController {
    pub async fn index(
        ValidatedQuery(params): ValidatedQuery<PageParams>,
    ) -> ApiResponse<serde_json::Value> {
        let service = UserService::new();
        match service.list(&params).await {
            Ok(paginated) => {
                let items: Vec<_> = paginated.items
                    .into_iter()
                    .map(|m| UserResource::from_source(m).to_array())
                    .collect();
                ApiResponse::success(json!({
                    "items": items, "total": paginated.total,
                    "page": paginated.page, "page_size": paginated.page_size,
                }))
            }
            Err(e) => ApiResponse::error(500, &e.to_string()),
        }
    }
}
```

### 路由注册

```rust
// app/src/route.rs
router = controller_routes!(
    router,
    "/api/users",
    UserController,
    [index, show, create, update, delete]
);
```

## 🛠️ Artisan CLI

```bash
# 创建 Controller（含 CRUD 骨架）
cargo run -p artisan make:controller Product

# 创建 Service
cargo run -p artisan make:service Order

# 创建 Resource
cargo run -p artisan make:resource Order

# 创建 Model（Entity + Repository 占位）
cargo run -p artisan make:model Category
```

生成的文件自动注册 `mod`，开箱即用。

旧版 `scripts/make.sh` 仍然可用，但推荐使用 Artisan CLI。

## ⚡ Redis 缓存

可选，`REDIS_URL` 留空时跳过初始化。

```bash
# 配置（格式：redis://[:password]@host:port）
REDIS_URL=redis://:password@localhost:6379
REDIS_POOL_SIZE=4
```

```rust
use database::Cache;

// 字符串
Cache::set("key", "value").await?;
let val: Option<String> = Cache::get("key").await?;
Cache::setex("token:abc", 3600, "user_id").await?;
Cache::del("key").await?;

// 哈希
Cache::hset("user:1", "name", "张三").await?;
let name: Option<String> = Cache::hget("user:1", "name").await?;

// 判断
let exists = Cache::exists("key").await?;
```

## 🔐 JWT 认证

演示账号：`admin@example.com` / `admin123`

```bash
# 登录获取 token
curl -X POST http://localhost:3000/api/login \
  -H 'content-type: application/json' \
  -d '{"email":"admin@example.com","password":"admin123"}'

# 使用 token 访问受保护路由
curl http://localhost:3000/api/protected \
  -H 'Authorization: Bearer <token>'
```

JWT 密钥通过 `JWT_SECRET` 环境变量配置，中间件 `auth_middleware` 可直接挂载到任意路由组。

## 🔌 WebSocket

连接后返回 `client_id` 和在线人数。支持私聊、广播、在线列表、心跳。

## 🐳 Docker

```bash
docker compose up -d    # 启动（含 MySQL）
docker build -t axum-api-core .
```

## 📚 API 文档

```bash
cargo run --features openapi
# 访问 http://localhost:3000/docs 查看 Swagger UI
```

## 🔧 构建选项

```bash
cargo build                              # 默认构建
cargo build --features openapi           # +OpenAPI 文档
cargo test -p app                        # 运行集成测试
cargo run -p migration up                # 运行数据库迁移
cargo run -p artisan make:controller X   # 生成代码
```

## 📄 License

本项目基于 [MIT License](LICENSE) 开源。
