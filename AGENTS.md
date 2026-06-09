# Axum Api Core - AGENTS

## 项目目标

使用 Axum 0.8.8 + Laravel 风格分层构建内存占用极少的 Web API 基础服务。

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

## 🏗️ 项目结构（Laravel 风格分层）

```
bin/                         入口，只组装不实现
  src/main.rs                启动入口，信号处理 + 优雅关闭
  src/bootstrap/             启动编排（配置→路由→DB→调度器→系统信息）

bin/artisan/                 Artisan CLI（cargo run -p artisan）
  src/main.rs                命令行入口，make:controller / make:service / make:resource / make:model

app/                         路由注册 + Controller + Service
  src/route.rs               全局路由定义（含 Controller 资源路由）
  src/controllers/           Controller 层（类似 Laravel Controllers）
    mod.rs                   Controller trait + controller_routes! 宏
    user_controller.rs       示例：UserController
  src/services/              Service 层（类似 Laravel Services）
    mod.rs                   Service trait
    user_service.rs          示例：UserService
  src/api/                   旧版 handler（遗留兼容，新代码走 controllers/）
  src/websocket/             WebSocket 连接管理
  src/docs.rs                OpenAPI ApiDoc 定义（feature = "openapi"）
  src/macros.rs              resources! 宏（向后兼容） + controller_routes! 宏

common/                      共享类型
  src/error.rs               AppError 统一错误枚举
  src/auth.rs                AuthUser + JWT + 密码哈希
  src/resources/             API Resource 层（类似 Laravel JsonResource）
    mod.rs                   JsonResource trait + ResourceCollection + simple_resource! 宏
  src/request/               FormRequest + 请求体类型
    form_request.rs          FormRequest trait + form_request! 宏
    system.rs                LoginRequest
  src/response/              响应体类型（LoginResponse）
  src/validator/             参数校验 extractor（ValidatedJson/Form/Path/Query）
  src/utils/                 工具（ApiResponse / 分页 / 上传 / 软删除 / 时间戳）
    response.rs              ApiResponse 统一响应封装
    pagination.rs            PageParams + Paginated<T>
    upload.rs                文件上传处理
    model.rs                 TimeStamp + SoftDelete

middleware/                   中间件
  src/request.rs             logging / rate_limiter / trace_middleware / metrics_middleware
  src/auth.rs                auth_middleware（JWT 认证）

kernel/                      基础设施
  src/config/                配置加载 + 校验（OnceLock + 环境变量）
  src/tasks/                 定时任务调度器（tokio-cron-scheduler）
  src/system.rs              系统信息打印

database/                    数据库层
  src/entity/                SeaORM 自动生成实体
  src/model/                 Eloquent 风格 Model 门面层
    mod.rs                   Model<E> 结构体 + Query<E> 流式查询 + ActiveModelExt
  src/repository/            仓库层（sys_user_repository—旧兼容，新代码走 model/）
  src/redis.rs               Redis 缓存

migration/                   数据库迁移（SeaORM Migration）
```

## 📐 核心约定

### 1. JSON 响应格式

所有 API 统一返回此结构：

```json
{
  "code": 200,
  "data": { ... },
  "message": "success",
  "errors": null
}
```

- 成功：`ApiResponse::success(data)` → code=200, message="success"
- 错误：handler 返回 `Result<_, AppError>`，自动转换

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

### 3. 分层调用链（Laravel 风格）

```
Router → Controller::handler()
              ↓ 调用
         Service::method()
              ↓ 调用
         Model::find() / query() / create()
              ↓ 映射
         Resource::make().respond()  →  ApiResponse
```

- **Controller**：接收请求参数，调用 Service，返回 ApiResponse
- **Service**：封装业务逻辑，调用 Model/Repository
- **Model**：Eloquent 风格 ORM 门面，封装 SeaORM
- **Resource**：控制 JSON 输出格式，每个 Model 对应一个 Resource

### 4. 路由注册

```rust
// Laravel 风格 Controller 路由
router = controller_routes!(router, "/api/users", UserController,
    [index, show, create, update, delete]
);
```

- 新 Controller 路由使用 `controller_routes!` 宏
- 旧版 `resources!` 宏保持向后兼容
- handler 函数签名保持 Axum 标准

### 5. Eloquent 风格 Model 查询

```rust
type User = Model<sys_user::Entity>;

// 静态方法
let user = User::find(1).await?;
let all = User::all().await?;
let count = User::count().await?;
let paginated = User::paginate(&params).await?;

// 流式查询（Query Builder）
let users = User::query()
    .filter_eq(sys_user::Column::Name, "张三")
    .filter_gt(sys_user::Column::Id, 10)
    .limit(10)
    .all().await?;

// 创建 / 更新 / 删除
User::create(|m| { m.name = Set("李寻欢".into()); }).await?;
User::update_by_id(1, |m| { m.name = Set("新名字".into()); }).await?;
User::delete_by_id(1).await?;
```

### 6. API Resource 输出

```rust
pub struct UserResource { user: sys_user::Model }

impl JsonResource for UserResource {
    type Source = sys_user::Model;
    fn from_source(source: Self::Source) -> Self { Self { user: source } }
    fn to_array(&self) -> serde_json::Value {
        json!({ "id": self.user.id, "name": self.user.name, "email": self.user.email })
    }
}

// 使用
UserResource::make(user).respond()              // 单个资源
UserResource::collection(users)                  // 集合
```

### 7. 配置管理

所有配置通过**环境变量**加载（支持 `.env` 文件），启动时自动校验。

- 全局单例 `OnceLock<AppConfig>`
- 通过 `server_config()` / `database_config()` / `redis_config()` 获取配置
- 完整文档见 `CONFIG.md`

### 8. 数据库（可选）

`DATABASE_URL` 为空时跳过数据库初始化，服务仍可正常启动。

- 未配置时：`/ready` → `{"status":"ready","database":"disabled"}`
- 配置时：`/ready` 校验连接池存活

### 9. 中间件顺序（从外到内）

```
trace_middleware（请求追踪, 最外层）
  → logging_middleware（请求日志, 条件启用）
    → rate_limiter（限流, 条件启用）
```

### 10. 日志规范

- 使用 `tracing::info/debug/warn/error`，禁止 `println!`
- 每个请求自动携带 `trace_id`（UUID），支持日志串联

## 📁 关键文件定位

| 需求 | 文件 |
|------|------|
| 添加新 Controller | `app/src/controllers/<name>_controller.rs` + 注册到 `app/src/controllers/mod.rs` |
| 添加新 Service | `app/src/services/<name>_service.rs` + 注册到 `app/src/services/mod.rs` |
| 添加新 Resource | `common/src/resources/<name>_resource.rs` 或直接在 Controller 中定义 |
| 添加新 Model | 用 `sea-orm-cli` 生成 entity + 在 `database/src/entity/` 使用 `Model<E>` |
| 注册路由 | `app/src/route.rs` → `add_api_routes()` → `controller_routes!` |
| 添加中间件 | `middleware/src/request.rs` 或 `auth.rs` |
| 添加配置项 | `kernel/src/config/server_config.rs` + `.env.example` + `CONFIG.md` |
| 添加 FormRequest | `common/src/request/` 下新建，derive `Validate` + 实现 `FormRequest` |
| 数据库迁移 | `cargo run -p migration up` / `down` |
| 代码生成 | `cargo run -p artisan make:controller User` |
| 添加 OpenAPI 文档 | 为 handler 加 `#[cfg_attr(feature = "openapi", utoipa::path(...))]` |
