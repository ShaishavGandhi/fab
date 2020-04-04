use crate::preferences::Preferences;
use crate::structs::FabConfig;
use crate::{auth, NO_BORDER_PRESET};
use clap::ArgMatches;
use comfy_table::{Attribute, Cell, CellAlignment, Color, ContentArrangement, Table};
use failure::Error;
use serde::Deserialize;
use serde_json::{Map, Value};

const MANIPHEST_SEARCH: &str = "api/maniphest.search";

pub fn get_tasks(
    limit: &str,
    priorities: &[i32],
    config: &FabConfig,
) -> Result<Vec<Maniphest>, Error> {
    let mut map = Map::new();
    map.insert("queryKey".to_string(), Value::from("assigned"));
    map.insert(
        "api.token".to_string(),
        Value::from(config.api_token.clone()),
    );
    map.insert("limit".to_string(), Value::from(limit));

    for (i, &priority) in priorities.iter().enumerate() {
        map.insert(
            format!("constraints[priorities][{}]", i),
            Value::from(priority),
        );
    }

    let json_body = Value::Object(map);

    let url = format!("{}{}", &config.hosted_instance, MANIPHEST_SEARCH);

    let result = auth::send::<ManiphestSearchData>(
        config,
        reqwest::blocking::Client::new().post(&url).form(&json_body),
    )?;

    Ok(result.data)

    // match result {
    //     Ok(response) => Result::Ok(response.data),
    //     Err(_mess) => Result::Err(String::from("Error fetching tasks")),
    // }
}

pub fn render_tasks(tasks: &[Maniphest], config: &FabConfig) {
    let mut table = Table::new();

    table
        .load_preset(NO_BORDER_PRESET)
        .set_content_arrangement(ContentArrangement::Dynamic);

    for task in tasks {
        table.add_row(vec![
            Cell::new(&task.fields.priority.name)
                .bg(task.get_background())
                .fg(task.get_foreground())
                .set_alignment(CellAlignment::Center)
                .add_attribute(Attribute::Bold),
            Cell::new(&task.fields.name),
            Cell::new(&task.get_task_url(config)).add_attribute(Attribute::Bold),
        ]);
    }

    println!("{}", table)
}

pub fn process_task_command(
    matches: &ArgMatches,
    config: &FabConfig,
    preferences: &Preferences,
) -> Result<(), Error> {
    process_list_tasks(matches, config, preferences)
}

fn process_list_tasks(
    matches: &ArgMatches,
    config: &FabConfig,
    preferences: &Preferences,
) -> Result<(), Error> {
    let pref_limit: &str = &preferences.default_limit.to_string();
    let limit = matches.value_of("limit").unwrap_or(pref_limit);

    let priorities: Vec<_> = matches.values_of("priority")
        .expect("Couldn't parse priority. Must be one of ['unbreak-now', 'needs-triage', 'high', 'normal', 'low', 'wishlist']")
        .collect();

    let priorities: Vec<i32> = priorities
        .iter()
        .map(|priority| Priority::get_value_for_name(priority).unwrap())
        .collect();

    let tasks = get_tasks(limit, &priorities, config)?;
    render_tasks(&tasks, config);
    Ok(())
}

#[derive(Debug, Deserialize)]
struct ManiphestSearchData {
    data: Vec<Maniphest>,
}

#[derive(Debug, Deserialize)]
pub struct Maniphest {
    id: i32,
    fields: Fields,
}

impl Maniphest {
    fn get_task_url(&self, config: &FabConfig) -> String {
        return format!("{}T{}", &config.hosted_instance, &self.id);
    }

    fn get_background(&self) -> Color {
        let priority = &self.fields.priority.value;
        match priority {
            100 => Color::Red,
            90 => Color::Magenta,
            80 => Color::DarkRed,
            50 => Color::DarkYellow,
            25 => Color::Yellow,
            0 => Color::Cyan,
            _ => Color::Blue,
        }
    }

