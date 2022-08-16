use serde::{Deserialize, Serialize};

#[derive(Eq, Hash, PartialEq, Debug, Serialize, Deserialize, Default, Clone)]
pub struct ApiAccount {
    pub uuid: String,
    pub username: String,
}

#[derive(Serialize, Deserialize, Default, Debug, PartialEq)]
pub struct ApiAccountCredentials {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Default, Debug, PartialEq)]
pub struct ApiAccountUuid {
    pub uuid: String,
}

#[derive(Serialize, Deserialize, Default, Debug, PartialEq)]
pub struct ApiAccountUsername {
    pub username: String,
}

#[derive(Serialize, Deserialize, Default, Debug, PartialEq)]
pub struct ApiAccountToken {
    pub token: String,
}

#[derive(Serialize, Deserialize, Default, Debug, PartialEq)]
pub struct ApiAccountTokenInfos {
    pub token: String,
    pub timestamp: u64,
    pub expiration_timestamp: u64,
    pub uuid: String, // Owner UUID
}

#[derive(Serialize, Deserialize, Default, Debug, PartialEq)]
pub struct ApiAccountError {
    // HTTP Status Code returned
    pub code: u16,
    // Reason for an error
    pub reason: String,
    // Description for an error if any
    pub description: Option<String>,
}

impl ApiAccountError {
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
