pub mod client;
pub mod error;
pub mod models;

pub use client::HermesClient;
pub use error::HermesError;

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn it_works() -> Result<(), HermesError> {
        // 假设 Hermes 部署在 "https://hermes.pyth.network"
        let client = HermesClient::new("https://hermes.pyth.network");

        // 1. 获取所有 price feeds
        let all_feeds = client.get_price_feeds(None, None).await?;
        println!("Total feeds: {}", all_feeds.len());

        // 2. 获取指定 price feed id(s) 的最新价格
        let feed_ids = ["e62df6c8b4a85fe1a67db44dc12de5db330f7ac66b72dc658afedf0f4a415b43"];
        let latest = client
            .get_latest_price_updates(&feed_ids, Some("hex"), Some(true), Some(false))
            .await?;
        println!("Latest updates: {:?}", latest);


        // 3. 获取指定 time window TWAP
        let twap_resp = client
            .get_twap_latest(300, &feed_ids, Some("hex"), Some(true), None)
            .await?;
        println!("TWAP response: {:?}", twap_resp);

        Ok(())
    }
}
