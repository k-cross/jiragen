use jiragen::{Config, CustomError, Error, JiraClient};
use serde_json::Value;

/// Get raw JSON from JIRA about a project to gather identifier info.
pub fn get(conf: Config, project: String) -> Result<(), jiragen::Error> {
    let jira = JiraClient::new();
    let url = format!("{}/rest/api/2/project/{}", conf.jira_url, project);

    let response = jira
        .client
        .get(&url)
        .basic_auth(conf.jira_user.as_str(), Some(conf.jira_key.as_str()))
        .send()?;

    if !&response.status().is_success() {
        return Err(Error::CustomError(CustomError {
            message: format!(
                "JIRA responded with status {}:",
                &response.status().as_str()
            ),
            details: response.text()?,
        }));
    }

    let v: Value = serde_json::from_str(response.text()?.as_str())?;

    println!("Info from project:\n\n{:#?}", v);

    Ok(())
}
