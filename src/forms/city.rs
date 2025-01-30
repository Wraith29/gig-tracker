use crossterm::event::{Event, KeyCode, KeyModifiers};
use ratatui::{
    layout::{Constraint, Layout, Rect},
    Frame,
};
use sqlx::{Pool, Sqlite};

use crate::{city::City, dataset::DataSet, error::Error};

use super::{
    savebutton::{SaveButton, SaveButtonEvent},
    textinput::{TextInput, TextInputEvent},
};

enum CityFieldError {
    Name(String),
    Save(String),
}

enum CityField {
    None,
    Name,
    Save,
}

impl CityField {
    fn next(&self) -> Self {
        match self {
            CityField::None => CityField::Name,
            CityField::Name => CityField::Save,
            CityField::Save => CityField::Save,
        }
    }

    fn prev(&self) -> Self {
        match self {
            CityField::None => CityField::None,
            CityField::Name => CityField::None,
            CityField::Save => CityField::Name,
        }
    }
}

pub struct CityForm<'a> {
    pool: Pool<Sqlite>,
    current_field: CityField,
    name: TextInput<'a>,
    save: SaveButton,
}

impl CityForm<'_> {
    pub fn new(pool: Pool<Sqlite>) -> Self {
        Self {
            pool,
            current_field: CityField::None,
            name: TextInput::new("Name"),
            save: SaveButton::new(),
        }
    }

    fn change_focus(&mut self, new_focus: CityField) {
        match self.current_field {
            CityField::Name => self.name.unfocus(),
            CityField::Save => self.save.unfocus(),
            _ => {}
        }

        self.current_field = new_focus;

        match self.current_field {
            CityField::Name => self.name.focus(),
            CityField::Save => self.save.focus(),
            _ => {}
        }
    }

    pub async fn handle_event(&mut self, event: Event) -> Result<bool, Error> {
        if let Event::Key(key) = event {
            match (key.modifiers, key.code) {
                (_, KeyCode::Enter) => {
                    if let CityField::None = self.current_field {
                        self.change_focus(self.current_field.next());
                        return Ok(false);
                    }
                }
                (KeyModifiers::CONTROL, KeyCode::Char('j')) => {
                    self.change_focus(self.current_field.next());
                    return Ok(false);
                }
                (KeyModifiers::CONTROL, KeyCode::Char('k')) => {
                    self.change_focus(self.current_field.prev());
                    return Ok(false);
                }
                _ => {}
            }
        }

        match self.current_field {
            CityField::Name => {
                if let Some(input_event) = self.name.handle_event(&event) {
                    match input_event {
                        TextInputEvent::Escape => {
                            self.name.unfocus();
                        }
                        TextInputEvent::Save => {
                            self.change_focus(self.current_field.next());
                        }
                    }
                }
            }
            CityField::Save => {
                if let Some(save_event) = self.save.handle_event(&event) {
                    match save_event {
                        SaveButtonEvent::Escape => self.save.unfocus(),
                        SaveButtonEvent::Save => {
                            if let Some(field_error) = self.save_value().await? {
                                match field_error {
                                    CityFieldError::Name(error) => self.name.set_err(error),
                                    CityFieldError::Save(error) => self.save.set_err(error),
                                }
                            } else {
                                return Ok(true);
                            }
                        }
                    }
                }
            }
            _ => {}
        }

        Ok(false)
    }

    pub fn render(&mut self, frame: &mut Frame, area: Rect) {
        let [top, bottom] =
            Layout::vertical(vec![Constraint::Length(3), Constraint::Length(3)]).areas(area);

        self.name.render(frame, top);
        self.save.render(frame, bottom);
    }

    async fn save_value(&self) -> Result<Option<CityFieldError>, Error> {
        let city_name = match self.name.get_value() {
            Some(name) => name,
            None => {
                return Ok(Some(CityFieldError::Name(
                    "Field \"Name\" cannot be empty".into(),
                )));
            }
        };

        let city = City::new(city_name);

        match City::save(city, &self.pool).await {
            Ok(_) => Ok(None),
            Err(err) => Ok(Some(CityFieldError::Save(err.to_string()))),
        }
    }
}
