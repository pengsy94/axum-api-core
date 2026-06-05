# Axum Api Core

基于 **Axum 0.8.8** 的轻量级 Web 服务基础框架。内存占用极少，适合低配云服务器部署。

## ✨ 特性

- ⚡ **Axum 0.8.8** — 高性能异步 HTTP 框架
- 🧱 **模块化 workspace** — 6 个 crate 职责清晰
- 📦 **数据库可选** — 不配 DATABASE_URL 也能跑
- 🔐 **认证中间件骨架** — 替换 `validate_token` 即可接入 JWT
- 📋 **请求追踪** — 每个请求自动注入 `X-Trace-Id` 关联日志
- 🚦 **限流 / CORS / Gzip** — 开箱即用
- 📊 **健康检查** — `/health`（存活）+ `/ready`（就绪）
- 📚 **OpenAPI 文档** — `cargo run --features openapi` 一键生成
- 🔌 **WebSocket** — 私聊 / 广播 / 在线列表
- ⏰ **定时任务** — tokio-cron-scheduler
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

## 🏗️ 项目结构

```
bin/                入口，只组装不实现
app/                路由注册 + API handler
common/             共享类型（响应格式 / 错误枚举 / 校验器）
middleware/         中间件（日志 / 限流 / 请求追踪 / 认证）
kernel/             基础设施（配置 / 调度器）
database/           数据库层（实体 / 仓库）
```

详细说明见 [`AGENTS.md`](AGENTS.md)。

## ⚙️ 配置

通过环境变量配置（支持 `.env` 文件），启动时自动校验。

```env
# 服务器
SERVER_HOST=0.0.0.0
SERVER_PORT=3000
LOG_LEVEL=INFO

# 数据库（可选，留空即不启用）
DATABASE_URL=

# WebSocket（可选）
SERVER_WS_OPEN=false
```

全部 18+ 个配置项及说明见 [`CONFIG.md`](CONFIG.md)。

## 📡 API 端点

| 端点 | 方法 | 说明 | 条件 |
|------|------|------|------|
| `/` | GET/POST | 欢迎页面 |
| `/health` | GET | 存活检查（始终 200） |
| `/ready` | GET | 就绪检查（验证 DB） |
| `/api/login` | POST | 用户登录 |
| `/api/openapi.json` | GET | OpenAPI 规范 | `--features openapi` |
| `/docs` | GET | Swagger UI | `--features openapi` |
| `/test/*` | - | 测试路由 | `DEBUG=true` |
| `/ws` | WS | WebSocket | `SERVER_WS_OPEN=true` |

## 💾 数据库

使用 **SeaORM 2.0**，支持 MySQL / PostgreSQL / SQLite。

**数据库为可选**：`DATABASE_URL` 留空时跳过初始化，不影响服务启动。

生成实体：

```bash
cargo install sea-orm-cli
sea-orm-cli generate entity \
  -u mysql://root:password@localhost:3306/database \
  --with-serde both \
  -o database/src/entity
```

## 🔌 WebSocket

连接后返回 `client_id` 和在线人数：

```json
{"type": "connected", "client_id": "04a56e58-...", "online_count": 2}
```

支持操作：私聊 / 广播 / 在线列表 / 心跳。详情见 WebSocket 源码。

## 🐳 Docker

```bash
# 启动（含 MySQL）
docker compose up -d

# 仅构建镜像
docker build -t axum-api-core .
```

## 📚 API 文档

```bash
cargo run --features openapi
```

访问：
- OpenAPI JSON：`http://localhost:3000/api/openapi.json`
- Swagger UI：`http://localhost:3000/docs`

## 🔧 构建选项

```bash
# 默认构建（无 OpenAPI）
cargo build

# 启用 OpenAPI 文档
cargo build --features openapi

# 生产构建
cargo build --release

# 生产构建 + OpenAPI
cargo build --release --features openapi
```

## 📄 License

MIT
