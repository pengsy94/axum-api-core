//! API Resource 层 — 类似 Laravel 的 JsonResource
//!
//! 将 Model 转换为标准 JSON 响应的中间层，实现表现层转换。
//!
//! # 快速使用
//!
//! ```ignore
//! use common::resources::{JsonResource, ResourceCollection};
//!
//! // 定义资源
//! pub struct UserResource {
//!     user: sys_user::Model,
//! }
//!
//! impl JsonResource for UserResource {
//!     type Source = sys_user::Model;
//!
//!     fn from_source(source: Self::Source) -> Self {
//!         Self { user: source }
//!     }
//!
//!     fn to_array(&self) -> serde_json::Value {
//!         json!({
//!             "id": self.user.id,
//!             "name": self.user.name,
//!         })
//!     }
//! }
//!
//! // 使用
//! let user = User::find(1).await?;
//! let response = UserResource::make(user).respond();
//! // 或集合
//! let users = User::all().await?;
//! let response = UserResource::collection(users);
//! ```


use crate::utils::response::ApiResponse;

/// JSON 资源 trait — 对应 Laravel 的 JsonResource
///
/// 实现此 trait 的类型可以将 Model 转换为 JSON 响应。
pub trait JsonResource: Sized {
    /// 源数据类型（通常是 Model）
    type Source;

    /// 从源数据构造资源
    fn from_source(source: Self::Source) -> Self;

    /// 将资源转换为 JSON 对象
    fn to_array(&self) -> serde_json::Value;

    /// 创建单个资源并包装为 ApiResponse
    ///
    /// 对应 Laravel: `new UserResource($user)`
    fn make(source: Self::Source) -> Self {
        Self::from_source(source)
    }

    /// 将资源转换为 ApiResponse
    ///
    /// 对应 Laravel: `(new UserResource($user))->response()`
    fn respond(self) -> ApiResponse<serde_json::Value> {
        ApiResponse::success(self.to_array())
    }

    /// 转换为带消息的响应
    fn respond_with_message(self, message: &str) -> ApiResponse<serde_json::Value> {
        ApiResponse::success_with_message(self.to_array(), message)
    }
}

/// 资源集合 trait — 对应 Laravel 的 ResourceCollection
pub trait ResourceCollection: JsonResource {
    /// 将集合转换为 ApiResponse
    ///
    /// 对应 Laravel: `UserResource::collection($users)`
    fn collection(sources: impl IntoIterator<Item = Self::Source>) -> ApiResponse<Vec<serde_json::Value>> {
        let items: Vec<serde_json::Value> = sources
            .into_iter()
            .map(|s| Self::from_source(s).to_array())
            .collect();
        ApiResponse::success(items)
    }

    /// 集合响应带消息
    fn collection_with_message(
        sources: impl IntoIterator<Item = Self::Source>,
        message: &str,
    ) -> ApiResponse<Vec<serde_json::Value>> {
        let items: Vec<serde_json::Value> = sources
            .into_iter()
            .map(|s| Self::from_source(s).to_array())
            .collect();
        ApiResponse::success_with_message(items, message)
    }
}

// 为所有 JsonResource 实现 ResourceCollection
impl<T: JsonResource> ResourceCollection for T {}

// ========================================
// 便捷宏：快速定义简单资源
// ========================================

/// 快速创建一个简单的 JsonResource
///
/// # Example
/// ```ignore
/// simple_resource!(UserResource, sys_user::Model, |user| json!({
///     "id": user.id,
///     "name": user.name,
/// }));
/// ```
#[macro_export]
macro_rules! simple_resource {
    ($name:ident, $source:ty, |$var:ident| $body:expr) => {
        pub struct $name {
            $var: $source,
        }

        impl ::common::resources::JsonResource for $name {
            type Source = $source;

            fn from_source(source: Self::Source) -> Self {
                Self { $var: source }
            }

            fn to_array(&self) -> serde_json::Value {
                let $var = &self.$var;
                $body
            }
        }
    };
}

/// 创建分页响应包装
///
/// 将 Paginated 数据通过 Resource 转换后返回
pub fn paginated_resource<R: JsonResource>(
    items: Vec<R::Source>,
    _resource: std::marker::PhantomData<R>,
) -> ApiResponse<Vec<serde_json::Value>> {
    ApiResponse::success(
        items
            .into_iter()
            .map(|s| R::from_source(s).to_array())
            .collect(),
    )
}

// ========================================
// 测试
// ========================================

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    struct TestResource {
        data: TestModel,
    }

    #[derive(Clone)]
    struct TestModel {
        id: i32,
        name: String,
    }

    impl JsonResource for TestResource {
        type Source = TestModel;

        fn from_source(source: Self::Source) -> Self {
            Self { data: source }
        }

        fn to_array(&self) -> serde_json::Value {
            json!({
                "id": self.data.id,
                "name": self.data.name,
            })
        }
    }

    #[test]
    fn test_single_resource() {
        let model = TestModel {
            id: 1,
            name: "测试".into(),
        };
        let resp = TestResource::make(model).respond();
        assert_eq!(resp.code, 200);
        assert_eq!(resp.data.unwrap()["name"], "测试");
    }

    #[test]
    fn test_collection_resource() {
        let models = vec![
            TestModel {
                id: 1,
                name: "A".into(),
            },
            TestModel {
                id: 2,
                name: "B".into(),
            },
        ];
        let resp = TestResource::collection(models);
        assert_eq!(resp.code, 200);
        let data = resp.data.unwrap();
        assert_eq!(data.len(), 2);
    }
}
