use serde::{Deserialize};

#[derive(Deserialize, Debug)]
pub struct NetworkResponse<T> {
    pub error_code: Option<String>,
    pub result: Option<T>
}
