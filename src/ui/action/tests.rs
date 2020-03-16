use super::Action;
use std::convert::TryFrom;
use std::result::Result;

impl Action {
    pub fn to_json(&self) -> String {
        // TODO: Handle failure better
        serde_json::to_string(self).unwrap()
    }
}

#[test]
fn serde_action_quit() -> Result<(), String> {
    check_serde_action(Action::Quit)
}

#[test]
fn serde_action_send() -> Result<(), String> {
    check_serde_action(Action::Send {
        input: "test message".to_string(),
    })
}

fn check_serde_action(test_action: Action) -> Result<(), String> {
    let json = test_action.to_json();

    Action::try_from(json)
        .and_then(|action| {
            assert_eq!(action, test_action);
            Ok(())
        })
        .map_err(|_err| "Failed to parse Action JSON string".to_string())
}
