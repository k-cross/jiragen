/// This file contains functionality specific to interacting with the JIRA API.
use crate::config::Config;
use http::Method;
use reqwest::{header::HeaderMap, Client, RequestBuilder};
use serde::Serialize;
use serde_json::Value;
use std::collections::HashMap;

/// Handles requests sent to JIRA.
pub struct JiraClient {
  pub client: Client,
  pub config: Config,
}

impl JiraClient {
  /// Creates a new `reqwest` client and returns the `JiraClient` struct wrapper.
  /// ```
  /// use jiragen::{Config, JiraClient};
  ///
  /// let config = Config {
  ///   jira_url: "https://my-jira.com".to_string(),
  ///   jira_user: "my-user".to_string(),
  ///   jira_password: "my-password".to_string(),
  /// };
  ///
  /// let jira = JiraClient::new(config);
  /// ```
  pub fn new(config: Config) -> Self {
    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", "application/json".parse().unwrap());

    let client = Client::builder().default_headers(headers).build().unwrap();

    Self { client, config }
  }

  /// Creates a reqwest Request Builder with some predefined authorization headers
  /// ```
  /// use jiragen::{Config, JiraClient};
  /// use serde_json::json;
  ///
  /// let config = Config {
  ///   jira_url: "https://my-jira.com".to_string(),
  ///   jira_user: "my-user".to_string(),
  ///   jira_password: "my-password".to_string(),
  /// };
  ///
  /// let jira = JiraClient::new(config);
  ///
  /// let request = jira.init_request("POST", "/rest/api/2/issue/bulk");
  /// let json = json!({ "some_key": "some_value" }).to_string();
  ///
  /// let response = request.body(json).send();
  /// ```
  pub fn init_request(&self, method_str: &str, endpoint: &str) -> RequestBuilder {
    let method_str_ucase = method_str.to_uppercase();
    let method = method_str_ucase.as_bytes();

    let url = format!("{}{}", self.config.jira_url, endpoint);
    self
      .client
      .request(Method::from_bytes(method).unwrap(), &url)
      .basic_auth(&self.config.jira_user, Some(&self.config.jira_password))
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
