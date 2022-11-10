/// This file contains functionality specific to interacting with the JIRA API.
use crate::config::Config;
use reqwest::blocking::{Client, RequestBuilder};
use reqwest::header::HeaderMap;
use serde::Serialize;
use serde_json::Value;
use std::collections::HashMap;

/// A `JiraClient` instance handles requests sent to JIRA. An instance is created via
/// `JiraClient::new(`[`Config`](struct.Config.html)`)`, and then that instance can then be used
/// for creating requests to JIRA, via `.init_request()` (which creates authorization headers using
/// the `Config` username/key).
pub struct JiraClient {
    client: Client,
    config: Config,
}

impl JiraClient {
    /// Creates a new `reqwest` client and returns the `JiraClient` struct wrapper.
    pub fn new(config: Config) -> Self {
        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", "application/json".parse().unwrap());

        let client = Client::builder().default_headers(headers).build().unwrap();

        Self { client, config }
    }

    /// Creates a reqwest Request Builder with some predefined authorization headers.
    pub fn init_request(&self, endpoint: &str) -> RequestBuilder {
        let url = format!("{}{}", self.config.jira_url, endpoint);
        self.client
            .post(&url)
            .basic_auth(&self.config.jira_user, Some(&self.config.jira_key))
    }
}

#[derive(Debug, Serialize)]
/// The object to send to JIRA’s "bulk issue creation" API endpoint
pub struct JiraIssue {
    /// not implemented, set as `None`.
    pub update: Option<HashMap<String, HashMap<String, Vec<String>>>>,
    /// A `serde_json` [Value](https://docs.serde.rs/serde_json/enum.Value.html).
    pub fields: Value,
}
