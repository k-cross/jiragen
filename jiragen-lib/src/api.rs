/// This file contains functionality specific to interacting with the JIRA API.
use reqwest::blocking::Client;
use reqwest::header::{HeaderMap, HeaderValue, ACCEPT, CONTENT_TYPE};
use serde::Serialize;
use serde_json::Value;
use std::collections::HashMap;

/// A `JiraClient` instance handles requests sent to JIRA. An instance is created via
/// `JiraClient::new(`[`Config`](struct.Config.html)`)`, and then that instance can then be used
/// for creating requests to JIRA, via `.init_request()` (which creates authorization headers using
/// the `Config` username/key).
#[derive(Debug)]
pub struct JiraClient {
    pub client: Client,
}

impl JiraClient {
    /// Creates a new `reqwest` client and returns the `JiraClient` struct wrapper.
    pub fn new() -> Self {
        let mut headers = HeaderMap::new();
        headers.insert(ACCEPT, HeaderValue::from_static("application/json"));
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

        let client = Client::builder().default_headers(headers).build().unwrap();

        Self { client }
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
