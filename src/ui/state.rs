#[derive(Copy)]
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
}

impl std::clone::Clone for State {
    fn clone(&self) -> Self {
        State { ..(*self) }
    }
}
