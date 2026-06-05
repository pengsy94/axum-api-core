#!/usr/bin/env bash
set -euo pipefail

# ============================================
# Axum Artisan — 代码脚手架
# 用法：
#   bash scripts/make.sh controller User
#   bash scripts/make.sh model Product
#   bash scripts/make.sh entity Order
# ============================================

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
CMD="${1:-help}"
NAME="${2:-}"

# ---- 工具函数 ----

# 转小写下划线：UserName → user_name
to_snake() {
  echo "$1" | sed -E 's/([a-z0-9])([A-Z])/\1_\2/g; s/([A-Z])([A-Z][a-z])/\1_\2/g' | tr '[:upper:]' '[:lower:]'
}

# macOS sed 兼容（需要空备份后缀）
if [[ "$(uname)" == "Darwin" ]]; then
  SED_INPLACE=(sed -i '')
else
  SED_INPLACE=(sed -i)
fi

replace_placeholders() {
  local file="$1"
  local name="$2"
  local snake
  snake="$(to_snake "$name")"
  "${SED_INPLACE[@]}" \
    -e "s/{{Name}}/$name/g" \
    -e "s/{{name}}/$(echo "$name" | tr '[:upper:]' '[:lower:]')/g" \
    -e "s/{{snake_name}}/$snake/g" \
    -e "s/{{NAME_UPPER}}/$(echo "$name" | tr '[:lower:]' '[:upper:]')/g" \
    "$file"
}

add_pub_mod() {
  local mod_file="$1"
  local mod_name="$2"
  if ! grep -q "pub mod $mod_name;" "$mod_file" 2>/dev/null; then
    echo "pub mod $mod_name;" >> "$mod_file"
    echo "   → 已追加 'pub mod $mod_name;' 到 $(basename "$mod_file")"
  fi
}

# ---- 命令 ----

make_controller() {
  local name="$1"
  local snake
  snake="$(to_snake "$name")"
  local target="app/src/api/${snake}.rs"

  if [[ -f "$ROOT/$target" ]]; then
    echo "⚠️  已存在: $target"
    return
  fi

  cp "$ROOT/templates/controller.rs.hbs" "$ROOT/$target"
  replace_placeholders "$ROOT/$target" "$name"
  add_pub_mod "$ROOT/app/src/api/mod.rs" "$snake"
  echo "✅  Controller 已创建: $target"
  echo "   注册到: app/src/api/mod.rs (pub mod $snake;)"
}

make_entity() {
  local name="$1"
  local snake
  snake="$(to_snake "$name")"
  local target="database/src/entity/${snake}.rs"

  if [[ -f "$ROOT/$target" ]]; then
    echo "⚠️  已存在: $target"
    return
  fi

  cp "$ROOT/templates/entity.rs.hbs" "$ROOT/$target"
  replace_placeholders "$ROOT/$target" "$name"
  add_pub_mod "$ROOT/database/src/entity/mod.rs" "$snake"
  echo "✅  Entity 已创建: $target"
  echo "   注册到: database/src/entity/mod.rs"
}

make_repository() {
  local name="$1"
  local snake
  snake="$(to_snake "$name")"
  local target="database/src/repository/${snake}_repository.rs"

  if [[ -f "$ROOT/$target" ]]; then
    echo "⚠️  已存在: $target"
    return
  fi

  cp "$ROOT/templates/repository.rs.hbs" "$ROOT/$target"
  replace_placeholders "$ROOT/$target" "$name"
  add_pub_mod "$ROOT/database/src/repository/mod.rs" "${snake}_repository"
  echo "✅  Repository 已创建: $target"
  echo "   注册到: database/src/repository/mod.rs"
}

# ---- Dispatch ----

case "$CMD" in
  controller)
    make_controller "$NAME"
    ;;
  model)
    make_entity "$NAME"
    make_repository "$NAME"
    ;;
  entity)
    make_entity "$NAME"
    ;;
  hash-password)
    if command -v cargo &>/dev/null; then
      # 使用 Rust 生成哈希
      cargo run --bin hash-pw 2>/dev/null || {
        # 回退到 openssl
        if command -v openssl &>/dev/null; then
          openssl rand -base64 32
        else
          echo "请安装 openssl 或使用 Rust 编译"
        fi
      }
    fi
    ;;
  *)
    echo "用法: bash scripts/make.sh <command> <Name>"
    echo ""
    echo "命令:"
    echo "  controller <Name>   创建 Controller (app/src/api/)"
    echo "  model     <Name>   创建 Entity + Repository (全栈)"
    echo "  entity    <Name>   仅创建 Entity (database/src/entity/)"
    echo ""
    echo "示例:"
    echo "  bash scripts/make.sh controller User"
    echo "  bash scripts/make.sh model Product"
    echo "  bash scripts/make.sh entity Order"
    ;;
esac
