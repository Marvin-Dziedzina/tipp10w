#[derive(Debug)]
pub enum Action {
    Show,
    Append,
    Delete,
    Exit,
}
impl Action {
    pub fn from(from: &str) -> Self {
        match from.to_lowercase().as_str() {
            "s" => Action::Show,
            "a" => Action::Append,
            "d" => Action::Delete,
            "e" => Action::Exit,
            _ => panic!("Invalid action selected!"),
        }
    }
}
