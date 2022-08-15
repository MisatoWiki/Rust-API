use serde::{Deserialize, Serialize};

#[derive(Eq, Hash, PartialEq, Debug, Serialize, Deserialize, Default, Clone)]
pub struct Account {
    pub uuid: String,
    pub username: String,
    pub token: String,
}

#[derive(Serialize, Deserialize, Default, Debug, PartialEq)]
pub struct AccountCredentials {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Default, Debug, PartialEq)]
pub struct AccountUuid {
    pub uuid: String,
}

#[derive(Serialize, Deserialize, Default, Debug, PartialEq)]
pub struct AccountUsername {
    pub username: String,
}

#[derive(Serialize, Deserialize, Default, Debug, PartialEq)]
pub struct AccountToken {
    pub token: String,
}

#[derive(Serialize, Deserialize, Default, Debug, PartialEq)]
pub struct AccountTokenInfos {
    pub token: String,
    pub timestamp: u64,
    pub expiration_timestamp: u64,
}

#[derive(Serialize, Deserialize, Default, Debug, PartialEq)]
pub struct AccountError {
    // HTTP Status Code returned
    pub code: u16,
    // Reason for an error
    pub reason: String,
    // Description for an error if any
    pub description: Option<String>,
}

impl AccountError {
    // building a custom error.
    pub fn build(code: u16, description: Option<String>) -> Self {
        let reason: String;
        match code {
            400 => reason = "Bad Request".to_string(),
            401 => reason = "Unauthorized".to_string(),
            _ => reason = "Error".to_string(),
        }
        Self {
            code,
            reason,
            description,
        }
    }
}
