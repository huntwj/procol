use serde::Serialize;

#[derive(Copy, Serialize)]
pub struct State {
    done: bool,
}

impl State {
    pub fn new() -> Self {
        State { done: false }
    }

    pub fn is_done(&self) -> bool {
        self.done
    }

    pub fn reduce_command(&self, cmd: &str) -> Self {
        if cmd == "quit" {
            State {
                done: true,
                ..(*self)
            }
        } else {
            *self
        }
    }

    pub fn to_json(&self) -> String {
        // TODO: Handle error case better here
        serde_json::to_string(self).unwrap()
    }
}

impl std::clone::Clone for State {
    fn clone(&self) -> Self {
        State { ..(*self) }
    }
}
