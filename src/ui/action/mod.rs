#[cfg(test)]
mod tests;

use serde::{Deserialize, Serialize};
use std::convert::TryFrom;

#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[serde(tag = "msg", rename_all = "camelCase")]
pub enum Action {
    Quit,
    Send { input: String },
}

impl Action {
    pub fn to_json(&self) -> String {
        // TODO: Handle failure better
        serde_json::to_string(self).unwrap()
    }
}

impl TryFrom<String> for Action {
    type Error = serde_json::error::Error;

    fn try_from(command: String) -> Result<Self, Self::Error> {
        serde_json::from_str(&command)
    }
}
