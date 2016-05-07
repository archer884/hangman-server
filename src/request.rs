use iron::prelude::*;
use error::ApplicationError;
use router::Router;

pub trait Data {
    fn token(&self) -> Result<&str, ApplicationError>;
    fn guess(&self) -> Result<&str, ApplicationError>;
}

impl<'a, 'b> Data for Request<'a, 'b> {
    fn token(&self) -> Result<&str, ApplicationError> {
        self.extensions.get::<Router>().unwrap().find("token").ok_or(ApplicationError::MissingToken)
    }
    
    fn guess(&self) -> Result<&str, ApplicationError> {
        self.extensions.get::<Router>().unwrap().find("letter").ok_or(ApplicationError::MissingLetter)
    }
}