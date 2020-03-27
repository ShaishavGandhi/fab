use clap::ArgMatches;
use serde::{Deserialize};
use crate::structs::FabConfig;
use comfy_table::{Table, ContentArrangement, TableComponent, Cell, CellAlignment, Attribute, Color};
use comfy_table::presets::UTF8_FULL;
use crate::NO_BORDER_PRESET;

const MANIPHEST_SEARCH: &str = "api/maniphest.search";

pub fn process_task_command(matches: &ArgMatches, config: &FabConfig) {
    process_list_tasks(matches, config)
}

fn process_list_tasks(matches: &ArgMatches, config: &FabConfig) {
    let limit = matches.value_of("limit").expect("No limit specified for query");
    let priority = matches.value_of("priority").unwrap_or("high");
    let priority = Priority::get_value_for_name(&priority.to_string())
        .expect("Couldn't parse priority. Must be one of ['unbreak-now', 'needs-triage', 'high', 'normal', 'low', 'wishlist']");

    let json_body = json!({
        "queryKey": "assigned",
        "constraints[priorities][0]": format!("{}", priority),
        "api.token": config.api_token,
        "limit": limit
    });

    let url = format!("{}{}", &config.hosted_instance, MANIPHEST_SEARCH);

    let result = reqwest::blocking::Client::new()
        .post(&url)
        .form(&json_body)
        .send()
        .expect("Error fetching tasks from Conduit")
        .json::<ManiphestSearchResponse>()
        .expect("Error deserializing JSON for search tasks")
        .result;


    let tasks = result.data;
    render_tasks(&tasks, config)
}

fn render_tasks(tasks: &Vec<Maniphest>, config: &FabConfig) {
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
            Cell::new(&task.get_task_url(config))
        ]);
    }

    println!("{}", table)
}

#[derive(Debug, Deserialize)]
struct ManiphestSearchResponse {
    result: ManiphestSearchData
}

#[derive(Debug, Deserialize)]
struct ManiphestSearchData {
    data: Vec<Maniphest>
}

#[derive(Debug, Deserialize)]
struct Maniphest {
    id: i32,
    fields: Fields
}

impl Maniphest {
    fn get_task_url(&self, config: &FabConfig) -> String {
        return format!("{}T{}", &config.hosted_instance, &self.id);
    }

    fn get_background(&self) -> Color {
        let priority = &self.fields.priority.value;
        return match priority {
            100 => Color::Red,
            90 => Color::Magenta,
            80 => Color::DarkRed,
            50 => Color::DarkYellow,
            25 => Color::Yellow,
            0 => Color::Cyan,
            _ => Color::Blue
        }
    }

    fn get_foreground(&self) -> Color {
        let priority = &self.fields.priority.value;
        return match priority {
            100 => Color::White,
            90 => Color::White,
            80 => Color::White,
            50 => Color::Black,
            25 => Color::Black,
            0 => Color::Black,
            _ => Color::White
        }
    }
}

#[derive(Debug, Deserialize)]
struct Fields {
    name: String,
    status: Status,
    priority: Priority
}

#[derive(Debug, Deserialize)]
struct Status {
    name: String,
    value: String
}

#[derive(Debug, Deserialize)]
struct Priority {
    value: i32,
    name: String
}

impl Priority {
    pub fn get_value_for_name(name: &String) -> Result<i32, &str> {
        return match name.trim() {
            "unbreak-now" => Result::Ok(100),
            "needs-triage" => Result::Ok(90),
            "high" => Result::Ok(80),
            "normal" => Result::Ok(50),
            "low" => Result::Ok(25),
            "wishlist" => Result::Ok(0),
            _ => Result::Err("Unknown value of priority")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_value_for_name() {
        let priority = "unbreak-now";
        assert_eq!(100, Priority::get_value_for_name(&priority.to_string()).unwrap_or(-1));

        let priority = "high";
        assert_eq!(80, Priority::get_value_for_name(&priority.to_string()).unwrap_or(80));

        let priority = "needs-triage";
        assert_eq!(90, Priority::get_value_for_name(&priority.to_string()).unwrap_or(90));

        let priority = "normal";
        assert_eq!(50, Priority::get_value_for_name(&priority.to_string()).unwrap_or(50));

        let priority = "low";
        assert_eq!(25, Priority::get_value_for_name(&priority.to_string()).unwrap_or(25));

        let priority = "wishlist";
        assert_eq!(0, Priority::get_value_for_name(&priority.to_string()).unwrap_or(0));
    }

    #[test]
    fn maniphest_get_colo_unbreak_now() {
        let maniphest = Maniphest {
            id: 32,
            fields: Fields {
                name: String::from("Name of the task"),
                status: Status {
                    name: String::from("open"),
                    value: String::from("Open")
                },
                priority: Priority {
                    value: 100,
                    name: String::from("Unbreak Now")
                }
            }
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
                    value: String::from("Open")
                },
                priority: Priority {
                    value: 80,
                    name: String::from("High")
                }
            }
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
                    value: String::from("Open")
                },
                priority: Priority {
                    value: 90,
                    name: String::from("Needs Triage")
                }
            }
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
                    value: String::from("Open")
                },
                priority: Priority {
                    value: 50,
                    name: String::from("Normal")
                }
            }
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
                    value: String::from("Open")
                },
                priority: Priority {
                    value: 25,
                    name: String::from("Low")
                }
            }
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
                    value: String::from("Open")
                },
                priority: Priority {
                    value: 0,
                    name: String::from("Wishlist")
                }
            }
        };

        assert_eq!(Color::Cyan, maniphest.get_background());
        assert_eq!(Color::Black, maniphest.get_foreground());
    }
}
