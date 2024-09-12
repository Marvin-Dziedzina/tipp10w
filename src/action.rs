#[derive(Debug)]
pub enum Action {
    Help,
    Print,
    Append,
    Delete,
    Exit,
    Invalid,
}
impl Action {
    pub fn from(from: &str) -> Self {
        match from.to_lowercase().as_str() {
            "h" => Action::Help,
            "p" => Action::Print,
            "a" => Action::Append,
            "d" => Action::Delete,
            "e" => Action::Exit,
            _ => Action::Invalid,
        }
    }
}
