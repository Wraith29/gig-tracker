use crossterm::event::{Event, KeyCode, KeyModifiers};
use ratatui::{
    layout::{Constraint, Layout, Rect},
    Frame,
};
use sqlx::{Pool, Sqlite};

use crate::{city::City, dataset::DataSet, error::Error, venue::Venue};

use super::{
    avfield::{AvField, AvFieldError},
    listinput::{ListInput, ListInputEvent},
    savebutton::{SaveButton, SaveButtonEvent},
    textinput::{TextInput, TextInputEvent},
};

pub struct VenueForm<'a> {
    pool: Pool<Sqlite>,
    current_field: AvField,

    name: TextInput<'a>,
    city: ListInput<'a, City>,
    save: SaveButton,
}

impl VenueForm<'_> {
    pub async fn new(pool: Pool<Sqlite>) -> Result<Self, Error> {
        let list_input = ListInput::new("City", &pool).await?;

        Ok(Self {
            pool,
            current_field: AvField::None,
            name: TextInput::new("Name"),
            city: list_input,
            save: SaveButton::new(),
        })
    }

    fn change_focus(&mut self, new_focus: AvField) {
        match self.current_field {
            AvField::Name => self.name.unfocus(),
            AvField::City => self.city.unfocus(),
            AvField::Save => self.save.unfocus(),
            _ => {}
        }

        self.current_field = new_focus;

        match self.current_field {
            AvField::Name => self.name.focus(),
            AvField::City => self.city.focus(),
            AvField::Save => self.save.focus(),
            _ => {}
        }
    }

    pub async fn handle_event(&mut self, event: Event) -> Result<(), Error> {
        if let Event::Key(key) = event {
            match (key.modifiers, key.code) {
                (_, KeyCode::Enter) => {
                    if let AvField::None = self.current_field {
                        self.change_focus(self.current_field.next());
                        return Ok(());
                    }
                }

                (KeyModifiers::CONTROL, KeyCode::Char('j')) => {
                    self.change_focus(self.current_field.next());
                    return Ok(());
                }
                (KeyModifiers::CONTROL, KeyCode::Char('k')) => {
                    self.change_focus(self.current_field.prev());
                    return Ok(());
                }
                _ => {}
            }
        }

        match self.current_field {
            AvField::Name => {
                if let Some(input_event) = self.name.handle_event(&event) {
                    match input_event {
                        TextInputEvent::Escape => self.name.unfocus(),
                        TextInputEvent::Save => {
                            self.change_focus(self.current_field.next());
                        }
                    }
                }
            }
            AvField::City => {
                if let Some(list_event) = self.city.handle_event(&event) {
                    match list_event {
                        ListInputEvent::Escape => self.city.unfocus(),
                        ListInputEvent::Select => {
                            self.change_focus(self.current_field.next());
                        }
                    }
                }
            }
            AvField::Save => {
                if let Some(save_event) = self.save.handle_event(&event) {
                    match save_event {
                        SaveButtonEvent::Escape => self.save.unfocus(),
                        SaveButtonEvent::Save => {
                            if let Some(field_error) = self.save_value().await? {
                                match field_error {
                                    AvFieldError::Name(err) => {
                                        self.name.set_err(err);
                                    }
                                    AvFieldError::City(err) => {
                                        self.city.set_err(err);
                                    }
                                    AvFieldError::Save(err) => {
                                        self.save.set_err(err);
                                    }
                                }
                            };
                        }
                    }
                }
            }

            _ => {}
        }

        Ok(())
    }

    pub fn render(&mut self, frame: &mut Frame, area: Rect) {
        let [top, middle, bottom] = Layout::vertical(vec![
            Constraint::Length(3),
            Constraint::Fill(1),
            Constraint::Length(3),
        ])
        .areas(area);

        self.name.render(frame, top);
        self.city.render(frame, middle);
        self.save.render(frame, bottom);
    }

    // The `Some` value of a return is an error message, because of bad data
    async fn save_value(&self) -> Result<Option<AvFieldError>, Error> {
        let artist_name = match self.name.get_value() {
            Some(name) => name,
            None => {
                return Ok(Some(AvFieldError::Name(
                    "Field \"Name\" cannot be empty".into(),
                )))
            }
        };

        let city = match self.city.get_value() {
            Some(city) => city,
            None => {
                return Ok(Some(AvFieldError::City(
                    "Field \"City\" cannot be empty".into(),
                )))
            }
        };

        let venue = Venue::new(artist_name, city.city_id);

        match Venue::save(venue, &self.pool).await {
            Ok(_) => Ok(None),
            Err(err) => Ok(Some(AvFieldError::Save(err.to_string()))),
        }
    }
}
