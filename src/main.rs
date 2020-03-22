
#[macro_use]
extern crate serde_json;

use clap;
use clap::{App, SubCommand};
use std::fs::read_to_string;
use std::io;
use crate::structs::FabConfig;

mod structs;
mod diffs;

fn main() {
    let matches = App::new("Fab")
        .author("Shaishav <shaishavgandhi05@gmail.com>")
        .version("0.1.0")
        .subcommand(SubCommand::with_name("diff")
            .version("0.1.0")
            .author("Shaishav <shaishavgandhi05@gmail.com>"))
        .get_matches();

    let result = init();
    let config = match result {
       Ok(config) => config,
       Err(message) => panic!("{}", message)
    };


    if let Some(matches) = matches.subcommand_matches("diff") {
        diffs::process_diff_command(matches, &config)
    }
}

fn init() -> Result<FabConfig, &'static str>{
    let contents = read_to_string("~/.fab/config");
    if contents.is_err() {
        println!("Welcome to Fab! Let's get you started");

        println!("Enter the URL where your Phabricator instance is hosted. Example: https://phab.mycompany.com/");
        let mut hosted_instance = String::new();

        io::stdin().read_line(&mut hosted_instance)
            .expect("Failed to read URL");

        // Trim newlines
        hosted_instance = hosted_instance.trim().to_string();

        // Add a trailing `/` if needed.
        if !hosted_instance.ends_with('/') {
            hosted_instance.push('/')
        }

        // Make sure hosted instance is present.
        if hosted_instance.is_empty() {
            return Result::Err("Hosted Instance cannot be null");
        }

        println!("Enter the API token that Phab can use. You can create one at {}settings/user/YOUR_USERNAME/page/apitokens", hosted_instance);
        let mut api_token = String::new();

        io::stdin().read_line(&mut api_token)
            .expect("Failed to read token");

        // Trim newlines
        api_token = api_token.trim().to_string();

        // Make sure API token is present.
        if api_token.is_empty() {
            return Result::Err("API Token cannot be null");
        }

        return Result::Ok(FabConfig {
            hosted_instance,
            api_token
        })
    }

    return Result::Err("Couldn't initialize fab");
}
