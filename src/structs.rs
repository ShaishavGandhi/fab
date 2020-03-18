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

    pub fn url(&self) -> StyledObject<String> {
        return Style::new().bold().apply_to(format!("https://code.uberinternal.com/{}", &self.id))
    }

    pub fn status(&self) -> StyledObject<String> {
        let status = &self.fields.status.name;
        return Self::get_style(status).apply_to(format!(" {} ", status));
    }

    fn get_style(status: &String) -> Style {
        let style = if status.eq("Needs Review") {
            Style::new().bg(Color::Magenta).white()
        } else if status.eq("Accepted") {
            Style::new().bg(Color::Green).black()
        } else if status.eq("Needs Revision") {
            Style::new().bg(Color::Red).white()
        } else if status.eq("Changes Planned") {
            Style::new().bg(Color::Red).white()
        } else {
            Style::new().bg(Color::Yellow).black()
        };
        return style.bold()
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

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_status_colors_accepted() {
        let revision = Revision { id : 1, fields : Fields {
            title : String::from("Sample diff"),
            status: Status {
                name: String::from("Accepted"),
                closed: false
            }
        }};

        let styled_status = Revision::get_style(&revision.fields.status.name);
        let expected_style = Style::new().bg(Color::Green).black().bold();
        assert_eq!(expected_style, styled_status);
    }

    #[test]
    fn test_status_colors_needs_revision() {
        let revision = Revision { id : 1, fields : Fields {
            title : String::from("Sample diff"),
            status: Status {
                name: String::from("Needs Revision"),
                closed: false
            }
        }};

        let styled_status = Revision::get_style(&revision.fields.status.name);
        let expected_style = Style::new().bg(Color::Red).white().bold();
        assert_eq!(expected_style, styled_status);
    }

    #[test]
    fn test_status_colors_needs_review() {
        let revision = Revision { id : 1, fields : Fields {
            title : String::from("Sample diff"),
            status: Status {
                name: String::from("Needs Review"),
                closed: false
            }
        }};

        let styled_status = Revision::get_style(&revision.fields.status.name);
        let expected_style = Style::new().bg(Color::Magenta).white().bold();
        assert_eq!(expected_style, styled_status);
    }
}
