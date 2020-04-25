#[macro_use]
extern crate serde_json;

use clap_generate::generate;
use clap_generate::generators::{Bash, Elvish, Fish, PowerShell, Zsh};
use failure::Error;
use std::io;
mod auth;
mod cli;
mod diffs;
mod preferences;
mod structs;
mod summary;
mod tasks;

const WHO_AM_I: &str = "api/user.whoami";
/// Preset for comfy-table so that it styles the table for no borders
const NO_BORDER_PRESET: &str = "                     ";

fn main() -> Result<(), Error> {
    let preferences = preferences::get_preferences()?;

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
    } else if let Some(_matches) = matches.subcommand_matches("generate-bash-completions") {
        generate::<Bash, _>(&mut cli::build_cli(&preferences), "fab", &mut io::stdout());
    } else if let Some(_matches) = matches.subcommand_matches("generate-zsh-completions") {
        generate::<Zsh, _>(&mut cli::build_cli(&preferences), "fab", &mut io::stdout());
    } else if let Some(_matches) = matches.subcommand_matches("generate-fish-completions") {
        generate::<Fish, _>(&mut cli::build_cli(&preferences), "fab", &mut io::stdout());
    } else if let Some(_matches) = matches.subcommand_matches("generate-elvish-completions") {
        generate::<Elvish, _>(&mut cli::build_cli(&preferences), "fab", &mut io::stdout());
    } else if let Some(_matches) = matches.subcommand_matches("generate-powershell-completions") {
        generate::<PowerShell, _>(&mut cli::build_cli(&preferences), "fab", &mut io::stdout());
    }
    Ok(())
}
