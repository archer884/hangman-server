use std::ascii::AsciiExt;
use iron::prelude::*;
use error::ApplicationError;
use router::Router;

pub trait Data {
    fn token(&self) -> Result<&str, ApplicationError>;
    fn guess(&self) -> Result<String, ApplicationError>;
}

impl<'a, 'b> Data for Request<'a, 'b> {
    fn token(&self) -> Result<&str, ApplicationError> {
        self.extensions.get::<Router>().unwrap().find("token").ok_or(ApplicationError::MissingToken)
    }

    fn guess(&self) -> Result<String, ApplicationError> {
        let word = self.extensions.get::<Router>().unwrap().find("letter").ok_or(ApplicationError::MissingLetter)?;
        if word.is_ascii() {
            Ok(word.to_ascii_lowercase())
        } else {
            Err(ApplicationError::InvalidGuess)
        }
    }
}