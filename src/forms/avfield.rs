pub enum AvFieldError {
    Name(String),
    City(String),
    Save(String),
}

pub enum AvField {
    None,
    Name,
    City,
    Save,
}

impl AvField {
    pub fn next(&self) -> Self {
        match self {
            AvField::None => AvField::Name,
            AvField::Name => AvField::City,
            AvField::City => AvField::Save,
            AvField::Save => AvField::Save,
        }
    }

    pub fn prev(&self) -> Self {
        match self {
            AvField::None => AvField::None,
            AvField::Name => AvField::None,
            AvField::City => AvField::Name,
            AvField::Save => AvField::City,
        }
    }
}
