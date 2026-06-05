use serde::{Deserialize, Serialize};
use validator::Validate;

/// 分页请求参数
///
/// 在 handler 中通过 `Query<PageParams>` 提取。
#[derive(Debug, Deserialize, Validate)]
pub struct PageParams {
    /// 页码（从 1 开始）
    #[validate(range(min = 1))]
    pub page: Option<u64>,

    /// 每页条数
    #[validate(range(min = 1, max = 500))]
    pub page_size: Option<u64>,
}

impl PageParams {
    /// 获取页码（默认 1）
    pub fn page(&self) -> u64 {
        self.page.unwrap_or(1)
    }

    /// 获取每页条数（默认 20）
    pub fn page_size(&self) -> u64 {
        self.page_size.unwrap_or(20)
    }

    /// 计算 SQL LIMIT
    pub fn limit(&self) -> u64 {
        self.page_size()
    }

    /// 计算 SQL OFFSET
    pub fn offset(&self) -> u64 {
        (self.page().saturating_sub(1)) * self.page_size()
    }
}

/// 分页响应
///
/// 包装列表查询结果。
#[derive(Debug, Serialize)]
pub struct Paginated<T: Serialize> {
    /// 当前页数据
    pub items: Vec<T>,
    /// 总记录数
    pub total: u64,
    /// 当前页码
    pub page: u64,
    /// 每页条数
    pub page_size: u64,
    /// 总页数
    pub total_pages: u64,
}

impl<T: Serialize> Paginated<T> {
    /// 构建分页响应
    pub fn new(items: Vec<T>, total: u64, params: &PageParams) -> Self {
        let page_size = params.page_size();
        let total_pages = (total as f64 / page_size as f64).ceil() as u64;

        Self {
            items,
            total,
            page: params.page(),
            page_size,
            total_pages: total_pages.max(1),
        }
    }
}
