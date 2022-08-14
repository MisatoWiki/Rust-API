use serde::{Deserialize, Serialize};

use crate::models::account_model::Account;

#[derive(Serialize, Deserialize, Default, Debug, PartialEq)]
pub struct AccountLogin {
    pub account: Option<Account>,
}

#[derive(Serialize, Deserialize, Default, Debug, PartialEq)]
pub struct AccountDeletion {}

#[derive(Serialize, Deserialize, Default, Debug, PartialEq)]
pub struct AccountRegistration {
    pub account: Option<Account>,
}
