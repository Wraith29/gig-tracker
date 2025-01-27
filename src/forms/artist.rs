use crossterm::event::{Event, KeyCode, KeyModifiers};
use ratatui::{
    layout::{Constraint, Layout, Rect},
    Frame,
};
use sqlx::{Pool, Sqlite};

use crate::{
    artist::{self, Artist},
    city::City,
    dataset::DataSet,
    error::Error,
};

use super::{
    listinput::{ListInput, ListInputEvent},
    savebutton::{SaveButton, SaveButtonEvent},
    textinput::{TextInput, TextInputEvent},
};

enum Field {
    None,
    Name,
    City,
    Save,
}

impl Field {
    fn next(&self) -> Self {
        match self {
            Field::None => Field::Name,
            Field::Name => Field::City,
            Field::City => Field::Save,
            Field::Save => Field::Save,
        }
    }

    fn prev(&self) -> Self {
        match self {
            Field::None => Field::None,
            Field::Name => Field::None,
            Field::City => Field::Name,
            Field::Save => Field::City,
        }
    }
}

pub struct ArtistForm<'a> {
    pool: Pool<Sqlite>,
    current_field: Field,

    name: TextInput<'a>,
    city: ListInput<'a, City>,
    save: SaveButton,
}

impl ArtistForm<'_> {
    pub async fn new(pool: Pool<Sqlite>) -> Result<Self, Error> {
        let list_input = ListInput::new("City", &pool).await?;

        Ok(Self {
            pool,
            current_field: Field::None,
            name: TextInput::new("Name"),
            city: list_input,
            save: SaveButton::new(),
        })
    }

    fn change_focus(&mut self, new_focus: Field) {
        match self.current_field {
            Field::Name => self.name.unfocus(),
            Field::City => self.city.unfocus(),
            Field::Save => self.save.unfocus(),
            _ => {}
        }

        self.current_field = new_focus;

        match self.current_field {
            Field::Name => self.name.focus(),
            Field::City => self.city.focus(),
            Field::Save => self.save.focus(),
            _ => {}
        }
    }

    pub async fn handle_event(&mut self, event: Event) -> Result<(), Error> {
        if let Event::Key(key) = event {
            match (key.modifiers, key.code) {
                (KeyModifiers::CONTROL, KeyCode::Char('j')) => {
                    self.change_focus(self.current_field.next());
                }
                (KeyModifiers::CONTROL, KeyCode::Char('k')) => {
                    self.change_focus(self.current_field.prev());
                }
                _ => {}
            }
        }

        match self.current_field {
            Field::Name => {
                if let Some(input_event) = self.name.handle_event(&event) {
                    match input_event {
                        TextInputEvent::Escape => self.name.unfocus(),
                        TextInputEvent::Save => {
                            self.change_focus(self.current_field.next());
                        }
                    }
                }
            }
            Field::City => {
                if let Some(list_event) = self.city.handle_event(&event) {
                    match list_event {
                        ListInputEvent::Escape => self.city.unfocus(),
                        ListInputEvent::Select => {
                            self.change_focus(self.current_field.next());
                        }
                    }
                }
            }
            Field::Save => {
                if let Some(save_event) = self.save.handle_event(&event) {
                    match save_event {
                        SaveButtonEvent::Escape => self.save.unfocus(),
                        SaveButtonEvent::Save => {
                            self.save_value().await?;
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

    /// The `Some` value of a return is an error message, because of bad data
    async fn save_value(&self) -> Result<Option<String>, Error> {
        let artist_name = match self.name.get_value() {
            Some(name) => name,
            None => return Ok(Some(String::from("Missing required field \"Artist Name\""))),
        };

        let city = match self.city.get_value() {
            Some(city) => city,
            None => return Ok(Some(String::from("Missing required field \"City\""))),
        };

        let artist = Artist::new(artist_name, city.city_id);

        Artist::save(artist, pool);

        Ok(None)
    }
}
