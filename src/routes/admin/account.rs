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
            .post(format!("http://127.0.0.1:8080/admin/{}", self.get_route()))
            .json(&self.get_json())
    }

    fn get_json(&self) -> serde_json::Value;

    fn get_route(&self) -> String;
}

/// Admin signup route
/// example.com/admin/signup
///
/// # Examples
///
/// Basic usage:
///
/// ```rust, no_run
/// use misato::{models::account_model::*, request::{Response, Request}, routes::admin::account::{Routes, Signup}};
///
/// let apitoken = "SOME_TOKEN";
/// let request = Request::new(None, Some(apitoken.to_string()));
///
/// let credentials = AccountCredentials {
///     username: "SomeUsername".to_string(),
///     password: "SomePassword".to_string(),
/// };
/// let result = tokio_test::block_on(async { Signup(credentials).execute(&request).await });
/// // Check if the request has been successfuly performed.
/// assert_eq!(result.is_ok(), true);
///
/// let result: Response<AccountTokenInfos, AccountError> = result.unwrap();
/// // Check if the Rest API allowed our request.
/// assert_eq!(result.is_ok(), true);
///
/// // The account has been signed up, the REST Api returned his TokenInfos.
/// let _tokeninfos: AccountTokenInfos = result.unwrap();
///
/// ```
pub struct Signup(pub AccountCredentials);

#[async_trait]
impl Routes<AccountTokenInfos, AccountError> for Signup {
    fn get_json(&self) -> serde_json::Value {
        return serde_json::to_value(&self.0).unwrap();
    }

    fn get_route(&self) -> String {
        return "signup".to_string();
    }
}

pub struct Profile(pub AccountUuid);

#[async_trait]
impl Routes<Account, AccountError> for Profile {
    fn get_json(&self) -> serde_json::Value {
        return serde_json::to_value(&self.0).unwrap();
    }

    fn get_route(&self) -> String {
        return "profile".to_string();
    }
}

pub struct RefreshToken(pub AccountUuid);

#[async_trait]
impl Routes<AccountTokenInfos, AccountError> for RefreshToken {
    fn get_json(&self) -> serde_json::Value {
        return serde_json::to_value(&self.0).unwrap();
    }

    fn get_route(&self) -> String {
        return "refresh-token".to_string();
    }
}

pub struct CheckToken(pub AccountToken);

#[async_trait]
impl Routes<AccountTokenInfos, AccountError> for CheckToken {
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
