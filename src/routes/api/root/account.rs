use async_trait::async_trait;
use reqwest::RequestBuilder;
use serde::de::DeserializeOwned;

use core::result::Result;

use crate::{
    models::{account_model::*, apiaccount_model::ApiAccountTokenInfos},
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
            .post(format!("http://127.0.0.1:8080/api/{}", self.get_route()))
            .json(&self.get_json())
    }

    fn get_json(&self) -> serde_json::Value;

    fn get_route(&self) -> String;
}

pub struct Signup();

#[async_trait]
impl Routes<ApiAccountTokenInfos, AccountError> for Signup {
    fn get_json(&self) -> serde_json::Value {
        return serde_json::Value::Null;
    }

    fn get_route(&self) -> String {
        return "signup".to_string();
    }
}

pub struct RefreshToken();

#[async_trait]
impl Routes<ApiAccountTokenInfos, AccountError> for RefreshToken {
    fn get_json(&self) -> serde_json::Value {
        return serde_json::Value::Null;
    }

    fn get_route(&self) -> String {
        return "refresh-token".to_string();
    }
}

pub struct CheckToken();

#[async_trait]
impl Routes<ApiAccountTokenInfos, AccountError> for CheckToken {
    fn get_json(&self) -> serde_json::Value {
        return serde_json::Value::Null;
    }

    fn get_route(&self) -> String {
        return "check-token".to_string();
    }
}

pub struct Delete();

#[async_trait]
impl Routes<String, AccountError> for Delete {
    fn get_json(&self) -> serde_json::Value {
        return serde_json::Value::Null;
    }

    fn get_route(&self) -> String {
        return "delete".to_string();
    }
}

pub struct ClearTokens();

#[async_trait]
impl Routes<String, AccountError> for ClearTokens {
    fn get_json(&self) -> serde_json::Value {
        return serde_json::Value::Null;
    }

    fn get_route(&self) -> String {
        return "clear-tokens".to_string();
    }
}
