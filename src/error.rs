use std::fmt;
use std::error::Error;
use iron::prelude::*;
use iron::status;

#[derive(Debug)]
pub enum ApplicationError {
    MissingToken,
    MissingLetter,
    InvalidGuess,
}

impl fmt::Display for ApplicationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for ApplicationError {
    fn description(&self) -> &str {
        match *self {
            ApplicationError::MissingToken => "Missing required token",
            ApplicationError::MissingLetter => "Missing required letter",
            ApplicationError::InvalidGuess => "Guesses must be ascii",
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