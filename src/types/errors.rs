#[derive(Debug)]
pub enum CustomError {
    IOError(std::io::Error),
    ReqwestError(reqwest::Error),
    SerdeJsonError(serde_json::Error),
    XmlError(roxmltree::Error),
    SqlError(sqlite::Error),
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
