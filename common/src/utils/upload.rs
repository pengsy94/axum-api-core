use std::path::{Path, PathBuf};
use tokio::io::AsyncWriteExt;

/// 文件上传结果
#[derive(Debug)]
pub struct UploadedFile {
    /// 原始文件名
    pub original_name: String,
    /// 存储文件名（UUID 重命名）
    pub stored_name: String,
    /// 完整存储路径
    pub path: PathBuf,
    /// 文件大小（字节）
    pub size: u64,
}

/// 保存上传的文件到磁盘
///
/// # 参数
/// - `data`: 文件二进制数据
/// - `original_name`: 原始文件名（用于提取扩展名）
/// - `upload_dir`: 存储目录
///
/// # 示例
/// ```ignore
/// use common::utils::upload::save_file;
///
/// let file = save_file(&bytes, "photo.jpg", "/app/uploads").await?;
/// println!("保存至: {}", file.path.display());
/// ```
pub async fn save_file(data: &[u8], original_name: &str, upload_dir: &str) -> Result<UploadedFile, String> {
    let dir = Path::new(upload_dir);

    // 确保目录存在
    tokio::fs::create_dir_all(dir)
        .await
        .map_err(|e| format!("创建上传目录失败: {}", e))?;

    // 生成唯一文件名
    let ext = Path::new(original_name)
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("bin");
    let stored_name = format!("{}_{}.{}", chrono::Local::now().format("%Y%m%d%H%M%S"), uuid::Uuid::new_v4(), ext);
    let path = dir.join(&stored_name);

    // 写入文件
    let mut f = tokio::fs::File::create(&path)
        .await
        .map_err(|e| format!("创建文件失败: {}", e))?;
    f.write_all(data)
        .await
        .map_err(|e| format!("写入文件失败: {}", e))?;

    Ok(UploadedFile {
        original_name: original_name.to_string(),
        stored_name,
        path,
        size: data.len() as u64,
    })
}
