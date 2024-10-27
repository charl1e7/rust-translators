use std::fmt;
use std::fmt::{Debug, Display};
use std::str::Utf8Error;
#[cfg(feature = "tokio-async")]
use tokio::task::JoinError;

// trait
pub trait Translator: Clone + Default + Debug + Send + Sync {
    #[cfg(feature = "tokio-async")]
    async fn translate_async(
        &self,
        text: &str,
        target_language: &str,
        source_language: &str,
    ) -> Result<String, Error>;

    fn translate_sync(
        &self,
        text: &str,
        target_language: &str,
        source_language: &str,
    ) -> Result<String, Error>;
}

// error
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    Builder(String),
    Redirect(String),
    Status(String),
    Timeout(String),
    ConnectFailed(String),
    DecodeBody(String),
    Encoding(String),
    Captcha(String),
    InvalidRequest(String),
    #[cfg(feature = "tokio-async")]
    JoinTask(String),
    Uknown(String),
}

impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Builder(ref e) => write!(f, "Builder error: {}", e),
            Error::Redirect(ref e) => write!(f, "Redirect error: {}", e),
            Error::Status(ref e) => write!(f, "Status error: {}", e),
            Error::Timeout(ref e) => write!(f, "Timeout error: {}", e),
            Error::ConnectFailed(ref e) => write!(f, "ConnectFailed error: {}", e),
            Error::DecodeBody(ref e) => write!(f, "Body decoding error: {}", e),
            Error::Captcha(ref e) => write!(f, "Captcha: {}", e),
            Error::Encoding(ref e) => write!(f, "Encoding error: {}", e),
            Error::InvalidRequest(ref e) => write!(f, "Invalid request: {}", e),
            #[cfg(feature = "tokio-async")]
            Error::JoinTask(ref e) => write!(f, "Tokio join task error: {}", e),
            Error::Uknown(ref e) => write!(f, "Unknown error: {}", e),
        }
    }
}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        if e.is_connect() {
            Error::ConnectFailed(e.to_string())
        } else if e.is_timeout() {
            Error::Timeout(e.to_string())
        } else if e.is_builder() {
            Error::Builder(e.to_string())
        } else if e.is_redirect() {
            Error::Redirect(e.to_string())
        } else if e.is_status() {
            Error::Status(e.to_string())
        } else if e.is_request() {
            Error::InvalidRequest(e.to_string())
        } else if e.is_decode() {
            Error::DecodeBody(e.to_string())
        } else {
            Error::Uknown(e.to_string())
        }
    }
}

impl From<Utf8Error> for Error {
    fn from(e: Utf8Error) -> Self {
        Error::Encoding(e.to_string())
    }
}

#[cfg(feature = "tokio-async")]
impl From<JoinError> for Error {
    fn from(e: JoinError) -> Self {
        Error::JoinTask(e.to_string())
    }
}
