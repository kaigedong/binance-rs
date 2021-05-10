use crate::account::*;
use crate::client::*;
use crate::config::*;
use crate::futures::account::FuturesAccount;
use crate::futures::general::*;
use crate::futures::market::*;
use crate::general::*;
use crate::market::*;
use crate::userstream::*;
use crate::client::*;

#[allow(clippy::all)]
pub enum API {
    Spot(Spot),
    Futures(Futures),
}

/// Endpoint for production and test orders.
///
/// Orders issued to test are validated, but not sent into the matching engine.
pub enum Spot {
    Ping,
    Time,
    ExchangeInfo,
    Depth,
    Trades,
    HistoricalTrades,
    AggTrades,
    Klines,
    AvgPrice,
    Ticker24hr,
    Price,
    BookTicker,
    Order,
    OrderTest,
    OpenOrders,
    AllOrders,
    Oco,
    OrderList,
    AllOrderList,
    OpenOrderList,
    Account,
    MyTrades,
    UserDataStream,
}

pub enum Futures {
    Ping,
    Time,
    ExchangeInfo,
    Depth,
    Trades,
    HistoricalTrades,
    AggTrades,
    Klines,
    ContinuousKlines,
    IndexPriceKlines,
    MarkPriceKlines,
    PremiumIndex,
    FundingRate,
    Ticker24hr,
    TickerPrice,
    BookTicker,
    AllForceOrders,
    AllOpenOrders,
    Order,
    PositionSide,
    OpenInterest,
    OpenInterestHist,
    TopLongShortAccountRatio,
    TopLongShortPositionRatio,
    GlobalLongShortAccountRatio,
    TakerlongshortRatio,
    LvtKlines,
    IndexInfo,
}

impl From<API> for String {
    fn from(item: API) -> Self {
        String::from(match item {
            API::Spot(route) => {
                match route {
                    Spot::Ping => "/api/v3/ping",
                    Spot::Time => "/api/v3/time",
                    Spot::ExchangeInfo => "/api/v3/exchangeInfo",
                    Spot::Depth => "/api/v3/depth",
                    Spot::Trades => "/api/v3/trades",
                    Spot::HistoricalTrades => "/api/v3/historicalTrades",
                    Spot::AggTrades => "/api/v3/aggTrades",
                    Spot::Klines => "/api/v3/klines",
                    Spot::AvgPrice => "/api/v3/avgPrice",
                    Spot::Ticker24hr => "/api/v3/ticker/24hr",
                    Spot::Price => "/api/v3/ticker/price",
                    Spot::BookTicker => "/api/v3/ticker/bookTicker",
                    Spot::Order => "/api/v3/order",
                    Spot::OrderTest => "/api/v3/order/test",
                    Spot::OpenOrders => "/api/v3/openOrders",
                    Spot::AllOrders => "/api/v3/allOrders",
                    Spot::Oco => "/api/v3/order/oco",
                    Spot::OrderList => "/api/v3/orderList",
                    Spot::AllOrderList => "/api/v3/allOrderList",
                    Spot::OpenOrderList => "/api/v3/openOrderList",
                    Spot::Account => "/api/v3/account",
                    Spot::MyTrades => "/api/v3/myTrades",
                    Spot::UserDataStream => "/api/v3/userDataStream",
                }
            },
            API::Futures(route) => {
                match route {
                    Futures::Ping => "/fapi/v1/ping",
                    Futures::Time => "/fapi/v1/time",
                    Futures::ExchangeInfo => "/fapi/v1/exchangeInfo",
                    Futures::Depth => "/fapi/v1/depth",
                    Futures::Trades => "/fapi/v1/trades",
                    Futures::HistoricalTrades => "/fapi/v1/historicalTrades",
                    Futures::AggTrades => "/fapi/v1/aggTrades",
                    Futures::Klines => "/fapi/v1/klines",
                    Futures::ContinuousKlines => "/fapi/v1/continuousKlines",
                    Futures::IndexPriceKlines => "/fapi/v1/indexPriceKlines",
                    Futures::MarkPriceKlines => "/fapi/v1/markPriceKlines",
                    Futures::PremiumIndex => "/fapi/v1/premiumIndex",
                    Futures::FundingRate => "/fapi/v1/fundingRate",
                    Futures::Ticker24hr => "/fapi/v1/ticker/24hr",
                    Futures::TickerPrice => "/fapi/v1/ticker/price",
                    Futures::BookTicker => "/fapi/v1/ticker/bookTicker",
                    Futures::AllForceOrders => "/fapi/v1/allForceOrders",
                    Futures::AllOpenOrders => "/fapi/v1/allOpenOrders",
                    Futures::PositionSide => "/fapi/v1/positionSide/dual",
                    Futures::Order => "/fapi/v1/order",
                    Futures::OpenInterest => "/fapi/v1/openInterest",
                    Futures::OpenInterestHist => "/futures/data/openInterestHist",
                    Futures::TopLongShortAccountRatio => "/futures/data/topLongShortAccountRatio",
                    Futures::TopLongShortPositionRatio => "/futures/data/topLongShortPositionRatio",
                    Futures::GlobalLongShortAccountRatio => "/futures/data/globalLongShortAccountRatio",
                    Futures::TakerlongshortRatio => "/futures/data/takerlongshortRatio",
                    Futures::LvtKlines => "/fapi/v1/lvtKlines",
                    Futures::IndexInfo => "/fapi/v1/indexInfo",
                }
            }
        })
    }
}

