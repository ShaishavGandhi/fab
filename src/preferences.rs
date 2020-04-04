use serde::{Deserialize, Serialize};

/// Get user's preferences
pub fn get_preferences() -> Result<Preferences, String> {
    let prefs = confy::load::<Preferences>("fab");
    match prefs {
        Ok(pref) => Result::Ok(pref),
        Err(_err) => Result::Err(String::from("Couldn't load preferences")),
    }
}

/// Stores new preferences to disk
pub fn set_preferences(preferences: &Preferences) {
    confy::store("fab", preferences).expect("Failed to set preferences");
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Preferences {
    pub summary_task_priority: Vec<String>,
    pub default_task_priority: Vec<String>,
    pub default_limit: i32,
}

impl ::std::default::Default for Preferences {
    fn default() -> Self {
        Self {
            summary_task_priority: vec![String::from("high"), String::from("needs-triage")],
            default_task_priority: vec![String::from("high")],
            default_limit: 20,
        }
    }
}
