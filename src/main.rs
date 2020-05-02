#[macro_use]
extern crate serde_json;

use crate::preferences::Preferences;
use anyhow::{anyhow, Error};
use clap_generate::generate;
use clap_generate::generators::{Bash, Elvish, Fish, PowerShell, Zsh};
use std::io;
mod auth;
mod cli;
mod diffs;
mod preferences;
mod structs;
mod summary;
mod tasks;
mod users;

const WHO_AM_I: &str = "api/user.whoami";
/// Preset for comfy-table so that it styles the table for no borders
const NO_BORDER_PRESET: &str = "                     ";

fn main() -> Result<(), Error> {
    let preferences = preferences::get_preferences()?;

    // Migrate any newer fields, assign them values and store them
    let preferences = migrate_preferences(preferences)?;

    let app = cli::build_cli(&preferences);
    let matches = &app.get_matches();

    let config = auth::init()?;

    if let Some(matches) = matches.subcommand_matches("diffs") {
        diffs::process_diff_command(matches, &config)?
    } else if let Some(matches) = matches.subcommand_matches("tasks") {
        tasks::process_task_command(matches, &config, &preferences)?
    } else if let Some(matches) = matches.subcommand_matches("summary") {
        summary::process_summary(matches, &config, &preferences)?;
    } else if let Some(matches) = matches.subcommand_matches("configure") {
        preferences::process_configuration(matches)?;
    } else if let Some(matches) = matches.subcommand_matches("generate-shell-completions") {
        let shell = matches
            .value_of("shell")
            .expect("No shell specified for generating completions");

        match shell {
            "bash" => {
                generate::<Bash, _>(&mut cli::build_cli(&preferences), "fab", &mut io::stdout())
            }
            "zsh" => {
                generate::<Zsh, _>(&mut cli::build_cli(&preferences), "fab", &mut io::stdout())
            }
            "fish" => {
                generate::<Fish, _>(&mut cli::build_cli(&preferences), "fab", &mut io::stdout())
            }
            "elvish" => {
                generate::<Elvish, _>(&mut cli::build_cli(&preferences), "fab", &mut io::stdout())
            }
            "powershell" => generate::<PowerShell, _>(
                &mut cli::build_cli(&preferences),
                "fab",
                &mut io::stdout(),
            ),
            _ => return Err(anyhow!("No matching shell specified")),
        }
    }
    Ok(())
}

/// Migrates any missing fields that were added as the application progresses.
///
/// It's important to remember to add any default values otherwise confy will blow
/// up with a BadTomlError
fn migrate_preferences(preferences: Preferences) -> Result<Preferences, Error> {
    let preferences = Preferences {
        default_limit_str: preferences.default_limit.to_string(),
        default_limit: preferences.default_limit,
        default_sort: preferences.default_sort,
        default_task_priority: preferences.default_task_priority,
        summary_task_priority: preferences.summary_task_priority,
    };
    preferences::set_preferences(&preferences)?;
    Ok(preferences)
}
