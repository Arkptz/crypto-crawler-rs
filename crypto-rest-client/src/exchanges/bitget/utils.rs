use crate::error::{Error, Result};
use reqwest;
use reqwest::{
    header,
    header::{HeaderMap, HeaderName, HeaderValue},
    Method, Response,
};
use serde_json::Value;
use std::collections::HashMap;
use std::{collections::BTreeMap, str::FromStr};

fn convert_hashmap_to_headermap(hashmap: HashMap<String, Value>) -> HeaderMap {
    let mut header_map = HeaderMap::new();

    for (key, value) in hashmap {
        let header_name = HeaderName::from_str(&key).expect("Invalid Header Name");
        let header_value =
            HeaderValue::from_str(&value.as_str().unwrap()).expect("Invalid Header Value");

        header_map.insert(header_name, header_value);
    }

    header_map
}
// Returns the raw response directly.
pub(super) async fn http_raw(
    method: &str,
    url: &str,
    params: Option<&BTreeMap<String, Value>>,
    body: Option<&Value>,
    headers: Option<HashMap<String, Value>>,
) -> Result<Response> {
    let mut full_url = url.to_string();
    let mut first = true;
    match params {
        Some(params) => {
            for (k, v) in params.iter() {
                if first {
                    full_url.push_str(format!("?{k}={v}").as_str());
                    first = false;
                } else {
                    full_url.push_str(format!("&{k}={v}").as_str());
                }
            }
        }
        _ => {}
    };
    // println!("{}", full_url);

    let mut headers = match headers {
        Some(value) => convert_hashmap_to_headermap(value),
        _ => header::HeaderMap::new(),
    };

    headers.insert(header::CONTENT_TYPE, header::HeaderValue::from_static("application/json"));
    let client = reqwest::Client::builder()
         .default_headers(headers)
         .user_agent("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/87.0.4280.88 Safari/537.36")
         .gzip(true)
         .build()?;

    let request = client.request(
        Method::from_str(&method.to_uppercase()).expect("Invalid method request"),
        full_url.as_str(),
    );

    let request = if let Some(json_body) = body { request.json(json_body) } else { request };
    let response = request.send().await?;

    Ok(response)
}

// // Returns the text in response.
// pub(super) async fn http_get(url: &str, params: &BTreeMap<String, String>) -> Result<String> {
//     match http_raw("GET", url, params, None).await {
//         Ok(response) => match response.error_for_status() {
//             Ok(resp) => Ok(resp.text().await?),
//             Err(error) => Err(Error::from(error)),
//         },
//         Err(err) => Err(err),
//     }
// }

// pub(super) async fn http_post(url: &str, body: Option<&Value>) -> Result<String> {
//     match http_raw("POST", url, &BTreeMap::<String, String>::new(), body).await {
//         Ok(response) => match response.error_for_status() {
//             Ok(resp) => Ok(resp.json().await?),
//             Err(error) => Err(Error::from(error)),
//         },
//         Err(err) => Err(err),
//     }
// }
