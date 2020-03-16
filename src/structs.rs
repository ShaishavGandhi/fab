use serde::{Deserialize};
use console::{Style, Color, StyledObject};

#[derive(Deserialize, Debug)]
pub struct RevisionResponse {
    pub result: RevisionData,
}

#[derive(Deserialize, Debug)]
pub struct RevisionData {
    pub data: Vec<Revision>
}

#[derive(Deserialize, Debug)]
pub struct Revision {
    pub id: i32,
    pub fields: Fields
}

impl Revision {
    pub fn status(&self) -> StyledObject<&String> {
        let status = &self.fields.status.name;
        let style = if status.eq("Needs Review") {
            Style::new().bg(Color::Yellow).black()
        } else if status.eq("Accepted") {
            Style::new().bg(Color::Green).black()
        } else if status.eq("Needs Revision") {
            Style::new().bg(Color::Red).black()
        } else if status.eq("Changes Planned") {
            Style::new().bg(Color::Magenta).black()
        } else {
            Style::new().bg(Color::Yellow).black()
        };
        return style.apply_to(status);
    }
}

#[derive(Deserialize, Debug)]
pub struct Fields {
    pub title: String,
    pub status: Status
}

#[derive(Deserialize, Debug)]
pub struct Status {
    pub name: String,
    pub closed: bool
}
