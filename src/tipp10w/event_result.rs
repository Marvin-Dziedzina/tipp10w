use crate::state::State;

#[derive(Debug, PartialEq)]
pub enum EventResult {
    Submit,
    SubmitState(State),
    Exit,
    None,
}
