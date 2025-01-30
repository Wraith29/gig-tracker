use std::fmt::Display;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Act {
    Main = 0,
    Support,
    Shared,
}

impl Act {
    pub fn next(&self) -> Self {
        match self {
            Act::Main => Act::Support,
            Act::Support => Act::Shared,
            Act::Shared => Act::Shared,
        }
    }

    pub fn prev(&self) -> Self {
        match self {
            Act::Main => Act::Main,
            Act::Support => Act::Main,
            Act::Shared => Act::Support,
        }
    }
}

impl From<Act> for Option<usize> {
    fn from(value: Act) -> Self {
        match value {
            Act::Main => Some(0),
            Act::Support => Some(1),
            Act::Shared => Some(2),
        }
    }
}

impl From<i64> for Act {
    fn from(value: i64) -> Act {
        match value {
            0 => Act::Main,
            1 => Act::Support,
            2 => Act::Shared,
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
