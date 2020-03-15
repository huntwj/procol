use serde::{Deserialize, Serialize};
use std::convert::TryFrom;

#[derive(Deserialize, Serialize)]
#[serde(tag = "msg")]
pub enum Action {
    Quit,
    Send { input: String },
}

impl TryFrom<String> for Action {
    type Error = serde_json::error::Error;

    fn try_from(command: String) -> Result<Self, Self::Error> {
        serde_json::from_str(&command)
    }
}
