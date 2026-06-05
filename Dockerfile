# ============================================
# Axum Api Core - 多阶段构建镜像
# ============================================

# ---- Builder 阶段 ----
FROM rust:1.85-slim-bookworm AS builder

WORKDIR /app
COPY . .

# 构建 release 二进制
RUN cargo build --release --bin bin

# ---- Runtime 阶段 ----
FROM debian:bookworm-slim

WORKDIR /app

# 安装运行时依赖（sea-orm 的 native-tls 需要系统 libc）
RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# 复制编译产物
COPY --from=builder /app/target/release/bin /app/bin
COPY --from=builder /app/.env.example /app/.env

EXPOSE 3000

CMD ["/app/bin"]
