#[macro_use]
extern crate serde_json;

use clap;
use clap::{App, Arg};
mod auth;
mod diffs;
mod network;
mod preferences;
mod structs;
mod summary;
mod tasks;

const WHO_AM_I: &str = "api/user.whoami";
/// Preset for comfy-table so that it styles the table for no borders
const NO_BORDER_PRESET: &str = "                     ";

fn main() {
    let version = "0.2.0";
    let preferences = preferences::get_preferences().unwrap();

    let default_task_priority: &Vec<&str> = &preferences
        .default_task_priority
        .iter()
        .map(std::ops::Deref::deref)
        .collect();

    let matches = App::new("Fab")
        .author("Shaishav <shaishavgandhi05@gmail.com>")
        .version(version)
        .subcommand(App::new("diffs")
            .version(version)
            .author("Shaishav <shaishavgandhi05@gmail.com>")
            .about("Commands related to your differential revisions")
            .arg(Arg::with_name("needs-review")
                .short('n')
                .long("needs-review")
                .help("Show diffs that need your review")))
        .subcommand(App::new("tasks")
            .about("Commands related to maniphest tasks")
            .version(version)
            .author("Shaishav <shaishavgandhi05@gmail.com>")
            .arg(Arg::with_name("priority")
                .short('p')
                .long("priority")
                .possible_values(&["unbreak-now", "needs-triage", "high", "normal", "low", "wishlist"])
                .help("Specify the priority of the task")
                .default_values(default_task_priority)
                .multiple(true))
            .arg(Arg::with_name("limit")
                .short('l')
                .long("limit")
                .help("limit results by a value")
                .default_value("20")))
        .subcommand(App::new("summary")
            .about("Gives a snapshot of what is relevant to you in the moment")
            .version(version)
            .author("Shaishav <shaishavgandhi05@gmail.com>"))
        .subcommand(App::new("configure")
            .about("Configure settings ")
            .version(version)
            .author("Shaishav <shaishavgandhi05@gmail.com>"))
        .get_matches();

    let result = auth::init();
    let config = match result {
        Ok(config) => config,
        Err(message) => panic!("{}", message),
    };

    if let Some(matches) = matches.subcommand_matches("diffs") {
        diffs::process_diff_command(matches, &config)
    } else if let Some(matches) = matches.subcommand_matches("tasks") {
        tasks::process_task_command(matches, &config, &preferences)
    } else if let Some(matches) = matches.subcommand_matches("summary") {
        summary::process_summary(matches, &config, &preferences);
    } else if let Some(matches) = matches.subcommand_matches("configure") {
        preferences::process_configuration(matches).expect("Failed to process configuration command");
    }
}
