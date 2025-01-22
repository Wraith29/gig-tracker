use std::fmt::Display;

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

impl Display for Act {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Act::Main => write!(f, "Main Act"),
            Act::Support => write!(f, "Support Act"),
            Act::Shared => write!(f, "Shared Headliner"),
        }
    }
}
