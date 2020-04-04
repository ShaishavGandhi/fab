use serde::{Deserialize, Serialize};
use clap::ArgMatches;
use dialoguer::Checkboxes;
use dialoguer::theme::ColorfulTheme;
use console::style;
use std::io;

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

pub fn process_configuration(_matches: &ArgMatches) {
    let current_preferences = get_preferences().expect("Couldn't get current preferences");

    let possible_priorities = vec!["unbreak-now", "needs-triage", "high", "normal", "low", "wishlist"];

    println!("{}", style("Chose the task priorities to include in your summary").bold().underlined());
    println!("(Press space to select a priority)");

    let result = get_chosen_priorities(&possible_priorities, &current_preferences.summary_task_priority);


}

fn get_chosen_priorities(possible_priorities: &Vec<&str>, current_priorities: &Vec<String>) -> io::Result<Vec<String>> {
    let theme = &ColorfulTheme::default();
    let checked_priorities: Vec<(&str, bool)> = possible_priorities.iter()
        .map(|&priority| (priority, current_priorities.contains(&priority.to_string())))
        .collect();

    let result = Checkboxes::with_theme(theme)
        .items_checked(&checked_priorities)
        .interact()?;

    let mut chosen_priorites: Vec<String> = Vec::with_capacity(result.len());

    for i in result  {
        chosen_priorites.push(possible_priorities[i].to_string())
    }

    Ok(chosen_priorites)
}