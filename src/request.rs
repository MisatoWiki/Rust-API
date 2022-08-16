use reqwest::{header::HeaderMap, *};
use serde::{de::DeserializeOwned, Deserialize, Serialize};

use core::result::Result;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Copy)]
#[serde(untagged)]
pub enum Response<Success, Error> {
    Success(Success),
    Error(Error),
}

impl<Success, Error> Response<Success, Error> {
    pub fn is_err(&self) -> bool {
        match self {
            Self::Success(_) => false,
            Self::Error(_) => true,
        }
    }

    pub fn is_ok(&self) -> bool {
        match self {
            Self::Success(_) => true,
            Self::Error(_) => false,
        }
    }

    pub fn unwrap(self) -> Success {
        match self {
            Self::Success(value) => value,
            Self::Error(_) => panic!("Response does not contain Success value."),
        }
    }

    pub fn unwrap_ok(self) -> Success {
        match self {
            Self::Success(value) => value,
            Self::Error(_) => panic!("Response does not contain Success value."),
        }
    }

    pub fn unwrap_err(self) -> Error {
        match self {
            Self::Success(_) => panic!("Response does not contain Error value."),
            Self::Error(value) => value,
        }
    }
}

#[derive(Debug)]
pub enum RequestErrors {
    RequestError,
    TextParsingError,
    ResponseTypeIncorrect,
}

#[derive(Debug)]
pub struct RequestError {
    pub reason: RequestErrors,
}

pub struct Request {
    usertoken: Option<String>,
    apitoken: Option<String>,
}

impl Request {
    pub async fn execute<Success: DeserializeOwned, Error: DeserializeOwned>(
        &self,
        request: RequestBuilder,
    ) -> Result<Response<Success, Error>, RequestError> {
        let response = request.headers(self.get_headers()).send().await;

        if response.is_err() {
            return Err(RequestError {
                reason: RequestErrors::RequestError,
            });
        }

        let response = response.unwrap();

        let text = response.text().await;

        if text.is_err() {
            return Err(RequestError {
                reason: RequestErrors::TextParsingError,
            });
        }

        let text = text.unwrap();

        match serde_json::from_str::<Response<Success, Error>>(&text) {
            Ok(result) => Ok(result),
            Err(_) => {
                return Err(RequestError {
                    reason: RequestErrors::ResponseTypeIncorrect,
                })
            }
        }
    }

    fn get_headers(&self) -> HeaderMap {
        let mut headers = HeaderMap::new();
        if self.usertoken.is_some() {
            headers.append(
                "X-Misato-User-Token",
                self.usertoken.as_ref().unwrap().parse().unwrap(),
            );
        }
        if self.apitoken.is_some() {
            headers.append(
                "X-Misato-API-Token",
                self.apitoken.as_ref().unwrap().parse().unwrap(),
            );
        }
        headers
    }

    pub fn new(usertoken: Option<String>, apitoken: Option<String>) -> Self {
        Self {
            usertoken,
            apitoken,
        }
    }
}
