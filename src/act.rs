#[derive(Clone)]
pub enum Act {
    Main = 1,
    Support,
    Shared,
}

impl From<i64> for Act {
    fn from(value: i64) -> Act {
        match value {
            1 => Act::Main,
            2 => Act::Support,
            3 => Act::Shared,
            _ => Act::Main,
        }
    }
}

impl ToString for Act {
    fn to_string(&self) -> String {
        match self {
            Act::Main => String::from("Main Act"),
            Act::Support => String::from("Support Act"),
            Act::Shared => String::from("Shared Headliner"),
        }
    }
}
