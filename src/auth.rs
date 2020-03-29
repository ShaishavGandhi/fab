use reqwest::blocking::RequestBuilder;
use std::{io, fs};
use serde::{Deserialize};
use crate::structs::{FabConfig, WhoAmIResponse};
use std::fs::{read_to_string, File};
use crate::WHO_AM_I;

pub fn init() -> Result<FabConfig, String> {
    let existing_config = read_config();

    match existing_config {
        Ok(config) => Result::Ok(config),
        Err(_) => {
            println!(" _    _      _                            _         ______    _
| |  | |    | |                          | |        |  ___|  | |
| |  | | ___| | ___ ___  _ __ ___   ___  | |_ ___   | |_ __ _| |__
| |/\\| |/ _ \\ |/ __/ _ \\| '_ ` _ \\ / _ \\ | __/ _ \\  |  _/ _` | '_ \\
\\  /\\  /  __/ | (_| (_) | | | | | |  __/ | || (_) | | || (_| | |_) |
 \\/  \\/ \\___|_|\\___\\___/|_| |_| |_|\\___|  \\__\\___/  \\_| \\__,_|_.__/");

            println!("Let's get you started!");
            println!("Enter the URL where your Phabricator instance is hosted. Example: https://phab.mycompany.com/");

            let hosted_instance = prompt_hosted_instance();
            if hosted_instance.is_err() {
                return Result::Err(hosted_instance.err().unwrap());
            }

            let hosted_instance = hosted_instance.unwrap();

            let token = prompt_token(&hosted_instance);
            if token.is_err() {
                return Result::Err(token.err().unwrap());
            }

            let token = token.unwrap();

            // Get user's details
            let phid = match get_phid(&hosted_instance, &token) {
                Ok(phid) => phid,
                Err(message) => panic!(message)
            };

            let config = FabConfig {
                hosted_instance,
                api_token: token,
                phid
            };

            write_config(&config);

            Result::Ok(config)
        }
    }
}

/// Function that will execute the network request provided by RequestBuilder and
/// prompt for an API token if session is invalidated.
pub fn send<T: serde::de::DeserializeOwned>(config: &FabConfig, request: RequestBuilder) -> Result<T, String> {
    let request = request.try_clone().unwrap();
    let response = request
        .send()
        .unwrap()
        .json::<NetworkResponse<T>>()
        .unwrap();

    if response.result.is_some() {
        return Result::Ok(response.result.unwrap());
    } else if response.error_code.is_some() {
        let error_code: String = response.error_code.unwrap();

        if error_code.eq("ERR-INVALID-AUTH") || error_code.eq("ERR-INVALID-SESSION") {
            println!("Your API Token has expired.");
            let current_config = read_config().expect("Couldn't find existing config file");
            let token = prompt_token(&config.hosted_instance);
            match token {
                Ok(token) => {
                    let new_config = FabConfig {
                        api_token: token,
                        hosted_instance: current_config.hosted_instance,
                        phid: current_config.phid
                    };

                    write_config(&new_config)
                },
                Err(_err) => {}
            }
        }
    }
    Result::Err(String::from("Token regenerated. Please try the command again"))
}

/// Prompts for a token and writes the token to the configuration file.
fn prompt_token(hosted_instance: &str) -> Result<String, String> {

    println!("Enter an API token that Fab can use. You can create one at {}settings/user/YOUR_USERNAME/page/apitokens", &hosted_instance);
    let mut api_token = String::new();

    io::stdin().read_line(&mut api_token)
        .expect("Failed to read token");

    // Trim newlines
    api_token = api_token.trim().to_string();

    if api_token.is_empty() {
        return Result::Err(String::from("API Token cannot be null or empty"));
    }

    Result::Ok(api_token)
}

fn prompt_hosted_instance() -> Result<String, String> {
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
        return Result::Err(String::from("Hosted Instance cannot be null"));
    }

    Result::Ok(hosted_instance)
}

fn write_config(config: &FabConfig) {
    let path_buf = dirs::home_dir()
        .expect("Couldn't find home directory");

    let home_dir = path_buf.to_str().expect("Couldn't convert home directory to string.");
    let fab_dir = format!("{}/.fab", home_dir);
    let config_file = format!("{}/.fab/config.json", home_dir);

    fs::create_dir_all(&fab_dir).unwrap_or_else(|_| panic!("Couldn't create directory {}", &fab_dir));
    serde_json::to_writer(&File::create(config_file).expect("Couldn't load file"), &config).expect("Couldn't write config to file");
}

/// Tries to read the config file
fn read_config() -> Result<FabConfig, String> {
    let path_buf = dirs::home_dir()
        .expect("Couldn't find home directory");

    let home_dir = path_buf.to_str().expect("Couldn't convert home directory to string.");
    let config_file = format!("{}/.fab/config.json", home_dir);

    let contents = read_to_string(&config_file);

    match contents {
        Err(_message) => Result::Err(String::from("Failed to read file")),
        Ok(content) => {
            let config: FabConfig = ::serde_json::from_str(&content).expect("Couldn't deserialize to FabConfig");
            Result::Ok(config)
        }
    }
}

fn get_phid(hosted_instance: &str, api_token: &str) -> Result<String, String> {
    let url = format!("{}{}", hosted_instance, WHO_AM_I);
    let json_body = json!({
            "api.token": api_token
            });

    let response = reqwest::blocking::Client::new()
        .post(&url)
        .form(&json_body)
        .send()
        .expect("Error fetching user details");

    match response.json::<WhoAmIResponse>() {
        Ok(res) => Result::Ok(res.result.phid),
        Err(_message) => Result::Err(String::from("Error getting user's phabricator ID"))
    }
}

#[derive(Deserialize, Debug)]
struct NetworkResponse<T> {
    pub error_code: Option<String>,
    pub result: Option<T>
}
