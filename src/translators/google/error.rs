use std::error::Error;
use std::fmt;
use std::str::Utf8Error;

#[cfg(feature = "tokio-async")]
use tokio::task::JoinError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GoogleError {
    Builder(String),
    Redirect(String),
    Status(String),
    Timeout(String),
    ConnectFailed(String),
    DecodeBody(String),
    Encoding(String),
    InvalidRequest(String),
    #[cfg(feature = "tokio-async")]
    JoinTask(String),
    Uknown(String),
}

impl Error for GoogleError {}

impl fmt::Display for GoogleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            GoogleError::Builder(ref e) => write!(f, "Builder error: {}", e),
            GoogleError::Redirect(ref e) => write!(f, "Redirect error: {}", e),
            GoogleError::Status(ref e) => write!(f, "Status error: {}", e),
            GoogleError::Timeout(ref e) => write!(f, "Timeout error: {}", e),
            GoogleError::ConnectFailed(ref e) => write!(f, "ConnectFailed error: {}", e),
            GoogleError::DecodeBody(ref e) => write!(f, "Body decoding error: {}", e),
            GoogleError::Encoding(ref e) => write!(f, "Encoding error: {}", e),
            GoogleError::InvalidRequest(ref e) => write!(f, "Invalid request: {}", e),
            #[cfg(feature = "tokio-async")]
            GoogleError::JoinTask(ref e) => write!(f, "Tokio join task error: {}", e),
            GoogleError::Uknown(ref e) => write!(f, "Unknown error: {}", e),
        }
    }
}

impl From<reqwest::Error> for GoogleError {
    fn from(e: reqwest::Error) -> Self {
        if e.is_connect() {
            GoogleError::ConnectFailed(e.to_string())
        } else if e.is_timeout() {
            GoogleError::Timeout(e.to_string())
        } else if e.is_builder() {
            GoogleError::Builder(e.to_string())
        } else if e.is_redirect() {
            GoogleError::Redirect(e.to_string())
        } else if e.is_status() {
            GoogleError::Status(e.to_string())
        } else if e.is_request() {
            GoogleError::InvalidRequest(e.to_string())
        } else if e.is_decode() {
            GoogleError::DecodeBody(e.to_string())
        } else {
            GoogleError::Uknown(e.to_string())
        }
    }
}

impl From<Utf8Error> for GoogleError {
    fn from(e: Utf8Error) -> Self {
        GoogleError::Encoding(e.to_string())
    }
}

#[cfg(feature = "tokio-async")]
impl From<JoinError> for GoogleError {
    fn from(e: JoinError) -> Self {
        GoogleError::JoinTask(e.to_string())
    }
}
