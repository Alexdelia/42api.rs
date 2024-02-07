mod access_token;
mod req;
mod user;

#[cfg(test)]
mod test;

use const_format::concatcp;

pub const BASE_URL: &str = "https://api.intra.42.fr/";
pub const API_VERSION: &str = "v2";

pub const API_URL: &str = concatcp!(BASE_URL, API_VERSION, "/");
pub const API_URL_AUTH: &str = concatcp!(API_URL, "oauth/token");

pub const EXPIRATION: u64 = 7200;

pub use access_token::{Token, TokenError, TokenInitFatalError};
pub use req::ReqResultError;
pub use user::User;
