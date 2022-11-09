use clap::ArgMatches;
use jiragen::{Config, Error};
use std::fs::write;

/// Processes the `init` SubCommand.
/// Creates the config file as well as the issues template file.
pub fn parse_init(matches: &ArgMatches) -> Result<(), Error> {
    // write config
    let config_path = matches.value_of("config").unwrap();
    let issues_path = matches.value_of("issues").unwrap();

    let config = Config {
        jira_url: "".to_string(),
        jira_user: "".to_string(),
        jira_password: "".to_string(),
    };

    write(config_path, serde_json::to_string_pretty(&config)?)?;
    println!("Wrote config: {}", config_path);

    let mut csv_writer = csv::Writer::from_path(issues_path)?;
    csv_writer.write_record(&[
        "project.key",
        "summary",
        "description",
        "issuetype.id",
        "labels[]",
        "assignee.name",
    ])?;
    csv_writer.write_record(&[
        "Project",
        "Summary",
        "Description",
        "Issue Type",
        "Labels",
        "Assignee",
    ])?;

    println!("Wrote issues: {}", issues_path);

    Ok(())
}
