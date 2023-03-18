use comfy_table::modifiers::UTF8_ROUND_CORNERS;
use comfy_table::presets::UTF8_FULL;
use comfy_table::{ContentArrangement, Table};
use itertools::izip;
use jiragen::{Config, CustomError, Error, JiraClient};
use serde_json::Value;

/// Get raw JSON from JIRA about a project to gather identifier info.
pub fn get(conf: Config, project: String) -> Result<(), jiragen::Error> {
    let jira = JiraClient::new();
    let url = format!("{}/rest/api/3/project/{}", conf.jira_url, &project);

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
    let table = create_project_table(v);

    println!("Project {}:\n\n{}", project, table);

    Ok(())
}

// fn get_project(jira: JiraClient, project: String) -> Result<(), jiragen::Error> {
//     let url = format!("{}/rest/api/2/project/{}", conf.jira_url, &project);
//
//     let response = jira
//         .client
//         .get(&url)
//         .basic_auth(conf.jira_user.as_str(), Some(conf.jira_key.as_str()))
//         .send()?;
//
//     if !&response.status().is_success() {
//         return Err(Error::CustomError(CustomError {
//             message: format!(
//                 "JIRA responded with status {}:",
//                 &response.status().as_str()
//             ),
//             details: response.text()?,
//         }));
//     }
//
//     let v: Value = serde_json::from_str(response.text()?.as_str())?;
//     let table = create_project_table(v);
//
//     println!("Project {}:\n\n{}", project, table);
//
//     Ok(())
// }
//
// fn is_error(response: &Response) -> Result<(), Error> {
//     if !&response.status().is_success() {
//         return Err(Error::CustomError(CustomError {
//             message: format!(
//                 "JIRA responded with status {}:",
//                 &response.status().as_str()
//             ),
//             details: response.text()?,
//         }));
//     }
//     Ok(())
// }

fn create_project_table(data: Value) -> Table {
    let mut table = Table::new();

    table
        .load_preset(UTF8_FULL)
        .apply_modifier(UTF8_ROUND_CORNERS)
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_header(vec!["Components", "Issue Type", "Roles"]);

    let components_col: Vec<_> = data["components"]
        .as_array()
        .unwrap()
        .into_iter()
        .map(|comp| {
            format!(
                "Name: {}\nID: {}",
                comp["name"].as_str().unwrap(),
                comp["id"].as_str().unwrap()
            )
        })
        .collect();

    let issue_types_col: Vec<_> = data["issueTypes"]
        .as_array()
        .unwrap()
        .into_iter()
        .map(|iss| {
            format!(
                "Name: {}\nID: {}",
                iss["name"].as_str().unwrap(),
                iss["id"].as_str().unwrap()
            )
        })
        .collect();

    let roles_col: Vec<_> = data["roles"]
        .as_object()
        .unwrap()
        .iter()
        .map(|(k, _v)| k.to_owned())
        .collect();

    let rows = rows(components_col, issue_types_col, roles_col);
    for row in rows {
        table.add_row(row);
    }

    table
}

fn rows(mut comps: Vec<String>, mut iss: Vec<String>, mut roles: Vec<String>) -> Vec<Vec<String>> {
    let mut rows = Vec::new();
    let mx = std::cmp::max(std::cmp::max(comps.len(), iss.len()), roles.len());
    comps.resize(mx, "".to_owned());
    iss.resize(mx, "".to_owned());
    roles.resize(mx, "".to_owned());

    for (c, i, r) in izip!(&comps, &iss, &roles) {
        rows.push(vec![c.to_owned(), i.to_owned(), r.to_owned()]);
    }
    rows
}
