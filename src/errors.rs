use serde::Deserialize;
use thiserror::Error;

#[derive(Debug, Deserialize)]
pub struct BinanceContentError {
    pub code: i16,
    pub msg: String,
}

#[derive(Error, Debug)]
pub enum ErrorKind {
    #[error("Binance Error")]
    BinanceError(BinanceContentError),
    #[error("{0} at {1} is missing")]
    KlineValueMissingError(usize, &'static str),
    #[error("ReqError: {0}")]
    ReqError(#[from] reqwest::Error),
    #[error("InvalidHeaderError: {0}")]
    InvalidHeaderError(#[from] reqwest::header::InvalidHeaderValue),
    #[error("IoError: {0}")]
    IoError(#[from] std::io::Error),
    #[error("ParseFloatError: {0}")]
    ParseFloatError(#[from] std::num::ParseFloatError),
    #[error("UrlParserError: {0}")]
    UrlParserError(#[from] url::ParseError),
    #[error("Json: {0}")]
    Json(#[from] serde_json::Error),
    #[error("Tungstenite: {0}")]
    Tungstenite(#[from] tungstenite::Error),
    #[error("TimestampError: {0}")]
    TimestampError(#[from] std::time::SystemTimeError),
}

// error_chain! {
//     errors {
//         BinanceError(response: BinanceContentError)
//
//         KlineValueMissingError(index: usize, name: &'static str) {
//             description("invalid Vec for Kline"),
//             display("{} at {} is missing", name, index),
//         }
//      }
//
//     foreign_links {
//         ReqError(reqwest::Error);
//         InvalidHeaderError(reqwest::header::InvalidHeaderValue);
//         IoError(std::io::Error);
//         ParseFloatError(std::num::ParseFloatError);
//         UrlParserError(url::ParseError);
//         Json(serde_json::Error);
//         Tungstenite(tungstenite::Error);
//         TimestampError(std::time::SystemTimeError);
//     }
// }
