use chrono::{DateTime, Utc};
use reqwest::blocking::Client;
use serde::Deserialize;
use serde_json::Value;

use crate::{ReqResultError, Token};

impl Token {
    pub fn req_user(&mut self, client: &Client, login: &str) -> Result<User, ReqResultError> {
        self.req::<User>(client, &format!("users/{login}"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::log_test;
    use crate::Token;

    const LOGIN: &str = "adelille";
    const LOG_FILE: &str = "test_req_user.log";

    #[test]
    fn test_req_user() {
        let client = Client::new();

        let mut token = Token::try_new(&client).unwrap();

        let user = token.req_user(&client, LOGIN).unwrap();

        assert_eq!(user.login, LOGIN);

        log_test(LOG_FILE, &format!("{user:#?}")).unwrap();
    }
}

#[derive(Deserialize, Debug)]
pub struct User {
    pub achievements: Vec<Value>,
    pub active: Option<bool>,
    pub alumni: Option<bool>,
    pub alumnized_at: Option<DateTime<Utc>>,
    pub anonymize_date: Option<DateTime<Utc>>,
    pub campus: Vec<Value>,
    pub campus_users: Vec<Value>,
    pub correction_point: i16,
    pub created_at: DateTime<Utc>,
    pub cursus_users: Vec<Value>,
    pub data_erasure_date: DateTime<Utc>,
    pub displayname: String,
    pub email: String,
    pub expertises_users: Vec<Value>,
    pub first_name: String,
    pub groups: Vec<Value>,
    pub id: usize,
    pub image: Image,
    pub kind: String,
    pub languages_users: Vec<Value>,
    pub last_name: String,
    pub location: Option<String>,
    pub login: String,
    pub partnerships: Vec<Value>,
    pub patroned: Vec<Value>,
    pub patroning: Vec<Value>,
    pub phone: String,
    pub pool_month: Option<String>,
    pub pool_year: Option<String>,
    pub projects_users: Vec<ProjectUser>,
    pub roles: Vec<Value>,
    pub staff: Option<bool>,
    pub titles: Vec<Value>,
    pub titles_users: Vec<Value>,
    pub updated_at: DateTime<Utc>,
    pub url: String,
    pub usual_first_name: Option<String>,
    pub usual_full_name: String,
    pub wallet: i64,
}

#[derive(Deserialize, Debug)]
pub struct Image {
    pub link: Option<String>,
    pub versions: ImageVersions,
}

#[derive(Deserialize, Debug)]
pub struct ImageVersions {
    pub large: Option<String>,
    pub medium: Option<String>,
    pub micro: Option<String>,
    pub small: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct ProjectUser {
    pub created_at: DateTime<Utc>,
    pub current_team_id: Option<usize>,
    pub cursus_ids: Vec<usize>,
    pub final_mark: Option<i16>,
    pub id: usize,
    pub marked: bool,
    pub marked_at: Option<DateTime<Utc>>,
    pub project: Project,
    pub retriable_at: Option<DateTime<Utc>>,
    pub status: String,
    pub updated_at: DateTime<Utc>,
    pub validated: Option<bool>,
}

#[derive(Deserialize, Debug)]
pub struct Project {
    pub id: usize,
    pub name: String,
    pub parent_id: Option<usize>,
    pub slug: String,
}
