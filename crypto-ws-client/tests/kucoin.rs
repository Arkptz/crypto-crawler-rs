#[macro_use]
mod utils;

#[cfg(test)]
mod kucoin_spot {
    use crypto_ws_client::{KuCoinSpotWSClient, WSClient};
    use std::sync::mpsc::{Receiver, Sender};

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe() {
        gen_test_code!(
            KuCoinSpotWSClient,
            subscribe,
            &vec![("/market/match".to_string(), "BTC-USDT".to_string())]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_all_bbo() {
        gen_test_code!(
            KuCoinSpotWSClient,
            subscribe,
            &vec![("/market/ticker".to_string(), "all".to_string())]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_raw_json() {
        gen_test_code!(
            KuCoinSpotWSClient,
            send,
            &vec![r#"{"id":"crypto-ws-client","type":"subscribe","topic":"/market/match:BTC-USDT","privateChannel":false,"response":true}"#.to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_trade() {
        gen_test_code!(
            KuCoinSpotWSClient,
            subscribe_trade,
            &vec!["BTC-USDT".to_string(), "ETH-USDT".to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_bbo() {
        gen_test_code!(
            KuCoinSpotWSClient,
            subscribe_bbo,
            &vec!["BTC-USDT".to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_orderbook() {
        gen_test_code!(
            KuCoinSpotWSClient,
            subscribe_orderbook,
            &vec!["BTC-USDT".to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_l3_orderbook() {
        gen_test_code!(
            KuCoinSpotWSClient,
            subscribe_l3_orderbook,
            &vec!["BTC-USDT".to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_orderbook_topk() {
        gen_test_code!(
            KuCoinSpotWSClient,
            subscribe_orderbook_topk,
            &vec!["BTC-USDT".to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_ticker() {
        gen_test_code!(
            KuCoinSpotWSClient,
            subscribe_ticker,
            &vec!["BTC-USDT".to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_candlestick() {
        gen_test_subscribe_candlestick!(KuCoinSpotWSClient, &vec![("BTC-USDT".to_string(), 60)]);
        gen_test_subscribe_candlestick!(
            KuCoinSpotWSClient,
            &vec![("BTC-USDT".to_string(), 604800)]
        );
    }
}

#[cfg(test)]
mod kucoin_inverse_swap {
    use crypto_ws_client::{KuCoinSwapWSClient, WSClient};
    use std::sync::mpsc::{Receiver, Sender};

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe() {
        gen_test_code!(
            KuCoinSwapWSClient,
            subscribe,
            &vec![
                (
                    "/contractMarket/execution".to_string(),
                    "XBTUSDM".to_string()
                ),
                (
                    "/contractMarket/execution".to_string(),
                    "ETHUSDM".to_string()
                )
            ]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_raw_json() {
        gen_test_code!(
            KuCoinSwapWSClient,
            send,
            &vec![r#"{"id":"crypto-ws-client","type":"subscribe","topic":"/contractMarket/execution:XBTUSDM","privateChannel":false,"response":true}"#.to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_trade() {
        gen_test_code!(
            KuCoinSwapWSClient,
            subscribe_trade,
            &vec!["XBTUSDM".to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_bbo() {
        gen_test_code!(
            KuCoinSwapWSClient,
            subscribe_bbo,
            &vec!["XBTUSDM".to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_orderbook() {
        gen_test_code!(
            KuCoinSwapWSClient,
            subscribe_orderbook,
            &vec!["XBTUSDM".to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_l3_orderbook() {
        gen_test_code!(
            KuCoinSwapWSClient,
            subscribe_l3_orderbook,
            &vec!["XBTUSDM".to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_orderbook_topk() {
        gen_test_code!(
            KuCoinSwapWSClient,
            subscribe_orderbook_topk,
            &vec!["XBTUSDM".to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_ticker() {
        gen_test_code!(
            KuCoinSwapWSClient,
            subscribe_ticker,
            &vec!["XBTUSDM".to_string()]
        );
    }

    #[ignore]
    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_candlestick() {
        gen_test_subscribe_candlestick!(
            KuCoinSwapWSClient,
            &vec![("XBTUSDM".to_string(), 60), ("ETHUSDM".to_string(), 60)]
        );
        gen_test_subscribe_candlestick!(
            KuCoinSwapWSClient,
            &vec![
                ("XBTUSDM".to_string(), 604800),
                ("ETHUSDM".to_string(), 604800)
            ]
        );
    }
}

#[cfg(test)]
mod kucoin_linear_swap {
    use crypto_ws_client::{KuCoinSwapWSClient, WSClient};
    use std::sync::mpsc::{Receiver, Sender};

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe() {
        gen_test_code!(
            KuCoinSwapWSClient,
            subscribe,
            &vec![(
                "/contractMarket/execution".to_string(),
                "XBTUSDTM".to_string()
            )]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_raw_json() {
        gen_test_code!(
            KuCoinSwapWSClient,
            send,
            &vec![r#"{"id":"crypto-ws-client","type":"subscribe","topic":"/contractMarket/execution:XBTUSDTM","privateChannel":false,"response":true}"#.to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_trade() {
        gen_test_code!(
            KuCoinSwapWSClient,
            subscribe_trade,
            &vec!["XBTUSDTM".to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_bbo() {
        gen_test_code!(
            KuCoinSwapWSClient,
            subscribe_bbo,
            &vec!["XBTUSDTM".to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_orderbook() {
        gen_test_code!(
            KuCoinSwapWSClient,
            subscribe_orderbook,
            &vec!["XBTUSDTM".to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_l3_orderbook() {
        gen_test_code!(
            KuCoinSwapWSClient,
            subscribe_l3_orderbook,
            &vec!["XBTUSDTM".to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_orderbook_topk() {
        gen_test_code!(
            KuCoinSwapWSClient,
            subscribe_orderbook_topk,
            &vec!["XBTUSDTM".to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_ticker() {
        gen_test_code!(
            KuCoinSwapWSClient,
            subscribe_ticker,
            &vec!["XBTUSDTM".to_string()]
        );
    }

    #[ignore]
    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_candlestick() {
        gen_test_subscribe_candlestick!(
            KuCoinSwapWSClient,
            &vec![("XBTUSDTM".to_string(), 60), ("ETHUSDTM".to_string(), 60)]
        );
        gen_test_subscribe_candlestick!(
            KuCoinSwapWSClient,
            &vec![
                ("XBTUSDTM".to_string(), 604800),
                ("ETHUSDTM".to_string(), 604800)
            ]
        );
    }
}

#[cfg(test)]
mod kucoin_inverse_future {
    use crypto_ws_client::{KuCoinSwapWSClient, WSClient};
    use std::sync::mpsc::{Receiver, Sender};

    #[ignore]
    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe() {
        gen_test_code!(
            KuCoinSwapWSClient,
            subscribe,
            &vec![(
                "/contractMarket/execution".to_string(),
                "XBTMH22".to_string()
            )]
        );
    }

    #[ignore]
    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_raw_json() {
        gen_test_code!(
            KuCoinSwapWSClient,
            send,
            &vec![r#"{"id":"crypto-ws-client","type":"subscribe","topic":"/contractMarket/execution:XBTMH22","privateChannel":false,"response":true}"#.to_string()]
        );
    }

    #[ignore]
    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_trade() {
        gen_test_code!(
            KuCoinSwapWSClient,
            subscribe_trade,
            &vec!["XBTMH22".to_string()]
        );
    }

    #[ignore]
    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_bbo() {
        gen_test_code!(
            KuCoinSwapWSClient,
            subscribe_bbo,
            &vec!["XBTMH22".to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_orderbook() {
        gen_test_code!(
            KuCoinSwapWSClient,
            subscribe_orderbook,
            &vec!["XBTMH22".to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_l3_orderbook() {
        gen_test_code!(
            KuCoinSwapWSClient,
            subscribe_l3_orderbook,
            &vec!["XBTMH22".to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_orderbook_topk() {
        gen_test_code!(
            KuCoinSwapWSClient,
            subscribe_orderbook_topk,
            &vec!["XBTMH22".to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_ticker() {
        gen_test_code!(
            KuCoinSwapWSClient,
            subscribe_ticker,
            &vec!["XBTMH22".to_string()]
        );
    }

    #[ignore]
    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_candlestick() {
        gen_test_subscribe_candlestick!(KuCoinSwapWSClient, &vec![("XBTMH22".to_string(), 60)]);
        gen_test_subscribe_candlestick!(KuCoinSwapWSClient, &vec![("XBTMH22".to_string(), 604800)]);
    }
}
