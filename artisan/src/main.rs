//! Artisan CLI — 类似 Laravel Artisan 的命令行工具
//!
//! # 用法
//!
//! ```bash
//! cargo run -p artisan -- make:controller User
//! cargo run -p artisan -- make:model Product
//! cargo run -p artisan -- make:service Order
//! cargo run -p artisan -- make:resource Category
//! ```

use clap::{Parser, Subcommand};
use std::fs;
use std::path::PathBuf;

/// Artisan CLI — Axum API Core 的脚手架工具
#[derive(Parser)]
#[command(name = "artisan", about = "Axum API Core 脚手架工具")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// 创建 Controller
    MakeController {
        /// Controller 名称（PascalCase，如 User、Order）
        name: String,
    },
    /// 创建 Service
    MakeService {
        /// Service 名称（PascalCase，如 User、Order）
        name: String,
    },
    /// 创建 Resource
    MakeResource {
        /// Resource 名称（PascalCase，如 User、Order）
        name: String,
    },
    /// 创建 Model（Entity + Repository 占位）
    MakeModel {
        /// Model 名称（PascalCase，如 Product、Category）
        name: String,
    },
}

fn main() {
    let cli = Cli::parse();

    let result = match cli.command {
        Commands::MakeController { name } => make_controller(&name),
        Commands::MakeService { name } => make_service(&name),
        Commands::MakeResource { name } => make_resource(&name),
        Commands::MakeModel { name } => make_model(&name),
    };

    match result {
        Ok(msg) => println!("✅ {}", msg),
        Err(e) => eprintln!("❌ {}", e),
    }
}

// ========================================
// 工具函数
// ========================================

fn to_snake(name: &str) -> String {
    let mut result = String::new();
    for (i, ch) in name.chars().enumerate() {
        if ch.is_uppercase() && i > 0 {
            result.push('_');
        }
        result.push(ch.to_ascii_lowercase());
    }
    result
}

fn workspace_root() -> PathBuf {
    // 从 artisan/ 向上一级到 workspace root
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .to_path_buf()
}

// ========================================
// make:controller
// ========================================

fn make_controller(name: &str) -> Result<String, String> {
    let snake = to_snake(name);
    let file_name = format!("{}_controller.rs", snake);
    let path = workspace_root()
        .join("app/src/controllers")
        .join(&file_name);

    if path.exists() {
        return Err(format!("已存在: {}", path.display()));
    }

    let content = format!(
        r#"//! {name} 控制器

use axum::extract::Path;
use common::resources::JsonResource;
use common::utils::response::ApiResponse;
use database::entity::sys_{snake};
use serde_json::json;

use super::Controller;
use crate::services::{snake}_service::{name}Service;

// ========================================
// {name}Resource
// ========================================

pub struct {name}Resource {{
    data: sys_{snake}::Model,
}}

impl JsonResource for {name}Resource {{
    type Source = sys_{snake}::Model;

    fn from_source(source: Self::Source) -> Self {{
        Self {{ data: source }}
    }}

    fn to_array(&self) -> serde_json::Value {{
        json!({{
            "id": self.data.id,
            "name": &self.data.name,
        }})
    }}
}}

// ========================================
// {name}Controller
// ========================================

pub struct {name}Controller {{
    service: {name}Service,
}}

impl {name}Controller {{
    pub fn new() -> Self {{
        Self {{ service: {name}Service::new() }}
    }}
}}

impl Controller for {name}Controller {{
    fn name() -> &'static str {{ "{name}Controller" }}
}}

impl {name}Controller {{
    /// GET /api/{snake}
    pub async fn index() -> ApiResponse<serde_json::Value> {{
        let service = {name}Service::new();
        // TODO: service.list(&params).await
        ApiResponse::success(json!({{ "message": "TODO" }}))
    }}

    /// GET /api/{snake}/{{id}}
    pub async fn show(Path(id): Path<i32>) -> ApiResponse<serde_json::Value> {{
        let service = {name}Service::new();
        match service.show(id).await {{
            Ok(Some(data)) => {name}Resource::make(data).respond(),
            Ok(None) => ApiResponse::error(404, "记录不存在"),
            Err(e) => ApiResponse::error(500, &e.to_string()),
        }}
    }}

    /// POST /api/{snake}
    pub async fn create() -> ApiResponse<serde_json::Value> {{
        ApiResponse::success(json!({{ "message": "TODO" }}))
    }}

    /// PUT /api/{snake}/{{id}}
    pub async fn update(Path(id): Path<i32>) -> ApiResponse<serde_json::Value> {{
        ApiResponse::success(json!({{ "message": "TODO" }}))
    }}

    /// DELETE /api/{snake}/{{id}}
    pub async fn delete(Path(id): Path<i32>) -> ApiResponse<serde_json::Value> {{
        let service = {name}Service::new();
        match service.destroy(id).await {{
            Ok(()) => ApiResponse::success(json!({{ "deleted": true }})),
            Err(e) => ApiResponse::error(500, &e.to_string()),
        }}
    }}
}}
"#,
    );

    fs::write(&path, content).map_err(|e| e.to_string())?;

    // 注册到 mod.rs
    let mod_path = workspace_root().join("app/src/controllers/mod.rs");
    let mod_content = fs::read_to_string(&mod_path).map_err(|e| e.to_string())?;
    let mod_line = format!("pub mod {}_controller;", snake);
    if !mod_content.contains(&mod_line) {
        let new_content = format!("{}\n{}", mod_content, mod_line);
        fs::write(&mod_path, new_content).map_err(|e| e.to_string())?;
    }

    Ok(format!("Controller 已创建: {}", file_name))
}

