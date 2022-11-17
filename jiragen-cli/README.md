# `jiragen-cli`

A command line tool to send bulk issue creation requests to JIRA from a .csv file.

## Installation

Download the binary (located in the releases section of the GitHub repo) and run it on the command line. Alternatively if you already have Rust installed, you can run `cargo install jiragen-cli`.

## Usage

TODO: update this

```sh
A CLI tool to generate JIRA issues and place them on a board.

USAGE:
    jiragen <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    init    Creates the JiraGen config file. [aliases: i]
    push    Pushes issues to JIRA. [aliases: p]
```

1. Run `jiragen init`. This creates a config file (default: `./jiragen.json`) and issues template file (default: `./jiragen_issues.csv`).
1. Edit the config file with your JIRA credentials and save the file.
1. Edit the issues template .csv file with the issues you would like to generate. Feel free to remove any field columns that are not necessary for issue creation. See [section about how to enter column data](../../../#csv-syntax).
1. Run `jiragen push`. This reads the data in the file and creates the corresponding issues in JIRA.

## Commands

### Command: `jiragen init`

Creates the JiraGen config file.

```sh
jiragen init
#=> creates ./issues.csv

jiragen --issues ./config/my-issues-template.csv init
#=> creates ./config/my-issues-template.csv
```

### Command: `jiragen push`

Takes the content from the issues template file and creates the issues in the JIRA project.

```sh
jiragen push
#=> reads issues.csv in the current folder and pushes issues to JIRA

jiragen --issues ./config/my-issues-template.csv push
#=> reads the file located at `./config/my-issues-template.csv` and pushes issues to JIRA
```

### Command Options

**`--issues`** (default: `"./jiragen-issues.csv"`)
A custom path where the issues template CSV file is created.

## Configuration

Configuration is stored in three environment variables:

**`JIRA_DOMAIN`** (string)
The URL of the Jira instance.

**`JIRA_USERNAME`** (string)
The JIRA user to login as.

**`JIRA_KEY`** (string)
The JIRA user’s API key. (The tool uses Basic Auth).
