use reqwest::blocking::Client;
use serde::Deserialize;

#[cfg(debug_assertions)]
use tracing::info;

use std::sync::Once;

use crate::ReqResultError;

static START: Once = Once::new();

#[derive(Deserialize)]
struct Secret {
    uid: String,
    secret: String,
}

impl Secret {
    fn try_new() -> envy::Result<Self> {
        START.call_once(|| {
            dotenv::dotenv().ok();
        });

        envy::from_env::<Secret>()
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct Token {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: u64,
    pub scope: String,
    pub created_at: u64,
}

#[derive(Debug)]
pub enum TokenError {
    Fatal(TokenInitFatalError),
    Status(reqwest::blocking::Response),
}

#[derive(Debug)]
pub enum TokenInitFatalError {
    Env(envy::Error),
    Req(reqwest::Error),
}

impl From<envy::Error> for TokenError {
    fn from(err: envy::Error) -> Self {
        TokenError::Fatal(TokenInitFatalError::Env(err))
    }
}

impl From<reqwest::Error> for TokenError {
    fn from(err: reqwest::Error) -> Self {
        TokenError::Fatal(TokenInitFatalError::Req(err))
    }
}

impl From<ReqResultError> for TokenError {
    fn from(err: ReqResultError) -> Self {
        match err {
            ReqResultError::Fatal(err) => TokenError::Fatal(TokenInitFatalError::Req(err)),
            ReqResultError::Status(res) => TokenError::Status(res),
        }
    }
}

impl From<TokenError> for ReqResultError {
    fn from(err: TokenError) -> Self {
        match err {
            TokenError::Fatal(err) => ReqResultError::Fatal(match err {
                TokenInitFatalError::Env(_) => {
                    unreachable!("cannot convert TokenInitFatalError::Env to ReqResultError::Fatal")
                }
                TokenInitFatalError::Req(err) => err,
            }),
            TokenError::Status(res) => ReqResultError::Status(res),
        }
    }
}

impl Token {
    const EXPIRATION: u64 = 7200;

    pub fn try_new(client: &Client) -> Result<Self, TokenError> {
        let s = Secret::try_new()?;

        let token = ReqResultError::success(
            client
                .post(crate::API_URL_AUTH)
                .form(&[
                    ("grant_type", "client_credentials"),
                    ("client_id", &s.uid),
                    ("client_secret", &s.secret),
                ])
                .send()?,
        )?
        .json::<Token>()?;

        #[cfg(debug_assertions)]
        info!("new token:\n{token:#?}");

        Ok(token)
    }

    pub fn is_expired(&self) -> bool {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("SystemTime before UNIX EPOCH!")
            .as_secs();

        let expired = now >= self.created_at + Self::EXPIRATION;

        #[cfg(debug_assertions)]
        info!(
            "checking token expiration:\t{}",
            if expired {
                "expired".to_string()
            } else {
                format!(
                    "expires in {} seconds",
                    self.created_at + Self::EXPIRATION - now
                )
            }
        );

        expired
    }

    pub fn refresh(&mut self, client: &Client) -> Result<(), TokenError> {
        #[cfg(debug_assertions)]
        info!("refreshing token");

        *self = Self::try_new(client)?;
        Ok(())
    }
}
