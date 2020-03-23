use super::State;

#[test]
fn initial_state_is_not_done() {
    let state = State::new();

    assert!(!state.is_done());
}
