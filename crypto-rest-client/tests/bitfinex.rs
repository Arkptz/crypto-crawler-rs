use crypto_rest_client::BitfinexRestClient;

#[test]
fn test_spot_symbols() {
    let symbols = BitfinexRestClient::fetch_spot_symbols().unwrap();
    assert!(!symbols.is_empty());
}

#[test]
fn test_swap_symbols() {
    let symbols = BitfinexRestClient::fetch_swap_symbols().unwrap();
    assert!(!symbols.is_empty());
}

#[test]
fn test_trades() {
    let text = BitfinexRestClient::fetch_trades("tBTCUSD", None, None, None, None).unwrap();
    assert!(text.starts_with("[["));
}

#[test]
fn test_l2_snapshot() {
    let text = BitfinexRestClient::fetch_l2_snapshot("tBTCUSD").unwrap();
    assert!(text.starts_with("[["));
}

#[test]
fn test_l3_snapshot() {
    let text = BitfinexRestClient::fetch_l3_snapshot("tBTCUSD").unwrap();
    assert!(text.starts_with("[["));
}
