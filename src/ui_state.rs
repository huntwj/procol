#[derive(Copy)]
pub struct UiState {
    done: bool,
}

impl UiState {
    pub fn new() -> Self {
        UiState { done: false }
    }

    pub fn is_done(&self) -> bool {
        self.done
    }

    pub fn reduce_command(&self, cmd: &str) -> Self {
        if cmd == "quit" {
            UiState {
                done: true,
                ..(*self)
            }
        } else {
            *self
        }
    }
}

impl std::clone::Clone for UiState {
    fn clone(&self) -> Self {
        UiState { ..(*self) }
    }
}
