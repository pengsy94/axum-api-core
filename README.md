# Axum Api Core

基于 `Axum 0.8.8` + Laravel 风格分层的 Rust Web API 基础工程，适合快速搭建可扩展的后端服务。

## 快速入口

- 使用手册：[MANUAL.md](./MANUAL.md)
- 配置文档：[CONFIG.md](./CONFIG.md)
- 请求示例：[test-request.http](./test-request.http)

## 快速开始

```bash
cp .env.example .env
cargo run -p bin
```

默认可访问：

- `http://127.0.0.1:3000/`
- `http://127.0.0.1:3000/health`
- `http://127.0.0.1:3000/ready`

## 你会在手册里看到什么

- 启动与配置
- 响应结构与 `body.code` 语义
- `/health` 与 `/ready` 的区别
- OpenAPI 与可视化 API 文档入口
- 数据库迁移与 Redis 使用
- `Artisan CLI` 脚手架命令
- 项目分层与开发流程
- 已知事项与排错建议

详细内容请直接阅读 [MANUAL.md](./MANUAL.md)。

## License

本项目基于 [MIT License](./LICENSE) 开源。
