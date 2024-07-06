use std::error::Error;
use std::fmt;
use std::str::Utf8Error;

#[derive(Debug)]
pub enum GoogleError {
    Captcha,
    InvalidRequest(String),
    EncodingError(String),
}
// Captcha prevents the request
impl Error for GoogleError {}

impl fmt::Display for GoogleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            GoogleError::Captcha => write!(f, "Captcha prevents the request, change the config."),
            GoogleError::InvalidRequest(ref e) => write!(f, "Invalid request: {}", e),
            GoogleError::EncodingError(ref e) => write!(f, "Encoding error: {}", e), // Display for EncodingError
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
