use async_trait::async_trait;
use reqwest::RequestBuilder;
use serde::de::DeserializeOwned;

use core::result::Result;

use crate::{
    models::account_model::*,
    request::{self, Request, RequestError},
};

#[async_trait]
pub trait Routes<Success: DeserializeOwned, Error: DeserializeOwned> {
    async fn execute(
        &self,
        request: &Request,
    ) -> Result<request::Response<Success, Error>, RequestError> {
        Ok(request.execute::<Success, Error>(self.build()).await?)
    }

    fn build(&self) -> RequestBuilder {
        let client = reqwest::Client::new();
        client
            .post(format!("http://127.0.0.1:8080/{}", self.get_route()))
            .json(&self.get_json())
    }

    fn get_json(&self) -> serde_json::Value;

    fn get_route(&self) -> String;
}

pub struct Login(pub AccountCredentials);

#[async_trait]
impl Routes<AccountTokenInfos, AccountError> for Login {
    fn get_json(&self) -> serde_json::Value {
        return serde_json::to_value(&self.0).unwrap();
    }

    fn get_route(&self) -> String {
        return "login".to_string();
    }
}
