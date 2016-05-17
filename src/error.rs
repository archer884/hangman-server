use std::fmt;
use std::error::Error;
use hangman_data::service::ServiceError;
use iron::prelude::*;
use iron::status;

#[derive(Debug)]
pub enum ApplicationError {
    UrlParam { name: String, message: String },
    Deserialization { message: String, inner: String },
    Db(Box<Error + Send>),
}

impl ApplicationError {
    pub fn url_param<T: Into<String>>(name: T, message: T) -> ApplicationError {
        ApplicationError::UrlParam {
            name: name.into(),
            message: message.into(),
        }
    }

    pub fn deserialization<T: Into<String>>(message: T, inner: T) -> ApplicationError {
        ApplicationError::Deserialization {
            message: message.into(),
            inner: inner.into(),
        }
    }
}

impl fmt::Display for ApplicationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for ApplicationError {
    fn description(&self) -> &str {
        match *self {
            ApplicationError::UrlParam { .. } => "Bad or missing url parameter",
            ApplicationError::Deserialization { .. } => "Unable to deserialize payload",
        }
    }
}

impl From<ApplicationError> for IronError {
    fn from(error: ApplicationError) -> IronError {
        IronError {
            response: Response::with((status::BadRequest, error.description())),
            error: box error,
        }
    }
}

impl From<ServiceError> for ApplicationError {
    fn from(error: ServiceError) -> ApplicationError {
        ApplicationError::Db(box error)
    }
}