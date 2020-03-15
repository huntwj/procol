use super::action::Action;
use serde::Serialize;

#[derive(Serialize)]
pub struct State {
    done: bool,
    sent: Vec<String>,
}

impl State {
    pub fn new() -> Self {
        State {
            done: false,
            sent: vec![],
        }
    }

    pub fn is_done(&self) -> bool {
        self.done
    }

    pub fn reduce_command(&self, action: &Action) -> Self {
        match action {
            Action::Quit => State {
                done: true,
                sent: self.sent.clone(),
            },
            Action::Send { input } => {
                let mut sent: Vec<String> = self.sent.clone();
                sent.push(input.to_string());
                State { sent, ..(*self) }
            }
        }
    }

    pub fn to_json(&self) -> String {
        // TODO: Handle error case better here
        serde_json::to_string(self).unwrap()
    }
}

impl std::clone::Clone for State {
    fn clone(&self) -> Self {
        State {
            sent: self.sent.clone(),
            ..(*self)
        }
    }
}
