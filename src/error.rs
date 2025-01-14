use reqwest;
use std::error;
use std::fmt::{Display, Formatter, Result as FmtResult};
use url;

#[derive(Debug)]
pub enum Error {
    NetworkError(reqwest::Error),
    UrlParseError(url::ParseError),
    Unexpected,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match *self {
            Error::NetworkError(ref e) => write!(f, "NetworkError:  {}", e),
            Error::UrlParseError(ref e) => write!(f, "UrlParseError:  {}", e),
            Error::Unexpected => write!(f, "UnexpectedError"),
        }
    }
}

impl From<url::ParseError> for Error {
    fn from(err: url::ParseError) -> Error {
        Error::UrlParseError(err)
    }
}
impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Error {
        Error::NetworkError(err)
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        ""
    }
}
