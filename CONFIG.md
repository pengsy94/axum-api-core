# 配置文档

所有配置通过**环境变量**加载（支持 `.env` 文件），启动时自动读取并校验。

---

## 环境选择

| 变量名 | 类型 | 默认值 | 说明 |
|--------|------|--------|------|
| `APP_ENV` | `string` | `local` | 运行环境，设置后自动加载 `.env.{APP_ENV}` 文件（优先级：已有环境变量 > `.env.{APP_ENV}` > `.env`） |

### 多环境示例

```bash
# 本地开发（默认）
# 加载 .env.local → .env
APP_ENV=local

# 生产环境
# 加载 .env.production → .env
APP_ENV=production

# 直接指定文件（不依赖 APP_ENV）
# 单文件模式，与现有行为一致
```

---

## 服务器配置

| 变量名 | 类型 | 默认值 | 说明 |
|--------|------|--------|------|
| `DEBUG` | `bool` | `true` | 调试模式，开启后挂载 `/test` 测试路由 |
| `SERVER_HOST` | `IpAddr` | `0.0.0.0` | 监听地址（IPv4 或 IPv6） |
| `SERVER_PORT` | `u16` | `3000` | 监听端口 |
| `SERVER_CONTENT_GZIP` | `bool` | `true` | 是否启用 Gzip 压缩（SSE 数据自动跳过 `text/event-stream`） |
| `SERVER_CRON` | `bool` | `false` | 是否启用定时任务调度器 |
| `SERVER_WS_OPEN` | `bool` | `false` | 是否开启 WebSocket 服务 |
| `SERVER_WS_PATH` | `string` | `/ws` | WebSocket 路径前缀 |
| `SERVER_RATE_LIMIT_ENABLED` | `bool` | `true` | 是否启用请求限流（100 req/s 阈值，超限延迟 100ms） |
| `CORS_ALLOWED_ORIGINS` | `string` | `*` | CORS 允许的来源，`*` 表示全部放行，多个用逗号分隔（如 `https://a.com,https://b.com`） |

---

## 日志配置

| 变量名 | 类型 | 默认值 | 说明 |
|--------|------|--------|------|
| `LOG_LEVEL` | `string` | `INFO` | 日志级别：`TRACE` / `DEBUG` / `INFO` / `WARN` / `ERROR` |
| `LOG_DIR` | `string` | `logs` | 日志文件输出目录 |
| `LOG_FILE` | `string` | `axum_log` | 日志文件前缀名（按小时滚动） |
| `LOG_ENABLE_OPER_LOG` | `bool` | `true` | 是否记录请求/响应操作日志（影响性能，生产按需开启） |

---

## 数据库配置

| 变量名 | 类型 | 默认值 | 说明 |
|--------|------|--------|------|
| `DATABASE_URL` | `string` | `mysql://root:root@localhost:3306/database` | 数据库连接字符串 |
| `DATABASE_MAX_CONNECTIONS` | `u32` | `10` | 连接池最大连接数 |
| `DATABASE_MIN_CONNECTIONS` | `u32` | `2` | 连接池最小连接数（空闲保留） |
| `DATABASE_CONNECT_TIMEOUT` | `u32` | `30` | 连接超时时间（秒） |

### 数据库 URL 格式

```
MySQL:    mysql://user:password@host:port/database
PostgreSQL: postgres://user:password@host:port/database
SQLite:   sqlite:///path/to/database.db?mode=rwc
```

> 编译时通过 feature flag 选择支持的数据库类型（默认全部开启）：
> - `default` — postgres + mysql + sqlite
> - `mysql` — 仅 MySQL
> - `postgres` — 仅 PostgreSQL
> - `sqlite` — 仅 SQLite

---

## 完整示例 (.env)

```env
# ===== 服务器 =====
DEBUG=true
SERVER_HOST=0.0.0.0
SERVER_PORT=3000
SERVER_CONTENT_GZIP=true
SERVER_CRON=false
SERVER_WS_OPEN=false
SERVER_WS_PATH=/ws
SERVER_RATE_LIMIT_ENABLED=true
CORS_ALLOWED_ORIGINS=*

# ===== 日志 =====
LOG_LEVEL=INFO
LOG_DIR=logs
LOG_FILE=axum_log
LOG_ENABLE_OPER_LOG=true

# ===== 数据库 =====
DATABASE_URL=mysql://root:password@localhost:3306/database
DATABASE_MAX_CONNECTIONS=10
DATABASE_MIN_CONNECTIONS=2
DATABASE_CONNECT_TIMEOUT=30
```

---

## 验证规则

启动时自动校验以下规则，不通过则打印错误并退出：

| 配置项 | 校验规则 |
|--------|----------|
| `SERVER_HOST` | 必须是合法的 IPv4 或 IPv6 地址 |
| `SERVER_PORT` | 必须是 1–65535 之间的整数 |
| `LOG_LEVEL` | 必须是 `TRACE`/`DEBUG`/`INFO`/`WARN`/`ERROR` 之一 |
| `DATABASE_URL` | 不能为空 |
| `DATABASE_MAX_CONNECTIONS` | 必须 ≥ `MIN_CONNECTIONS` |
| `DATABASE_CONNECT_TIMEOUT` | 必须 ≥ 1 秒 |
