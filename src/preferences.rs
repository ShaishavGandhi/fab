use clap::ArgMatches;
use console::style;
use dialoguer::theme::ColorfulTheme;
use dialoguer::{Checkboxes, Input};
use failure::Error;
use serde::{Deserialize, Serialize};
use std::io;

/// Get user's preferences
pub fn get_preferences() -> Result<Preferences, Error> {
    let prefs = confy::load::<Preferences>("fab")?;
    // match prefs {
    //     Ok(pref) => Result::Ok(pref),
    //     Err(_err) => Result::Err(String::from("Couldn't load preferences")),
    // }
    Ok(prefs)
}

/// Stores new preferences to disk
pub fn set_preferences(preferences: &Preferences) -> Result<(), Error> {
    confy::store("fab", preferences)?;
    Ok(())
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

pub fn process_configuration(_matches: &ArgMatches) -> Result<(), Error> {
    let current_preferences = get_preferences()?;

    let possible_priorities = vec![
        "unbreak-now",
        "needs-triage",
        "high",
        "normal",
        "low",
        "wishlist",
    ];

    println!(
        "{}",
        style("Choose the task priorities to include in your summary")
            .bold()
            .underlined()
    );
    println!("(Press space to select a priority)");

    let summary_priorities = get_chosen_priorities(
        &possible_priorities,
        &current_preferences.summary_task_priority,
    )?;

    println!(
        "{}",
        style("Choose the task priorities as your default for `fab tasks`")
            .bold()
            .underlined()
    );
    println!("(Press space to select a priority)");

    let default_task_priorities = get_chosen_priorities(
        &possible_priorities,
        &current_preferences.default_task_priority,
    )?;

    println!(
        "{}",
        style("Choose default limit for Fab results")
            .bold()
            .underlined()
    );

    let default_limit = Input::with_theme(&ColorfulTheme::default())
        .with_initial_text(&current_preferences.default_limit.to_string())
        .interact()?;

    let new_preferences = Preferences {
        summary_task_priority: summary_priorities,
        default_task_priority: default_task_priorities,
        default_limit,
    };

    set_preferences(&new_preferences)
}

fn get_chosen_priorities(
    possible_priorities: &[&str],
    current_priorities: &[String],
) -> io::Result<Vec<String>> {
    let theme = &ColorfulTheme::default();
    let checked_priorities: Vec<(&str, bool)> = possible_priorities
        .iter()
        .map(|&priority| (priority, current_priorities.contains(&priority.to_string())))
        .collect();

    let result = Checkboxes::with_theme(theme)
        .items_checked(&checked_priorities)
        .interact()?;

    let mut chosen_priorites: Vec<String> = Vec::with_capacity(result.len());

    for i in result {
        chosen_priorites.push(possible_priorities[i].to_string())
    }

    Ok(chosen_priorites)
}
