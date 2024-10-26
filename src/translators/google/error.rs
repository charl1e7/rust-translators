use std::error::Error;
use std::fmt;
use std::str::Utf8Error;

#[cfg(feature = "tokio-async")]
use tokio::task::JoinError;

#[derive(Debug, Clone)]
pub enum GoogleError {
    InvalidRequest(String),
    EncodingError(String),
    #[cfg(feature = "tokio-async")]
    TokioJoinError(String),
}
// Captcha prevents the request
impl Error for GoogleError {}

impl fmt::Display for GoogleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            GoogleError::InvalidRequest(ref e) => write!(f, "Invalid request: {}", e),
            GoogleError::EncodingError(ref e) => write!(f, "Encoding error: {}", e),
            #[cfg(feature = "tokio-async")]
            GoogleError::TokioJoinError(ref e) => write!(f, "Tokio join error: {}", e),
        }
    }
}

impl From<reqwest::Error> for GoogleError {
    fn from(error: reqwest::Error) -> Self {
        GoogleError::InvalidRequest(error.to_string())
    }
}

impl From<Utf8Error> for GoogleError {
    fn from(error: Utf8Error) -> Self {
        GoogleError::EncodingError(error.to_string())
    }
}

#[cfg(feature = "tokio-async")]
impl From<JoinError> for GoogleError {
    fn from(error: JoinError) -> Self {
        GoogleError::TokioJoinError(error.to_string())
    }
}
