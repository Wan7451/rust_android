use std::fmt::{Debug, Display, Formatter};


/// 不同类型的错误可以通过组合使用
/// std::error::Error 和 std::convert::From / std::convert::Into 特质来转换为一个统一的错误类型。
pub enum Error {
    CustomError(String),
    SerdeError(serde_json::Error),
    HttpError(reqwest::Error),
}

impl From<serde_json::Error> for Error {
    fn from(value: serde_json::Error) -> Self {
        Error::SerdeError(value)
    }
}

impl From<reqwest::Error> for Error {
    fn from(value: reqwest::Error) -> Self {
        Error::HttpError(value)
    }
}

impl Debug for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::CustomError(e) => write!(f, "{}", e),
            Error::SerdeError(e) => write!(f, "{}", e),
            Error::HttpError(e) => write!(f, "{}", e),
            _ => write!(f, "unknown error"),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::CustomError(_) => None,
            Error::SerdeError(e) => Some(e),
            Error::HttpError(e) => Some(e),
        }
    }
}

