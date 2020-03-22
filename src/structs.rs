use serde::{Deserialize, Serialize};
use comfy_table::Color;

#[derive(Deserialize, Serialize, Debug)]
pub struct FabConfig {
    pub hosted_instance: String,
    pub api_token: String,
    pub phid: String
}

#[derive(Deserialize, Debug)]
pub struct RevisionResponse {
    pub result: RevisionData,
}

#[derive(Deserialize, Debug)]
pub struct RevisionData {
    pub data: Vec<Revision>
}

#[derive(Deserialize, Debug)]
pub struct WhoAmIResponse {
    pub result: UserResponse
}

#[derive(Deserialize, Debug)]
pub struct UserResponse {
    pub phid: String,
    #[serde(rename = "userName")]
    pub user_ame: String
}

#[derive(Deserialize, Debug)]
pub struct Revision {
    pub id: i32,
    pub fields: Fields
}

impl Revision {

    pub fn url(&self, config: &FabConfig) -> String {
        return format!("{}D{}", &config.hosted_instance, &self.id)
    }

    pub fn get_background(&self) -> Color {
        let status = &self.fields.status.name;
        return if status.eq("Needs Review") {
            Color::Magenta
        } else if status.eq("Accepted") {
            Color::Green
        } else if status.eq("Needs Revision") {
            Color::Red
        } else if status.eq("Changes Planned") {
            Color::Red
        } else {
            Color::Yellow
        };
    }

    pub fn get_foreground(&self) -> Color {
        let status = &self.fields.status.name;
        return if status.eq("Needs Review") {
            Color::White
        } else if status.eq("Accepted") {
            Color::Black
        } else if status.eq("Needs Revision") {
            Color::White
        } else if status.eq("Changes Planned") {
            Color::White
        } else {
            Color::Black
        };
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
    fn test_get_foreground_background_accepted() {
        let revision = Revision { id : 1, fields : Fields {
            title : String::from("Sample diff"),
            status: Status {
                name: String::from("Accepted"),
                closed: false
            }
        }};

        assert_eq!(Color::Green, revision.get_background());
        assert_eq!(Color::Black, revision.get_foreground());
    }

    #[test]
    fn test_get_foreground_background_needs_revision() {
        let revision = Revision { id : 1, fields : Fields {
            title : String::from("Sample diff"),
            status: Status {
                name: String::from("Needs Revision"),
                closed: false
            }
        }};

        assert_eq!(Color::Red, revision.get_background());
        assert_eq!(Color::White, revision.get_foreground());
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

        assert_eq!(Color::Magenta, revision.get_background());
        assert_eq!(Color::White, revision.get_foreground());
    }
}
