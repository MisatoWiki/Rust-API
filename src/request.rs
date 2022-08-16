use reqwest::{header::HeaderMap, *};
use serde::{de::DeserializeOwned, Deserialize, Serialize};

use core::result::Result;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(untagged)]
pub enum Response<Success, Error> {
    Success(Success),
    Error(Error),
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
