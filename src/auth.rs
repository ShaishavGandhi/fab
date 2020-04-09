use crate::structs::{FabConfig, WhoAmIResponse};
use crate::WHO_AM_I;
use failure::Error;
use reqwest::RequestBuilder;
use serde::Deserialize;
use std::fs::{read_to_string, File};
use std::{fs, io};

pub fn init() -> Result<FabConfig, Error> {
    let existing_config = read_config();

    match existing_config {
        Ok(config) => Result::Ok(config),
        Err(_) => {
            println!(
                " _    _      _                            _         ______    _
| |  | |    | |                          | |        |  ___|  | |
| |  | | ___| | ___ ___  _ __ ___   ___  | |_ ___   | |_ __ _| |__
| |/\\| |/ _ \\ |/ __/ _ \\| '_ ` _ \\ / _ \\ | __/ _ \\  |  _/ _` | '_ \\
\\  /\\  /  __/ | (_| (_) | | | | | |  __/ | || (_) | | || (_| | |_) |
 \\/  \\/ \\___|_|\\___\\___/|_| |_| |_|\\___|  \\__\\___/  \\_| \\__,_|_.__/"
            );

            println!("Let's get you started!");
            println!("Enter the URL where your Phabricator instance is hosted. Example: https://phab.mycompany.com/");

            let hosted_instance = prompt_hosted_instance()?;

            let token = prompt_token(&hosted_instance)?;

            // Get user's details
            let phid = get_phid(&hosted_instance, &token)?;

            let config = FabConfig {
                hosted_instance,
                api_token: token,
                phid,
            };

            write_config(&config)?;

            Result::Ok(config)
        }
    }
}

/// Function that will execute the network request provided by RequestBuilder and
/// prompt for an API token if session is invalidated.
pub async fn send<T: serde::de::DeserializeOwned>(
    config: &FabConfig,
    request: RequestBuilder,
) -> Result<T, Error> {
    let request = request.try_clone().unwrap();
    let response = request.send().await?.json::<NetworkResponse<T>>().await?;

    if response.result.is_some() {
        return Result::Ok(response.result.unwrap());
    } else if response.error_code.is_some() {
        let error_code: String = response.error_code.unwrap();

        if error_code.eq("ERR-INVALID-AUTH") || error_code.eq("ERR-INVALID-SESSION") {
            println!("Your API Token has expired.");
            let current_config = read_config()?;
            let token = prompt_token(&config.hosted_instance);
            match token {
                Ok(token) => {
                    let new_config = FabConfig {
                        api_token: token,
                        hosted_instance: current_config.hosted_instance,
                        phid: current_config.phid,
                    };

                    write_config(&new_config)?;
                }
                Err(_err) => {}
            }
        }
    }
    Result::Err(failure::err_msg(
        "Token regenerated. Please try the command again",
    ))
}

/// Prompts for a token and writes the token to the configuration file.
fn prompt_token(hosted_instance: &str) -> Result<String, Error> {
    println!("Enter an API token that Fab can use. You can create one at {}settings/user/YOUR_USERNAME/page/apitokens", &hosted_instance);
    let mut api_token = String::new();

    io::stdin().read_line(&mut api_token)?;

    // Trim newlines
    api_token = api_token.trim().to_string();

    if api_token.is_empty() {
        return Result::Err(failure::err_msg("API Token cannot be null or empty"));
    }

    Result::Ok(api_token)
}

fn prompt_hosted_instance() -> Result<String, Error> {
    let mut hosted_instance = String::new();

    io::stdin().read_line(&mut hosted_instance)?;

    // Trim newlines
    hosted_instance = hosted_instance.trim().to_string();

    // Add a trailing `/` if needed.
    if !hosted_instance.ends_with('/') {
        hosted_instance.push('/')
    }

    // Make sure hosted instance is present.
    if hosted_instance.is_empty() {
        return Result::Err(failure::err_msg("Hosted instance cannot be empty"));
    }

    Ok(hosted_instance)
}

fn write_config(config: &FabConfig) -> Result<(), Error> {
    let path_buf = dirs::home_dir().unwrap();

    let home_dir = path_buf.to_str().unwrap();
    let fab_dir = format!("{}/.fab", home_dir);
    let config_file = format!("{}/.fab/config.json", home_dir);

    fs::create_dir_all(&fab_dir)?;

    serde_json::to_writer(
        &File::create(config_file).expect("Couldn't load file"),
        &config,
    )?;

    Ok(())
}

/// Tries to read the config file
fn read_config() -> Result<FabConfig, Error> {
    let path_buf = dirs::home_dir().unwrap();

    let home_dir = path_buf.to_str().unwrap();

    let config_file = format!("{}/.fab/config.json", home_dir);

    let contents = read_to_string(&config_file)?;

    let config: FabConfig = ::serde_json::from_str(&contents)?;

    Ok(config)
}

fn get_phid(hosted_instance: &str, api_token: &str) -> Result<String, Error> {
    let url = format!("{}{}", hosted_instance, WHO_AM_I);
    let json_body = json!({ "api.token": api_token });

    let response = reqwest::blocking::Client::new()
        .post(&url)
        .form(&json_body)
        .send()?
        .json::<WhoAmIResponse>()?;

    Ok(response.result.phid)
}

#[derive(Deserialize, Debug)]
struct NetworkResponse<T> {
    pub error_code: Option<String>,
    pub result: Option<T>,
}
