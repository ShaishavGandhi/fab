
#[macro_use]
extern crate serde_json;

use clap;
use clap::{App, SubCommand, Arg};
mod structs;
mod diffs;
mod tasks;
mod network;
mod auth;
mod summary;

const WHO_AM_I: &str = "api/user.whoami";
/// Preset for comfy-table so that it styles the table for no borders
const NO_BORDER_PRESET: &str = "                     ";

fn main() {
    let matches = App::new("Fab")
        .author("Shaishav <shaishavgandhi05@gmail.com>")
        .version("0.1.0")
        .subcommand(SubCommand::with_name("diffs")
            .version("0.1.0")
            .author("Shaishav <shaishavgandhi05@gmail.com>")
            .about("Commands related to your differential revisions")
            .arg(Arg::with_name("needs-review")
                .short("n")
                .long("needs-review")
                .help("Show diffs that need your review")))
        .subcommand(SubCommand::with_name("tasks")
            .about("Commands related to maniphest tasks")
            .version("0.1.0")
            .author("Shaishav <shaishavgandhi05@gmail.com>")
            .arg(Arg::with_name("priority")
                .short("p")
                .long("priority")
                .possible_values(&["unbreak-now", "needs-triage", "high", "normal", "low", "wishlist"])
                .help("Specify the priority of the task")
                .default_value("high")
                .multiple(true))
            .arg(Arg::with_name("limit")
                .short("l")
                .long("limit")
                .help("limit results by a value")
                .default_value("20")))
        .subcommand(SubCommand::with_name("summary")
            .about("Gives a snapshot of what is relevant to you in the moment")
            .version("0.1.0")
            .author("Shaishav <shaishavgandhi05@gmail.com>"))
        .get_matches();

    let result = auth::init();
    let config = match result {
       Ok(config) => config,
       Err(message) => panic!("{}", message)
    };


    if let Some(matches) = matches.subcommand_matches("diffs") {
        diffs::process_diff_command(matches, &config)
    } else if let Some(matches) = matches.subcommand_matches("tasks") {
        tasks::process_task_command(matches, &config)
    } else if let Some(matches) = matches.subcommand_matches("summary") {
        summary::process_summary(matches, &config);
    }
}