pub trait Binance {
    fn new(api_key: Option<String>, secret_key: Option<String>) -> Self;
    fn new_with_config(
        api_key: Option<String>, secret_key: Option<String>, config: &Config,
    ) -> Self;
}

impl Binance for General {
    fn new(api_key: Option<String>, secret_key: Option<String>) -> General {
        Self::new_with_config(api_key, secret_key, &Config::default())
    }

    fn new_with_config(
        api_key: Option<String>, secret_key: Option<String>, config: &Config,
    ) -> General {
        General {
            client: Client::new(api_key, secret_key, config.rest_api_endpoint.clone()),
        }
    }
}

impl Binance for Account {
    fn new(api_key: Option<String>, secret_key: Option<String>) -> Account {
        Self::new_with_config(api_key, secret_key, &Config::default())
    }

    fn new_with_config(
        api_key: Option<String>, secret_key: Option<String>, config: &Config,
    ) -> Account {
        Account {
            client: Client::new(api_key, secret_key, config.rest_api_endpoint.clone()),
            recv_window: config.recv_window,
        }
    }
}

impl Binance for Market {
    fn new(api_key: Option<String>, secret_key: Option<String>) -> Market {
        Self::new_with_config(api_key, secret_key, &Config::default())
    }

    fn new_with_config(
        api_key: Option<String>, secret_key: Option<String>, config: &Config,
    ) -> Market {
        Market {
            client: Client::new(api_key, secret_key, config.rest_api_endpoint.clone()),
            recv_window: config.recv_window,
        }
    }
}

impl Binance for UserStream {
    fn new(api_key: Option<String>, secret_key: Option<String>) -> UserStream {
        Self::new_with_config(api_key, secret_key, &Config::default())
    }

    fn new_with_config(
        api_key: Option<String>, secret_key: Option<String>, config: &Config,
    ) -> UserStream {
        UserStream {
            client: Client::new(api_key, secret_key, config.rest_api_endpoint.clone()),
            recv_window: config.recv_window,
        }
    }
}

// *****************************************************
//              Binance Futures API
// *****************************************************

impl Binance for FuturesGeneral {
    fn new(api_key: Option<String>, secret_key: Option<String>) -> FuturesGeneral {
        Self::new_with_config(api_key, secret_key, &Config::default())
    }

    fn new_with_config(
        api_key: Option<String>, secret_key: Option<String>, config: &Config,
    ) -> FuturesGeneral {
        FuturesGeneral {
            client: Client::new(api_key, secret_key, config.futures_rest_api_endpoint.clone()),
        }
    }
}

impl Binance for FuturesMarket {
    fn new(api_key: Option<String>, secret_key: Option<String>) -> FuturesMarket {
        Self::new_with_config(api_key, secret_key, &Config::default())
    }

    fn new_with_config(
        api_key: Option<String>, secret_key: Option<String>, config: &Config,
    ) -> FuturesMarket {
        FuturesMarket {
            client: Client::new(api_key, secret_key, config.futures_rest_api_endpoint.clone()),
            recv_window: config.recv_window,
        }
    }
}

impl Binance for FuturesAccount {
    fn new(api_key: Option<String>, secret_key: Option<String>) -> Self {
        Self::new_with_config(api_key, secret_key, &Config::default())
    }

    fn new_with_config(api_key: Option<String>, secret_key: Option<String>, config: &Config) -> Self {
        Self {
            client: Client::new(api_key, secret_key, config.futures_rest_api_endpoint.clone()),
            recv_window: config.recv_window,
        }
    }
}
