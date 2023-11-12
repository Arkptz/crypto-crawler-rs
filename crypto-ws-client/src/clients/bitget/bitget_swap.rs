use crate::{
    clients::common_traits::{
        Candlestick, Level3OrderBook, OrderBook, OrderBookTopK, Ticker, Trade, BBO,
    },
    common::{command_translator::CommandTranslator, ws_client_internal::WSClientInternal},
    WSClient,
};
use async_trait::async_trait;
use base64::encode;
use hmac::{Hmac, Mac};
use log::info;
use serde_json::json;
use sha2::Sha256;
use std::time::{SystemTime, UNIX_EPOCH};

use super::{
    utils::{BitgetCommandTranslator, BitgetMessageHandler, UPLINK_LIMIT},
    EXCHANGE_NAME,
};

const WEBSOCKET_URL: &str = "wss://ws.bitget.com/mix/v1/stream";

type HmacSha256 = Hmac<Sha256>;
/// The WebSocket client for Bitget swap markets.
///
/// * WebSocket API doc: <https://bitgetlimited.github.io/apidoc/en/mix/#websocketapi>
/// * Trading at: <https://www.bitget.com/en/swap/>
pub struct BitgetSwapWSClient {
    client: WSClientInternal<BitgetMessageHandler>,
    translator: BitgetCommandTranslator<'M'>,
    api_key: Option<String>,
    api_secret: Option<String>,
    api_passphrase: Option<String>,
}

impl BitgetSwapWSClient {
    pub async fn new(tx: std::sync::mpsc::Sender<String>, url: Option<&str>) -> Self {
        let real_url = match url {
            Some(endpoint) => endpoint,
            None => WEBSOCKET_URL,
        };
        BitgetSwapWSClient {
            client: WSClientInternal::connect(
                EXCHANGE_NAME,
                real_url,
                BitgetMessageHandler {},
                Some(UPLINK_LIMIT),
                tx,
            )
            .await,
            translator: BitgetCommandTranslator::<'M'> {},
            api_key: None,
            api_secret: None,
            api_passphrase: None,
        }
    }
    pub async fn set_api_keys(
        &mut self,
        api_key: Option<String>,
        api_secret: Option<String>,
        api_passphrase: Option<String>,
    ) {
        self.api_key = api_key;
        self.api_secret = api_secret;
        self.api_passphrase = api_passphrase;
    }
    pub async fn login(&self) {
        self.api_key.as_ref().expect("Not specified api_key");
        self.api_secret.as_ref().expect("Not specified api_secret");
        self.api_passphrase.as_ref().expect("Not specified api_passphrase");
        let timestamp = (SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs()
            .to_string());
        let sign = self.create_signature(&timestamp);
        let login_msg = json!({
            "op":"login",
            "args":[
                {
                "apiKey":self.api_key.as_ref().unwrap(),
                "passphrase":self.api_passphrase.as_ref().unwrap(),
                "timestamp":timestamp,
                "sign":sign,

        }]
        });
        info!("{}", timestamp);
        let data = login_msg.to_string();
        self.client.send(&[data]).await
        // let command = format!("
        //     "op":"login",
        //     "args":[", );
        // self.client.send(commands)
    }

    fn create_signature(&self, timestamp: &str) -> String {
        let method = "GET";
        let request_path = "/user/verify";
        let message = format!("{}{}{}", timestamp, method, request_path);
        let mut mac = HmacSha256::new_from_slice(self.api_secret.as_ref().unwrap().as_bytes())
            .expect("Creating HMAC failed");
        mac.update(message.as_bytes());
        let result = mac.finalize();
        let code_bytes = result.into_bytes();
        encode(&code_bytes)
    }

    // pub async fn qq(&self){
    //     &self.client.send(commands)
    // }
}

impl_trait!(Trade, BitgetSwapWSClient, subscribe_trade, "trade");
#[rustfmt::skip]
impl_trait!(OrderBookTopK, BitgetSwapWSClient, subscribe_orderbook_topk, "books15");
impl_trait!(OrderBook, BitgetSwapWSClient, subscribe_orderbook, "books");
impl_trait!(Ticker, BitgetSwapWSClient, subscribe_ticker, "ticker");
impl_candlestick!(BitgetSwapWSClient);

panic_bbo!(BitgetSwapWSClient);
panic_l3_orderbook!(BitgetSwapWSClient);

impl_ws_client_trait!(BitgetSwapWSClient);
