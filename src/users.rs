use crate::auth;
use crate::structs::FabConfig;
use failure::Error;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

pub async fn get_user(username: &str, config: &FabConfig) -> Result<User, Error> {
    let mut map = Map::new();
    map.insert(
        "api.token".to_string(),
        Value::from(config.api_token.clone()),
    );
    map.insert(
        "constraints[usernames][0]".to_string(),
        Value::from(username),
    );

    let url = format!("{}{}", &config.hosted_instance, "api/user.search");

    let json_body = Value::Object(map);

    let result =
        auth::send::<UserSearchData>(config, reqwest::Client::new().post(&url).form(&json_body))
            .await?
            .data
            .into_iter()
            .next()
            .unwrap();

    Ok(result)
}

#[derive(Deserialize, Serialize, Debug)]
struct UserSearchData {
    data: Vec<User>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct User {
    pub id: i32,
    pub phid: String,
}
