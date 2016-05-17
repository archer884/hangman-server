use std::error::Error;
use iron::prelude::*;
use error::ApplicationError;
use router::Router;
use serde::Deserialize;
use serde_json;

pub mod model;

pub trait RequestData {
    fn id(&self) -> Result<i64, ApplicationError>;
    fn token(&self) -> Result<&str, ApplicationError>;
    fn page(&self) -> Result<i64, ApplicationError>;
    fn body<T>(&mut self) -> Result<T, ApplicationError>
        where T: Deserialize;
}

impl<'a, 'b> RequestData for Request<'a, 'b> {
    fn id(&self) -> Result<i64, ApplicationError> {
        let id = self.extensions.get::<Router>().unwrap().find("id").ok_or(
            ApplicationError::url_param("id", "missing parameter")
        )?;

        id.parse::<i64>().map_err(|e| ApplicationError::url_param(
            "id",
            e.description()
        ))
    }

    fn token(&self) -> Result<&str, ApplicationError> {
        self.extensions.get::<Router>().unwrap().find("token").ok_or(
            ApplicationError::url_param("token", "missing parameter")
        )
    }

    fn page(&self) -> Result<i64, ApplicationError> {
        let page = self.extensions.get::<Router>().unwrap().find("page").ok_or(
            ApplicationError::url_param("page", "missing parameter")
        )?;

        page.parse::<i64>().map_err(|e| ApplicationError::url_param(
            "page",
            e.description()
        ))
    }

    fn body<T: Deserialize>(&mut self) -> Result<T, ApplicationError> {
        use std::io::Read;

        let body = {
            let mut buf = String::new();
            self.body.read_to_string(&mut buf).map_err(|e| ApplicationError::deserialization(
                "Unable to parse body as UTF8",
                e.description(),
            ))?;
            buf
        };

        serde_json::from_str(&body).map_err(|e| ApplicationError::deserialization(
            "Unable to deserialize",
            e.description(),
        ))
    }
}