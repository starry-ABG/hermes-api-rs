use serde::{Deserialize, Serialize};

/// TWAP 相关返回
#[derive(Debug, Serialize, Deserialize)]
pub struct TwapResponse {
    #[serde(default)]
    pub binary: Option<TwapBinary>,

    #[serde(default)]
    pub parsed: Option<Vec<TwapParsed>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TwapBinary {
    #[serde(default)]
    pub data: Option<Vec<String>>,
    #[serde(default)]
    pub encoding: Option<String>,
}

/// 单个TWAP的解析结果
#[derive(Debug, Serialize, Deserialize)]
pub struct TwapParsed {
    pub id: String,
    /// TWAP计算开始与结束时间戳
    #[serde(default)]
    pub start_timestamp: Option<i64>,
    #[serde(default)]
    pub end_timestamp: Option<i64>,

    /// 有些接口里会包含下列字段
    #[serde(default)]
    pub down_slots_ratio: Option<String>,

    #[serde(default)]
    pub twap: Option<TwapPriceData>,
}

/// TWAP价格
#[derive(Debug, Serialize, Deserialize)]
pub struct TwapPriceData {
    pub conf: String,
    pub expo: i64,
    pub price: String,
    pub publish_time: i64,
}