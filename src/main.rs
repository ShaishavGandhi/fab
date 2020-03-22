
#[macro_use]
extern crate serde_json;

use clap;
use clap::{App, SubCommand, Arg};
use std::fs::{read_to_string, File};
use std::{io, fs};
use crate::structs::{FabConfig, WhoAmIResponse};

mod structs;
mod diffs;

const WHO_AM_I: &str = "api/user.whoami";

fn main() {
    let matches = App::new("Fab")
        .author("Shaishav <shaishavgandhi05@gmail.com>")
        .version("0.1.0")
        .subcommand(SubCommand::with_name("diffs")
            .version("0.1.0")
            .author("Shaishav <shaishavgandhi05@gmail.com>")
            .arg(Arg::with_name("needs-review")
                .short("n")
                .long("needs-review")
                .help("Show diffs that need your review"))
        ).get_matches();

    let result = init();
    let config = match result {
       Ok(config) => config,
       Err(message) => panic!("{}", message)
    };


    if let Some(matches) = matches.subcommand_matches("diffs") {
        diffs::process_diff_command(matches, &config)
    }
}

fn init() -> Result<FabConfig, &'static str> {
    let path_buf = dirs::home_dir()
        .expect("Couldn't find home directory");

    let home_dir = path_buf.to_str().expect("Couldn't convert home directory to string.");
    let fab_dir = format!("{}/.fab", home_dir);
    let config_file = format!("{}/.fab/config.json", home_dir);

    let contents = read_to_string(&config_file);
    return match contents {
        Err(_message) => {
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

            // Get user's details
            let phid = match get_phid(&hosted_instance, &api_token) {
                Ok(phid) => phid,
                Err(message) => panic!(message)
            };

            let config = FabConfig {
                hosted_instance,
                api_token,
                phid
            };

            fs::create_dir_all(&fab_dir).expect(&format!("Couldn't create directory '{}'", &fab_dir));
            serde_json::to_writer(&File::create(config_file).expect("Couldn't load file"), &config).expect("Couldn't write config to file");

            Result::Ok(config)
        },
        Ok(content) => {
            let config: FabConfig = ::serde_json::from_str(&content).expect("Couldn't deserialize to FabConfig");
            Result::Ok(config)
        }
    }
}

fn get_phid(hosted_instance: &String, api_token: &String) -> Result<String, &'static str> {
    let url = format!("{}{}", hosted_instance, WHO_AM_I);
    let json_body = json!({
            "api.token": api_token
            });

    let response = reqwest::blocking::Client::new()
        .post(&url)
        .form(&json_body)
        .send()
        .expect("Error fetching user details");

    return match response.json::<WhoAmIResponse>() {
        Ok(res) => Result::Ok(res.result.phid),
        Err(_message) => Result::Err("Error getting user's phabricator ID")
    }
}
