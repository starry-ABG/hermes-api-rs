use reqwest::StatusCode;
use thiserror::Error;

/// HermesError 封装了调用过程中可能出现的各种错误
#[derive(Error, Debug)]
pub enum HermesError {
    #[error("HTTP request failed: {0}")]
    ReqwestError(#[from] reqwest::Error),

    #[error("Unexpected response status: {status}, body: {body}")]
    ResponseError {
        status: StatusCode,
        body: String,
    },

    #[error("Invalid input: {0}")]
    InvalidInput(String),

    #[error("Other error: {0}")]
    Other(String),
}