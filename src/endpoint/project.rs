// use chrono::{DateTime, Utc};
use reqwest::blocking::Client;
// use serde::Deserialize;
use serde_json::Value;

use crate::{ReqResultError, Token};

#[derive(Debug, Clone)]
pub struct ReqProjectsParams {
    pub page: usize,
    pub filter: Vec<(String, String)>,
}

impl Token {
    // pub fn req_projects(
    //     &mut self,
    //     client: &Client,
    //     params: ReqProjectsParams,
    // ) -> Result<Value, ReqResultError> {
    //     let mut endpoint = format!("cursus/21/projects?page={}", params.page);

    //     for (key, value) in params.filter {
    //         endpoint.push_str(&format!("?filter[{key}]={value}"));
    //     }

    //     self.req::<Value>(client, &endpoint)
    // }

    pub fn req_projects_endpoint(params: ReqProjectsParams) -> String {
        let mut endpoint = format!("cursus/21/projects?page={}", params.page);

        for (key, value) in params.filter {
            endpoint.push_str(&format!("&filter[{key}]={value}"));
        }

        endpoint
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::log_test;
    use crate::Token;

    const LOG_FILE: &str = "req_projects";

    #[test]
    fn test_req_projects() {
        let client = Client::new();

        let mut token = Token::try_new(&client).unwrap();

        let filter = vec![
            ("visible".to_string(), "true".to_string()),
            ("exam".to_string(), "false".to_string()),
        ];

        let res = token
            .simple_req(
                &client,
                &Token::req_projects_endpoint(ReqProjectsParams {
                    page: 1,
                    filter: filter.clone(),
                }),
            )
            .unwrap();

        log_test(
            &format!("{LOG_FILE}_header"),
            &format!("{:#?}", res.headers()),
        )
        .unwrap();

        for i in 1..=16 {
            let projects = token
                .req::<serde_json::Value>(
                    &client,
                    &Token::req_projects_endpoint(ReqProjectsParams {
                        page: i,
                        filter: filter.clone(),
                    }),
                )
                .unwrap();

            log_test(&format!("{LOG_FILE}_{i}"), &format!("{projects:#?}")).unwrap();
        }
    }
}

// #[derive(Deserialize, Debug)]
// pub struct Project {
//     pub id: usize,
//     pub name: String,
//     pub parent_id: Option<usize>,
//     pub slug: String,
// }
