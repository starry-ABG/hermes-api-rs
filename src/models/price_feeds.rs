use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// GET /v2/price_feeds 接口返回的单个 price feed 信息
#[derive(Debug, Serialize, Deserialize)]
pub struct PriceFeed {
    /// Feed ID
    pub id: String,

    /// Feed 的自定义属性
    #[serde(default)]
    pub attributes: Option<HashMap<String, String>>,
}