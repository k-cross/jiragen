use comfy_table::modifiers::UTF8_ROUND_CORNERS;
use comfy_table::presets::UTF8_FULL;
use comfy_table::{ContentArrangement, Table};
use csv::{Reader, StringRecord};
use jiragen::{csv_to_json, Config, CustomError, Error, JiraClient, JiraIssue};
use serde_json::{json, Value};
use std::path::PathBuf;

/// Creates issues from a template file in JIRA.
pub fn create_tickets(
    conf: Config,
    issues_path: PathBuf,
    link: Option<String>,
) -> Result<(), Error> {
    let jira = JiraClient::new();
    let mut csv_reader = Reader::from_path(&issues_path).unwrap();
    let ids_record = csv_reader.headers()?.clone();
    let ids: Vec<&str> = ids_record.iter().collect();
    let mut csv_records = csv_reader.into_records();

    // skip line 2, which contains human-readable field names
    csv_records.next();

    // create bulk issues to send starting on line 3
    let filtered_csv_records: Vec<StringRecord> = csv_records
        .filter_map(|record_result| match record_result {
            Ok(result) => Some(result),
            Err(_) => None,
        })
        .collect();
    let json_values = csv_to_json(ids, filtered_csv_records)?;
    let link_data = match link {
        None => None,
        Some(l) => {
            let data = format!(
                r#"
                {{
                    "issuelinks":[
                        {{
                           "add":{{
                              "type":{{
                                 "name":"Relates",
                                 "outward":"relates to",
                                 "inward":"relates to"
                              }},
                              "outwardIssue":{{
                                 "key":"{}"
                              }}
                           }}
                        }}
                    ]
                }}"#,
                l
            );
            let link_data: Value = serde_json::from_str(data.as_str())?;
            Some(link_data)
        }
    };

    let issues_to_create: Vec<JiraIssue> = json_values
        .into_iter()
        .map(|record_json| JiraIssue {
            update: link_data.clone(),
            fields: record_json,
        })
        .collect();

    let request_json = json!({ "issueUpdates": issues_to_create });
    let url = format!("{}/rest/api/2/issue/bulk", &conf.jira_url);
    let req = jira
        .client
        .post(&url)
        .json(&request_json)
        .basic_auth(&conf.jira_user, Some(&conf.jira_key))
        .build()?;

    let response = jira.client.execute(req)?;

    if !&response.status().is_success() {
        return Err(Error::CustomError(CustomError {
            message: format!(
                "JIRA responded with status {}:",
                &response.status().as_str()
            ),
            details: response.text()?,
        }));
    }

    let push_data: Value = serde_json::from_str(response.text()?.as_str())?;
    let table = create_table(push_data);

    println!("Issues created successfully:\n\n{}", table);

    Ok(())
}

fn create_table(data: Value) -> Table {
    let mut table = Table::new();

    table
        .load_preset(UTF8_FULL)
        .apply_modifier(UTF8_ROUND_CORNERS)
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_header(vec!["Issues"]);

    let issues_col: Vec<_> = data["issues"]
        .as_array()
        .unwrap()
        .into_iter()
        .map(|iss| {
            format!(
                "ID: {}\nKey: {}\nLink: {}",
                iss["id"].as_str().unwrap(),
                iss["key"].as_str().unwrap(),
                iss["self"].as_str().unwrap(),
            )
        })
        .collect();

    for i in issues_col {
        table.add_row(vec![i]);
    }

    table
}
