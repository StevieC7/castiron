#[derive(Debug)]
pub enum CustomError {
    IOError(std::io::Error),
    ReqwestError(reqwest::Error),
    SerdeJsonError(serde_json::Error),
    XmlError(roxmltree::Error),
    SqlError(sqlite::Error),
    ParseError(url::ParseError),
    TimeParseError(time::error::Parse),
    Empty(()),
}

impl From<reqwest::Error> for CustomError {
    fn from(err: reqwest::Error) -> Self {
        CustomError::ReqwestError(err)
    }
}

impl From<roxmltree::Error> for CustomError {
    fn from(err: roxmltree::Error) -> Self {
        CustomError::XmlError(err)
    }
}

impl From<serde_json::Error> for CustomError {
    fn from(err: serde_json::Error) -> Self {
        CustomError::SerdeJsonError(err)
    }
}

impl From<std::io::Error> for CustomError {
    fn from(err: std::io::Error) -> Self {
        CustomError::IOError(err)
    }
}

impl From<sqlite::Error> for CustomError {
    fn from(err: sqlite::Error) -> Self {
        CustomError::SqlError(err)
    }
}

impl From<url::ParseError> for CustomError {
    fn from(err: url::ParseError) -> Self {
        CustomError::ParseError(err)
    }
}

impl From<time::error::Parse> for CustomError {
    fn from(err: time::error::Parse) -> Self {
        CustomError::TimeParseError(err)
    }
}

impl From<()> for CustomError {
    fn from(err: ()) -> Self {
        CustomError::Empty(err)
    }
}
