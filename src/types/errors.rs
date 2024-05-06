#[derive(Debug)]
pub enum CustomError {
    IOError(std::io::Error),
    SerdeJsonError(serde_json::Error),
    ReqwestError(reqwest::Error),
    XmlError(roxmltree::Error),
    ErrorMessage(ErrorMessage),
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

#[derive(Debug)]
pub struct ErrorMessage {
    details: String,
}

impl ErrorMessage {
    fn new(msg: &str) -> ErrorMessage {
        ErrorMessage {
            details: msg.to_string(),
        }
    }
}

impl std::fmt::Display for ErrorMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", &self.details)
    }
}

impl std::error::Error for ErrorMessage {
    fn description(&self) -> &str {
        &self.details
    }
}
