use clap::ArgMatches;
use serde::{Deserialize};
use crate::structs::FabConfig;
use comfy_table::{Table, ContentArrangement, TableComponent, Cell, CellAlignment, Attribute, Color};
use comfy_table::presets::UTF8_FULL;

const MANIPHEST_SEARCH: &str = "api/maniphest.search";

pub fn process_task_command(matches: &ArgMatches, config: &FabConfig) {
    if let Some(matches) = matches.subcommand_matches("list") {
        process_list_tasks(matches, config)
    }
}

fn process_list_tasks(matches: &ArgMatches, config: &FabConfig) {
    let priority_pption = matches.value_of("priority").unwrap_or("high");
    let priority = Priority::get_value_for_name(&priority_pption.to_string())
        .expect("Couldn't parse priority. Must be one of ['unbreak-now', 'needs-triage', 'high', 'normal', 'low', 'wishlist']");

    let json_body = json!({
        "queryKey": "assigned",
        "constraints[priorities][0]": format!("{}", priority),
        "api.token": config.api_token
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
        .load_preset(UTF8_FULL)
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_style(TableComponent::BottomBorder, ' ')
        .set_style(TableComponent::TopBorder, ' ')
        .set_style(TableComponent::LeftBorder, ' ')
        .set_style(TableComponent::RightBorder, ' ')
        .set_style(TableComponent::HorizontalLines, ' ')
        .set_style(TableComponent::VerticalLines, ' ')
        .set_style(TableComponent::BottomBorderIntersections, ' ')
        .set_style(TableComponent::LeftBorderIntersections, ' ')
        .set_style(TableComponent::RightBorderIntersections, ' ')
        .set_style(TableComponent::TopBorderIntersections, ' ')
        .set_style(TableComponent::MiddleIntersections, ' ')
        .set_style(TableComponent::RightHeaderIntersection, ' ')
        .set_style(TableComponent::LeftHeaderIntersection, ' ')
        .set_style(TableComponent::TopLeftCorner, ' ')
        .set_style(TableComponent::TopRightCorner, ' ')
        .set_style(TableComponent::BottomLeftCorner, ' ')
        .set_style(TableComponent::BottomRightCorner, ' ');

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
