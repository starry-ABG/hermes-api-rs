use serde::{Deserialize, Serialize};

/// 价格更新
#[derive(Debug, Serialize, Deserialize)]
pub struct PriceUpdate {
    pub id: String,

    /// 常规价格
    #[serde(default)]
    pub price: Option<PriceData>,

    /// EMA 价格
    #[serde(default)]
    pub ema_price: Option<PriceData>,

    /// 更新元数据
    #[serde(default)]
    pub metadata: Option<PriceMetadata>,
}

/// 价格数据
#[derive(Debug, Serialize, Deserialize)]
pub struct PriceData {
    pub conf: String,
    pub expo: i64,
    pub price: String,
    pub publish_time: i64,
}

/// 更新元数据
#[derive(Debug, Serialize, Deserialize)]
pub struct PriceMetadata {
    #[serde(default)]
    pub prev_publish_time: Option<i64>,
    #[serde(default)]
    pub proof_available_time: Option<i64>,
    #[serde(default)]
    pub slot: Option<u64>,
}

/// 该结构与许多接口的返回格式对应，比如：
/// { "binary": { "data": [...], "encoding": "hex"}, "parsed": [...] }
#[derive(Debug, Serialize, Deserialize)]
pub struct PriceUpdatesResponse {
    #[serde(default)]
    pub binary: Option<BinaryData>,

    #[serde(default)]
    pub parsed: Option<Vec<PriceUpdate>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BinaryData {
    #[serde(default)]
    pub data: Option<Vec<String>>,

    #[serde(default)]
    pub encoding: Option<String>,
}