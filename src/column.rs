use std::cmp::{max, min};

#[derive(Hash, PartialEq, Eq, Clone)]
pub enum FocusedApp {
    Artists,
    Venues,
    Gigs,
}

impl FocusedApp {
    pub fn down(&self) -> Self {
        match self {
            FocusedApp::Artists => FocusedApp::Venues,
            FocusedApp::Venues => FocusedApp::Gigs,
            FocusedApp::Gigs => FocusedApp::Artists,
        }
    }

    pub fn up(&self) -> Self {
        match self {
            FocusedApp::Artists => FocusedApp::Gigs,
            FocusedApp::Venues => FocusedApp::Artists,
            FocusedApp::Gigs => FocusedApp::Venues,
        }
    }
}

#[derive(PartialEq, Eq)]
pub enum Column {
    Left(FocusedApp),
    Middle(u8),
    Right(u8),
}

impl Column {
    pub fn up(&self) -> Self {
        match self {
            Column::Left(focused_app) => Column::Left(focused_app.up()),
            Column::Middle(col) => Column::Middle(min(1, col + 1)),
            Column::Right(col) => Column::Right(min(1, col + 1)),
        }
    }

    pub fn down(&self) -> Self {
        match self {
            Column::Left(focused_app) => Column::Left(focused_app.down()),
            Column::Middle(col) => Column::Middle(max(0, col - 1)),
            Column::Right(col) => Column::Right(max(0, col - 1)),
        }
    }

    pub fn left(&self) -> Self {
        match self {
            Column::Left(focused_app) => Column::Left(focused_app.clone()),
            Column::Middle(_) => Column::Left(FocusedApp::Artists),
            Column::Right(col) => Column::Middle(col.to_owned()),
        }
    }

    pub fn right(&self) -> Self {
        match self {
            Column::Left(_) => Column::Middle(0),
            Column::Middle(col) => Column::Right(col.to_owned()),
            Column::Right(col) => Column::Right(col.to_owned()),
        }
    }
}
