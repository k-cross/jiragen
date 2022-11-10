use serde::{Deserialize, Serialize};

/// JiraGen configuration that is used for sending requests to JIRA. The username and API key
/// fields are required because JiraGen uses Basic Authentication to send requests to JIRA.
#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    /// The URL of the JIRA server.
    pub jira_url: String,
    /// The user account name to login to JIRA
    pub jira_user: String,
    /// The user's API key to login to JIRA
    pub jira_key: String,
}
