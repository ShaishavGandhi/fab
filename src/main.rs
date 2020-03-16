
#[macro_use]
extern crate serde_json;

use clap;
use clap::{App, SubCommand};

mod structs;
mod diffs;

fn main() {
    let matches = App::new("Phab")
        .author("Shaishav <shaishavgandhi05@gmail.com>")
        .version("0.1.0")
        .subcommand(SubCommand::with_name("diff")
            .version("0.1.0")
            .author("Shaishav <shaishavgandhi05@gmail.com>"))
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("diff") {
        diffs::process_diff_command(matches)
    }
}
