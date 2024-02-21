use serde::Deserialize;
use thiserror::Error;

#[derive(Debug, Deserialize)]
pub struct BinanceContentError {
    pub code: i16,
    pub msg: String,
}

#[derive(Error, Debug)]
pub enum CustomError {
    #[error("Binance Error")]
    BinanceError(BinanceContentError),
    #[error("{index} at {name} is missing")]
    KlineValueMissingError { index: usize, name: &'static str },
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

    #[error(transparent)]
    Other(#[from] anyhow::Error), // source and Display delegate to anyhow::Error
}
