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
            .post(format!(
                "http://127.0.0.1:8080/api/admin/{}",
                self.get_route()
            ))
            .json(&self.get_json())
    }

    fn get_json(&self) -> serde_json::Value;

    fn get_route(&self) -> String;
}

pub struct Signup(pub AccountUuid);

#[async_trait]
impl Routes<ApiAccountTokenInfos, AccountError> for Signup {
    fn get_json(&self) -> serde_json::Value {
        return serde_json::to_value(&self.0).unwrap();
    }

    fn get_route(&self) -> String {
        return "signup".to_string();
    }
}

pub struct RefreshToken(pub AccountUuid);

#[async_trait]
impl Routes<ApiAccountTokenInfos, AccountError> for RefreshToken {
    fn get_json(&self) -> serde_json::Value {
        return serde_json::to_value(&self.0).unwrap();
    }

    fn get_route(&self) -> String {
        return "refresh-token".to_string();
    }
}

pub struct CheckToken(pub AccountToken);

#[async_trait]
impl Routes<ApiAccountTokenInfos, AccountError> for CheckToken {
    fn get_json(&self) -> serde_json::Value {
        return serde_json::to_value(&self.0).unwrap();
    }

    fn get_route(&self) -> String {
        return "check-token".to_string();
    }
}

pub struct Delete(pub AccountUuid);

#[async_trait]
impl Routes<String, AccountError> for Delete {
    fn get_json(&self) -> serde_json::Value {
        return serde_json::to_value(&self.0).unwrap();
    }

    fn get_route(&self) -> String {
        return "delete".to_string();
    }
}

pub struct ClearTokens(pub AccountUuid);

#[async_trait]
impl Routes<String, AccountError> for ClearTokens {
    fn get_json(&self) -> serde_json::Value {
        return serde_json::to_value(&self.0).unwrap();
    }

    fn get_route(&self) -> String {
        return "clear-tokens".to_string();
    }
}
