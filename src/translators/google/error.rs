use std::error::Error;
use std::fmt;
use std::str::Utf8Error;
use tokio::task::JoinError;

#[derive(Debug)]
pub enum GoogleError {
    InvalidRequest(String),
    EncodingError(String),
    TokioJoinError(String),

}
// Captcha prevents the request
impl Error for GoogleError {}

impl fmt::Display for GoogleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            GoogleError::InvalidRequest(ref e) => write!(f, "Invalid request: {}", e),
            GoogleError::EncodingError(ref e) => write!(f, "Encoding error: {}", e),
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

impl From<JoinError> for GoogleError {
    fn from(error: JoinError) -> Self {
        GoogleError::TokioJoinError(error.to_string())
    }
}