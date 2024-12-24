use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PublisherStakeCapsResponse {
    #[serde(default)]
    pub binary: Option<PublisherStakeCapsBinary>,

    #[serde(default)]
    pub parsed: Option<Vec<PublisherStakeCapsParsed>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PublisherStakeCapsBinary {
    #[serde(default)]
    pub data: Option<Vec<String>>,
    #[serde(default)]
    pub encoding: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PublisherStakeCapsParsed {
    #[serde(default)]
    pub publisher_stake_caps: Option<Vec<PublisherStakeCap>>,
}

/// 代表单个 publisher 的 stake cap
#[derive(Debug, Serialize, Deserialize)]
pub struct PublisherStakeCap {
    pub publisher: String,
    pub cap: u64,
}