    fn get_foreground(&self) -> Color {
        let priority = &self.fields.priority.value;
        match priority {
            100 => Color::White,
            90 => Color::White,
            80 => Color::White,
            50 => Color::Black,
            25 => Color::Black,
            0 => Color::Black,
            _ => Color::White,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct Fields {
    name: String,
    status: Status,
    priority: Priority,
}

#[derive(Debug, Deserialize)]
pub struct Status {
    name: String,
    value: String,
}

#[derive(Debug, Deserialize)]
pub struct Priority {
    value: i32,
    name: String,
}

impl Priority {
    pub fn get_value_for_name(name: &str) -> Result<i32, Error> {
        match name.trim() {
            "unbreak-now" => Result::Ok(100),
            "needs-triage" => Result::Ok(90),
            "high" => Result::Ok(80),
            "normal" => Result::Ok(50),
            "low" => Result::Ok(25),
            "wishlist" => Result::Ok(0),
            _ => Result::Err(failure::err_msg("Unknown value of priority")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_value_for_name() {
        let priority = "unbreak-now";
        assert_eq!(
            100,
            Priority::get_value_for_name(&priority.to_string()).unwrap_or(-1)
        );

        let priority = "high";
        assert_eq!(
            80,
            Priority::get_value_for_name(&priority.to_string()).unwrap_or(80)
        );

        let priority = "needs-triage";
        assert_eq!(
            90,
            Priority::get_value_for_name(&priority.to_string()).unwrap_or(90)
        );

        let priority = "normal";
        assert_eq!(
            50,
            Priority::get_value_for_name(&priority.to_string()).unwrap_or(50)
        );

        let priority = "low";
        assert_eq!(
            25,
            Priority::get_value_for_name(&priority.to_string()).unwrap_or(25)
        );

        let priority = "wishlist";
        assert_eq!(
            0,
            Priority::get_value_for_name(&priority.to_string()).unwrap_or(0)
        );
    }

    #[test]
    fn maniphest_get_colo_unbreak_now() {
        let maniphest = Maniphest {
            id: 32,
            fields: Fields {
                name: String::from("Name of the task"),
                status: Status {
                    name: String::from("open"),
                    value: String::from("Open"),
                },
                priority: Priority {
                    value: 100,
                    name: String::from("Unbreak Now"),
                },
            },
        };

        assert_eq!(Color::Red, maniphest.get_background());
        assert_eq!(Color::White, maniphest.get_foreground());
    }

    #[test]
    fn maniphest_get_colo_high() {
        let maniphest = Maniphest {
            id: 32,
            fields: Fields {
                name: String::from("Name of the task"),
                status: Status {
                    name: String::from("open"),
                    value: String::from("Open"),
                },
                priority: Priority {
                    value: 80,
                    name: String::from("High"),
                },
            },
        };

        assert_eq!(Color::DarkRed, maniphest.get_background());
        assert_eq!(Color::White, maniphest.get_foreground());
    }

    #[test]
    fn maniphest_get_color_needs_triage() {
        let maniphest = Maniphest {
            id: 32,
            fields: Fields {
                name: String::from("Name of the task"),
                status: Status {
                    name: String::from("open"),
                    value: String::from("Open"),
                },
                priority: Priority {
                    value: 90,
                    name: String::from("Needs Triage"),
                },
            },
        };

        assert_eq!(Color::Magenta, maniphest.get_background());
        assert_eq!(Color::White, maniphest.get_foreground());
    }

    #[test]
    fn maniphest_get_color_normal() {
        let maniphest = Maniphest {
            id: 32,
            fields: Fields {
                name: String::from("Name of the task"),
                status: Status {
                    name: String::from("open"),
                    value: String::from("Open"),
                },
                priority: Priority {
                    value: 50,
                    name: String::from("Normal"),
                },
            },
        };

        assert_eq!(Color::DarkYellow, maniphest.get_background());
        assert_eq!(Color::Black, maniphest.get_foreground());
    }

    #[test]
    fn maniphest_get_color_low() {
        let maniphest = Maniphest {
            id: 32,
            fields: Fields {
                name: String::from("Name of the task"),
                status: Status {
                    name: String::from("open"),
                    value: String::from("Open"),
                },
                priority: Priority {
                    value: 25,
                    name: String::from("Low"),
                },
            },
        };

        assert_eq!(Color::Yellow, maniphest.get_background());
        assert_eq!(Color::Black, maniphest.get_foreground());
    }

    #[test]
    fn maniphest_get_color_wishlist() {
        let maniphest = Maniphest {
            id: 32,
            fields: Fields {
                name: String::from("Name of the task"),
                status: Status {
                    name: String::from("open"),
                    value: String::from("Open"),
                },
                priority: Priority {
                    value: 0,
                    name: String::from("Wishlist"),
                },
            },
        };

        assert_eq!(Color::Cyan, maniphest.get_background());
        assert_eq!(Color::Black, maniphest.get_foreground());
    }
}
