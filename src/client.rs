use crate::error::HermesError;
use crate::models::price_feeds::PriceFeed;
use crate::models::price_updates::{PriceUpdatesResponse};
use crate::models::publisher_stake_caps::PublisherStakeCapsResponse;
use crate::models::twap::TwapResponse;

use reqwest::{Client, StatusCode};
use std::fmt::Debug;

/// HermesClient 是对外的主要交互入口
#[derive(Clone, Debug)]
pub struct HermesClient {
    base_url: String,
    http_client: Client,
}

impl HermesClient {
    /// 创建一个新的 HermesClient
    pub fn new<S: Into<String>>(base_url: S) -> Self {
        HermesClient {
            base_url: base_url.into(),
            http_client: Client::new(),
        }
    }

    /// 获取所有 price feeds
    /// GET /v2/price_feeds
    pub async fn get_price_feeds(
        &self,
        query: Option<&str>,
        asset_type: Option<&str>,
    ) -> Result<Vec<PriceFeed>, HermesError> {
        let url = format!("{}/v2/price_feeds", self.base_url);
        let mut request = self.http_client.get(&url);

        if let Some(q) = query {
            request = request.query(&[("query", q)]);
        }
        if let Some(a) = asset_type {
            request = request.query(&[("asset_type", a)]);
        }

        let resp = request.send().await?;
        if resp.status().is_success() {
            let result = resp.json::<Vec<PriceFeed>>().await?;
            Ok(result)
        } else {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            Err(HermesError::ResponseError { status, body })
        }
    }

    /// 获取指定 price feed id(s) 的最新价格更新
    /// GET /v2/updates/price/latest
    pub async fn get_latest_price_updates(
        &self,
        ids: &[&str],
        encoding: Option<&str>,             // hex or base64
        parsed: Option<bool>,               // default = true
        ignore_invalid_price_ids: Option<bool>, // default = false
    ) -> Result<PriceUpdatesResponse, HermesError> {
        let url = format!("{}/v2/updates/price/latest", self.base_url);
        let mut request = self.http_client.get(&url);

        // 设置 query params
        for id in ids {
            request = request.query(&[("ids[]", *id)]);
        }
        if let Some(enc) = encoding {
            request = request.query(&[("encoding", enc)]);
        }
        if let Some(p) = parsed {
            request = request.query(&[("parsed", p.to_string())]);
        }
        if let Some(i) = ignore_invalid_price_ids {
            request = request.query(&[("ignore_invalid_price_ids", i.to_string())]);
        }

        let resp = request.send().await?;
        match resp.status() {
            StatusCode::OK => {
                let result = resp.json::<PriceUpdatesResponse>().await?;
                Ok(result)
            }
            code => {
                let text = resp.text().await.unwrap_or_default();
                Err(HermesError::ResponseError {
                    status: code,
                    body: text,
                })
            }
        }
    }

    /// SSE 接口（GET /v2/updates/price/stream）可根据需要额外封装
    /// 此处仅给出函数签名参考
    /// 如果需要使用 SSE，可以考虑 `reqwest_eventsource` 或者 `eventsource-client` 库
    /*
    pub async fn stream_price_updates(&self, ids: &[&str], /* more params */) -> Result<(), HermesError> {
        let url = format!("{}/v2/updates/price/stream", self.base_url);
        // ...
        todo!()
    }
    */

    /// 根据时间戳查询价格更新
    /// GET /v2/updates/price/{publish_time}
    pub async fn get_price_updates_by_time(
        &self,
        publish_time: i64,
        ids: &[&str],
        encoding: Option<&str>,
        parsed: Option<bool>,
        ignore_invalid_price_ids: Option<bool>,
    ) -> Result<PriceUpdatesResponse, HermesError> {
        let url = format!("{}/v2/updates/price/{}", self.base_url, publish_time);
        let mut request = self.http_client.get(&url);

        // 设置 query params
        for id in ids {
            request = request.query(&[("ids[]", *id)]);
        }
        if let Some(enc) = encoding {
            request = request.query(&[("encoding", enc)]);
        }
        if let Some(p) = parsed {
            request = request.query(&[("parsed", p.to_string())]);
        }
        if let Some(i) = ignore_invalid_price_ids {
            request = request.query(&[("ignore_invalid_price_ids", i.to_string())]);
        }

        let resp = request.send().await?;
        match resp.status() {
            StatusCode::OK => {
                let result = resp.json::<PriceUpdatesResponse>().await?;
                Ok(result)
            }
            code => {
                let text = resp.text().await.unwrap_or_default();
                Err(HermesError::ResponseError {
                    status: code,
                    body: text,
                })
            }
        }
    }

    /// 获取最新的 Publisher Stake Caps
    /// GET /v2/updates/publisher_stake_caps/latest
    pub async fn get_publisher_stake_caps(
        &self,
        encoding: Option<&str>,
        parsed: Option<bool>,
    ) -> Result<PublisherStakeCapsResponse, HermesError> {
        let url = format!("{}/v2/updates/publisher_stake_caps/latest", self.base_url);
        let mut request = self.http_client.get(&url);

        if let Some(enc) = encoding {
            request = request.query(&[("encoding", enc)]);
        }
        if let Some(p) = parsed {
            request = request.query(&[("parsed", p.to_string())]);
        }

        let resp = request.send().await?;
        match resp.status() {
            StatusCode::OK => {
                let result = resp.json::<PublisherStakeCapsResponse>().await?;
                Ok(result)
            }
            code => {
                let text = resp.text().await.unwrap_or_default();
                Err(HermesError::ResponseError {
                    status: code,
                    body: text,
                })
            }
        }
    }

    /// 获取最新自定义窗口TWAP
    /// GET /v2/updates/twap/{window_seconds}/latest
    pub async fn get_twap_latest(
        &self,
        window_seconds: u64,
        ids: &[&str],
        encoding: Option<&str>,
        parsed: Option<bool>,
        ignore_invalid_price_ids: Option<bool>,
    ) -> Result<TwapResponse, HermesError> {
        let url = format!(
            "{}/v2/updates/twap/{}/latest",
            self.base_url, window_seconds
        );
        let mut request = self.http_client.get(&url);

        for id in ids {
            request = request.query(&[("ids[]", *id)]);
        }
        if let Some(enc) = encoding {
            request = request.query(&[("encoding", enc)]);
        }
        if let Some(p) = parsed {
            request = request.query(&[("parsed", p.to_string())]);
        }
        if let Some(i) = ignore_invalid_price_ids {
            request = request.query(&[("ignore_invalid_price_ids", i.to_string())]);
        }

        let resp = request.send().await?;
        match resp.status() {
            StatusCode::OK => {
                let result = resp.json::<TwapResponse>().await?;
                Ok(result)
            }
            code => {
                let text = resp.text().await.unwrap_or_default();
                Err(HermesError::ResponseError {
                    status: code,
                    body: text,
                })
            }
        }
    }
}