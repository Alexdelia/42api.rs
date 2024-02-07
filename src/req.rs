use reqwest::{blocking::Client, StatusCode};
use serde::de::DeserializeOwned;

#[cfg(debug_assertions)]
use tracing::{error, info};

use crate::{Token, API_URL};

#[derive(Debug)]
pub enum ReqResultError {
    Fatal(reqwest::Error),
    Status(reqwest::blocking::Response),
}

impl From<reqwest::Error> for ReqResultError {
    fn from(err: reqwest::Error) -> Self {
        ReqResultError::Fatal(err)
    }
}

impl ReqResultError {
    pub fn success(
        response: reqwest::blocking::Response,
    ) -> Result<reqwest::blocking::Response, Self> {
        // decided to go with `is_success` instead of
        // `response.error_for_status()` or `status.is_client_error() || status.is_server_error()`
        if response.status().is_success() {
            Ok(response)
        } else {
            Err(ReqResultError::Status(response))
        }
    }

    pub fn is_status(&self, status: StatusCode) -> bool {
        match self {
            ReqResultError::Status(res) => res.status() == status,
            _ => false,
        }
    }
}

impl Token {
    pub fn req<T: DeserializeOwned>(
        &mut self,
        client: &Client,
        endpoint: &str,
    ) -> Result<T, ReqResultError> {
        #[cfg(debug_assertions)]
        info!("requesting {API_URL}{endpoint}");

        if self.is_expired() {
            self.refresh(client)?;
        }

        match ReqResultError::success(self.simple_req(client, endpoint)?) {
            Ok(res) => match res.json::<T>() {
                Ok(json) => Ok(json),
                Err(err) => {
                    #[cfg(debug_assertions)]
                    {
                        let res_json = self
                            .simple_req(client, endpoint)?
                            .json::<serde_json::Value>()?;
                        error!("failed to parse json:\n\t{res_json:#?}\n\n\t{err:#?}");
                    }
                    Err(ReqResultError::Fatal(err))
                }
            },
            Err(ReqResultError::Status(res)) => {
                if res.status() == StatusCode::UNAUTHORIZED {
                    #[cfg(debug_assertions)]
                    error!(
                        "request failed with status {status}\n{res:#?}",
                        status = res.status()
                    );

                    self.refresh(client)?;

                    #[cfg(debug_assertions)]
                    info!("retrying {API_URL}{endpoint}");

                    Ok(ReqResultError::success(self.simple_req(client, endpoint)?)?.json::<T>()?)
                } else {
                    Err(ReqResultError::Status(res))
                }
            }
            Err(ReqResultError::Fatal(err)) => Err(ReqResultError::Fatal(err)),
        }
    }

    fn simple_req(
        &self,
        client: &Client,
        endpoint: &str,
    ) -> Result<reqwest::blocking::Response, ReqResultError> {
        client
            .get(format!("{API_URL}{endpoint}"))
            .bearer_auth(&self.access_token)
            .send()
            .map_err(|err| ReqResultError::Fatal(err))
    }

    #[cfg(debug_assertions)]
    pub fn req_dbg(&self, client: &Client, endpoint: &str) -> () {
        let res = client
            .get(format!("{API_URL}{endpoint}"))
            .bearer_auth(&self.access_token)
            .send()
            .expect("request failed");
        dbg!(&res);
        dbg!(res
            .json::<serde_json::Value>()
            .expect("failed to parse json"));
    }

    #[cfg(debug_assertions)]
    pub fn req_expired(client: &Client) -> () {
        let epoch = std::time::SystemTime::UNIX_EPOCH;

        let now = std::time::SystemTime::now();
        let expires_in = (now + std::time::Duration::from_secs(crate::EXPIRATION))
            .duration_since(epoch)
            .expect("SystemTime before UNIX EPOCH!")
            .as_secs();

        let mut token = Token {
            access_token: String::from("some_expired_token"),
            token_type: String::new(),
            expires_in,
            scope: String::new(),
            created_at: now
                .duration_since(epoch)
                .expect("SystemTime before UNIX EPOCH!")
                .as_secs(),
        };

        info!("requesting with expired token");

        let login = "adelille";

        match token.req_user(client, login) {
            Ok(user) => info!(
                "successfuly regenreated token, got {displayname} for {login}",
                displayname = user.displayname
            ),
            Err(err) => unreachable!("failed:\n{err:#?}"),
        }
    }
}