// ========================================
// make:service
// ========================================

fn make_service(name: &str) -> Result<String, String> {
    let snake = to_snake(name);
    let file_name = format!("{}_service.rs", snake);
    let path = workspace_root().join("app/src/services").join(&file_name);

    if path.exists() {
        return Err(format!("已存在: {}", path.display()));
    }

    let content = format!(
        r#"//! {name} 服务

use common::error::AppError;
use common::utils::pagination::{{PageParams, Paginated}};
use database::entity::sys_{snake};
use database::model::Model;

use super::Service;

pub struct {name}Service;

impl {name}Service {{
    pub fn new() -> Self {{ Self }}
}}

impl Service for {name}Service {{
    type Error = AppError;
    fn name() -> &'static str {{ "{name}Service" }}
}}

type {name}Model = Model<sys_{snake}::Entity>;

impl {name}Service {{
    pub async fn list(&self, params: &PageParams) -> Result<Paginated<sys_{snake}::Model>, AppError> {{
        {name}Model::paginate(params).await
    }}

    pub async fn show(&self, id: i32) -> Result<Option<sys_{snake}::Model>, AppError> {{
        {name}Model::find(id).await
    }}

    pub async fn destroy(&self, id: i32) -> Result<(), AppError> {{
        {name}Model::delete_by_id(id).await?;
        Ok(())
    }}
}}
"#,
    );

    fs::write(&path, content).map_err(|e| e.to_string())?;

    // 注册到 mod.rs
    let mod_path = workspace_root().join("app/src/services/mod.rs");
    let mod_content = fs::read_to_string(&mod_path).map_err(|e| e.to_string())?;
    let mod_line = format!("pub mod {}_service;", snake);
    if !mod_content.contains(&mod_line) {
        let new_content = format!("{}\n{}", mod_content, mod_line);
        fs::write(&mod_path, new_content).map_err(|e| e.to_string())?;
    }

    Ok(format!("Service 已创建: {}", file_name))
}

// ========================================
// make:resource
// ========================================

fn make_resource(name: &str) -> Result<String, String> {
    let snake = to_snake(name);
    let file_name = format!("{}_resource.rs", snake);
    let path = workspace_root().join("common/src/resources").join(&file_name);

    if path.exists() {
        return Err(format!("已存在: {}", path.display()));
    }

    let content = format!(
        r#"//! {name} 资源

use common::resources::JsonResource;
use serde_json::json;

pub struct {name}Resource {{
    data: TODO_Model,
}}

impl JsonResource for {name}Resource {{
    type Source = TODO_Model;

    fn from_source(source: Self::Source) -> Self {{
        Self {{ data: source }}
    }}

    fn to_array(&self) -> serde_json::Value {{
        json!({{
            "id": self.data.id,
        }})
    }}
}}
"#,
    );

    fs::write(&path, content).map_err(|e| e.to_string())?;
    Ok(format!("Resource 已创建: {}", file_name))
}

// ========================================
// make:model
// ========================================

fn make_model(name: &str) -> Result<String, String> {
    let snake = to_snake(name);
    let file_name = format!("{}.rs", snake);
    let path = workspace_root().join("database/src/entity").join(&file_name);

    if path.exists() {
        return Err(format!("已存在: {}", path.display()));
    }

    let content = format!(
        r#"//! SeaORM Entity for `{snake}` table

use sea_orm::entity::prelude::*;
use serde::{{Deserialize, Serialize}};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "{snake}")]
pub struct Model {{
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: Option<String>,
}}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {{}}

impl ActiveModelBehavior for ActiveModel {{}}
"#,
    );

    fs::write(&path, content).map_err(|e| e.to_string())?;

    // 注册到 entity/mod.rs
    let mod_path = workspace_root().join("database/src/entity/mod.rs");
    let mod_content = fs::read_to_string(&mod_path).map_err(|e| e.to_string())?;
    let mod_line = format!("pub mod {};", snake);
    if !mod_content.contains(&mod_line) {
        let new_content = format!("{}\n{}", mod_content, mod_line);
        fs::write(&mod_path, new_content).map_err(|e| e.to_string())?;
    }

    Ok(format!("Model 已创建: {}", file_name))
}
