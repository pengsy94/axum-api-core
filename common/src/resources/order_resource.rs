//! Order 资源

use common::resources::JsonResource;
use serde_json::json;

pub struct OrderResource {
    data: TODO_Model,
}

impl JsonResource for OrderResource {
    type Source = TODO_Model;

    fn from_source(source: Self::Source) -> Self {
        Self { data: source }
    }

    fn to_array(&self) -> serde_json::Value {
        json!({
            "id": self.data.id,
        })
    }
}
