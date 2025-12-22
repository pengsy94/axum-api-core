use common::config::{RESOURCE_DIR, WEB_VIEW_DIR};
use once_cell::sync::Lazy;
use std::collections::HashMap;
use tera::{Context, Tera};

// 全局缓存 Tera 实例，避免重复加载模板
static TERA_CACHE: Lazy<std::sync::Mutex<Tera>> = Lazy::new(|| {
    let pattern = format!("{}{}/**/*.blade.html", RESOURCE_DIR, WEB_VIEW_DIR);
    let mut tera = match Tera::new(&*pattern) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("Template parsing error(s): {}", e);
            std::process::exit(1);
        }
    };
    tera.autoescape_on(vec![".html", ".htm", ".xml"]);
    std::sync::Mutex::new(tera)
});

/// 渲染模板（使用 HashMap 数据）
pub fn render_map(template_name: &str, data: HashMap<&str, &str>) -> Result<String, String> {
    let mut context = Context::new();
    for (key, value) in data {
        context.insert(key, value);
    }

    let template_name = ensure_suffix(template_name);
    let tera = TERA_CACHE.lock().unwrap();

    match tera.render(&template_name, &context) {
        Ok(html) => Ok(html),
        Err(e) => Err(format!("Template error: {}", e)),
    }
}

pub fn render_view(template_name: &str) -> Result<String, String> {
    let context = Context::new();

    let template_name = ensure_suffix(template_name);
    let tera = TERA_CACHE.lock().unwrap();

    match tera.render(&template_name, &context) {
        Ok(html) => Ok(html),
        Err(e) => Err(format!("Template error: {}", e)),
    }
}

/// 渲染模板（使用可序列化结构体数据）
pub fn render_with_struct<T: serde::Serialize>(
    template_name: &str,
    data: &T,
) -> Result<String, String> {
    let context = match Context::from_serialize(data) {
        Ok(ctx) => ctx,
        Err(e) => return Err(format!("Context error: {}", e)),
    };

    let template_name = ensure_suffix(template_name);
    let tera = TERA_CACHE.lock().unwrap();

    match tera.render(&template_name, &context) {
        Ok(html) => Ok(html),
        Err(e) => Err(format!("Template error: {}", e)),
    }
}

/// 确保模板名以 .blade.html 结尾
fn ensure_suffix(template_name: &str) -> String {
    if template_name.ends_with(".blade.html") {
        template_name.to_string()
    } else {
        format!("{}.blade.html", template_name)
    }
}

/// 检查模板是否存在
pub fn template_exists(template_name: &str) -> bool {
    let template_name = ensure_suffix(template_name);
    let tera = TERA_CACHE.lock().unwrap();
    tera.get_template_names().any(|name| name == template_name)
}

/// 获取所有模板列表
pub fn list_templates() -> Vec<String> {
    let tera = TERA_CACHE.lock().unwrap();
    tera.get_template_names().map(|s| s.to_string()).collect()
}
