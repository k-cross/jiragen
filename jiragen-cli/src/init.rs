use jiragen::Error;
use std::path::PathBuf;

/// Processes the `init` SubCommand.
/// Creates the config file as well as the issues template file.
pub fn create_file_templates(issues_path: PathBuf) -> Result<(), Error> {
    let mut csv_writer = csv::Writer::from_path(&issues_path)?;
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

    println!("Wrote issues: {:?}", issues_path.as_os_str());

    Ok(())
}
