use hyperliquid::HyperliquidApi;
use serde_json::Value;
use std::env;

pub struct HyperliquidClient {
    api: HyperliquidApi,
    wallet_address: String,
    private_key: String,
}

impl HyperliquidClient {
    pub fn new() -> Self {
        let wallet_address = env::var("HL_TESTNET_WALLET_ADDRESS")
            .expect("HL_TESTNET_WALLET_ADDRESS must be set");
        let private_key = env::var("HL_TESTNET_PRIVATE_KEY")
            .expect("HL_TESTNET_PRIVATE_KEY must be set");

        let api = HyperliquidApi::testnet();

        Self {
            api,
            wallet_address,
            private_key,
        }
    }

    pub async fn get_balance(&self) -> Result<Value, Box<dyn std::error::Error>> {
        let response = self.api.get_user_state(&self.wallet_address).await?;
        Ok(response)
    }

    pub async fn place_order(
        &self,
        coin: &str,
        is_buy: bool,
        size: f64,
        price: f64,
    ) -> Result<Value, Box<dyn std::error::Error>> {
        let order = self.api
            .create_order(
                &self.wallet_address,
                &self.private_key,
                coin,
                is_buy,
                size,
                Some(price),
            )
            .await?;
        Ok(order)
    }
}