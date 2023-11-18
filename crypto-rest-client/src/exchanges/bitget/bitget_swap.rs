use super::utils::http_raw;
use crate::error::Result;
use base64::encode;
use hmac::{Hmac, Mac};
use log::info;
use reqwest::Response;
use serde_json::{json, Value};
use sha2::Sha256;
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};
const BASE_URL: &str = "https://api.bitget.com";
type HmacSha256 = Hmac<Sha256>;

/// The RESTful client for Bitget swap markets.
///
/// * RESTful API doc: <https://bitgetlimited.github.io/apidoc/en/mix/#restapi>
/// * Trading at: <https://www.bitget.com/mix/>
pub struct BitgetSwapRestClient {
    _api_key: Option<String>,
    _api_secret: Option<String>,
    _api_passphrase: Option<String>,
}

impl BitgetSwapRestClient {
    pub fn new(
        api_key: Option<String>,
        api_secret: Option<String>,
        api_passphrase: Option<String>,
    ) -> Self {
        BitgetSwapRestClient {
            _api_key: api_key,
            _api_secret: api_secret,
            _api_passphrase: api_passphrase,
        }
    }

    /// Get the latest Level2 snapshot of orderbook.
    ///
    /// For example: <https://api.bitget.com/api/mix/v1/market/depth?symbol=BTCUSDT_UMCBL&limit=100>
    ///
    /// Rate Limit：20 requests per 2 seconds
    // pub fn fetch_l2_snapshot(symbol: &str) -> Result<String> {
    //     gen_api_bitget!("GET", format!("/api/mix/v1/market/depth?symbol={symbol}&limit=100"), None)
    // }

    fn create_signature_headers(
        &self,
        method: &str,
        request_path: &str,
        body: Option<&Value>,
    ) -> HashMap<String, Value> {
        let mut headers = HashMap::new();
        headers.insert("ACCESS-KEY".to_string(), self._api_key.clone().into());
        headers.insert("ACCESS-PASSPHRASE".to_string(), self._api_passphrase.clone().into());
        headers.insert("locale".to_string(), "en-US".into());
        let timestamp = (SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs()
            .to_string());
        headers.insert("ACCESS-TIMESTAMP".to_string(), timestamp.clone().into());
        let message = match body {
            Some(value) => {
                format!("{}{}{}{}", timestamp, method, request_path, value.to_string())
            }
            None => {
                format!("{}{}{}", timestamp, method, request_path)
            }
        };
        let mut mac = HmacSha256::new_from_slice(self._api_secret.as_ref().unwrap().as_bytes())
            .expect("Creating HMAC failed");
        mac.update(message.as_bytes());
        let result = mac.finalize();
        let code_bytes = result.into_bytes();
        let sign = encode(&code_bytes);
        headers.insert("ACCESS-SIGN".to_string(), sign.into());
        headers
    }

    pub async fn request(
        &self,
        method: &str,
        path: &str,
        params: Option<&BTreeMap<String, Value>>,
        body: Option<&Value>,
        need_signature: Option<bool>,
    ) -> String {
        let mut headers = match need_signature {
            Some(need_sign) => {
                if need_sign {
                    self.create_signature_headers(&method, path, body)
                } else {
                    HashMap::new()
                }
            }
            _ => HashMap::new(),
        };
        let url = format!("{}{}", BASE_URL, path);
        let response = http_raw(&method, url.as_str(), params, body, headers.into())
            .await
            .expect("Failed request to bitget");
        response.text().await.expect("Failed parse response")
    }
    /// Get open interest.
    ///
    /// For example:
    ///
    /// - <https://api.bitget.com/api/mix/v1/market/open-interest?symbol=BTCUSDT_UMCBL>
    pub async fn fetch_open_interest(&self, symbol: &str) -> String {
        let url = format!("/api/mix/v1/market/open-interest?symbol={symbol}");
        self.request("GET", url.as_str(), None, None, None).await
    }
}
