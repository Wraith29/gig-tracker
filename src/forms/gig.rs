use crate::{artist::Artist, dataset::DataSet, date::Date, error::Error, gig::Gig, venue::Venue};
use crossterm::event::{Event, KeyCode, KeyModifiers};
use ratatui::{
    layout::{Constraint, Layout, Rect},
    Frame,
};
use sqlx::{Pool, Sqlite};

use super::{
    actinput::{ActInput, ActInputEvent},
    listinput::{ListInput, ListInputEvent},
    savebutton::{SaveButton, SaveButtonEvent},
    textinput::{TextInput, TextInputEvent},
};

enum GigFieldError {
    Artist(String),
    Venue(String),
    Date(String),
    Act(String),
    Save(String),
}

enum GigField {
    None,
    Artist,
    Venue,
    Date,
    Act,
    Save,
}

impl GigField {
    fn next(&self) -> Self {
        match self {
            GigField::None => GigField::Artist,
            GigField::Artist => GigField::Venue,
            GigField::Venue => GigField::Date,
            GigField::Date => GigField::Act,
            GigField::Act => GigField::Save,
            GigField::Save => GigField::Save,
        }
    }

    fn prev(&self) -> Self {
        match self {
            GigField::None => GigField::None,
            GigField::Artist => GigField::None,
            GigField::Venue => GigField::Artist,
            GigField::Date => GigField::Venue,
            GigField::Act => GigField::Date,
            GigField::Save => GigField::Act,
        }
    }
}

pub struct GigForm<'a> {
    pool: Pool<Sqlite>,
    current_field: GigField,

    artist_input: ListInput<'a, Artist>,
    venue_input: ListInput<'a, Venue>,
    date_input: TextInput<'a>,
    act_input: ActInput,
    save_button: SaveButton,
}

impl GigForm<'_> {
    pub async fn new(pool: Pool<Sqlite>) -> Result<Self, Error> {
        let artist_input = ListInput::new("Artist", &pool).await?;
        let venue_input = ListInput::new("Venue", &pool).await?;
        let date_input = TextInput::new("Date");
        let act_input = ActInput::new();
        let save_button = SaveButton::new();

        Ok(Self {
            pool,
            current_field: GigField::None,
            artist_input,
            venue_input,
            date_input,
            act_input,
            save_button,
        })
    }

    fn change_focus(&mut self, new_focus: GigField) {
        match self.current_field {
            GigField::Artist => self.artist_input.unfocus(),
            GigField::Venue => self.venue_input.unfocus(),
            GigField::Date => self.date_input.unfocus(),
            GigField::Act => self.act_input.unfocus(),
            GigField::Save => self.save_button.unfocus(),
            _ => {}
        }

        self.current_field = new_focus;

        match self.current_field {
            GigField::Artist => self.artist_input.focus(),
            GigField::Venue => self.venue_input.focus(),
            GigField::Date => self.date_input.focus(),
            GigField::Act => self.act_input.focus(),
            GigField::Save => self.save_button.focus(),
            _ => {}
        }
    }

    pub async fn handle_event(&mut self, event: Event) -> Result<(), Error> {
        if let Event::Key(key) = event {
            match (key.modifiers, key.code) {
                (_, KeyCode::Enter) => {
                    if let GigField::None = self.current_field {
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
            GigField::Artist => {
                if let Some(artist_value) = self.artist_input.handle_event(&event) {
                    match artist_value {
                        ListInputEvent::Escape => self.artist_input.unfocus(),
                        ListInputEvent::Select => {
                            self.change_focus(self.current_field.next());
                        }
                    }
                }
            }

            GigField::Venue => {
                if let Some(venue_input) = self.venue_input.handle_event(&event) {
                    match venue_input {
                        ListInputEvent::Escape => self.venue_input.unfocus(),
                        ListInputEvent::Select => {
                            self.change_focus(self.current_field.next());
                        }
                    }
                }
            }

            GigField::Date => {
                if let Some(date_input) = self.date_input.handle_event(&event) {
                    match date_input {
                        TextInputEvent::Escape => self.date_input.unfocus(),
                        TextInputEvent::Save => {
                            self.change_focus(self.current_field.next());
                        }
                    }
                }
            }

            GigField::Act => {
                if let Some(act_input) = self.act_input.handle_event(&event) {
                    match act_input {
                        ActInputEvent::Escape => self.act_input.unfocus(),
                        ActInputEvent::Select => {
                            self.change_focus(self.current_field.next());
                        }
                    }
                }
            }

            GigField::Save => {
                if let Some(save_input) = self.save_button.handle_event(&event) {
                    match save_input {
                        SaveButtonEvent::Escape => self.save_button.unfocus(),
                        SaveButtonEvent::Save => {
                            if let Some(field_error) = self.save_value().await? {
                                match field_error {
                                    GigFieldError::Artist(err) => self.artist_input.set_err(err),
                                    GigFieldError::Venue(err) => self.venue_input.set_err(err),
                                    GigFieldError::Date(err) => self.date_input.set_err(err),
                                    GigFieldError::Act(err) => self.act_input.set_err(err),
                                    GigFieldError::Save(err) => self.save_button.set_err(err),
                                }
                            }
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
            Constraint::Fill(1),
            Constraint::Length(3),
            Constraint::Length(3),
        ])
        .areas(area);

        let [top_left, top_right] = Layout::horizontal([Constraint::Fill(1); 2]).areas(top);
        let [mid_left, mid_right] =
            Layout::horizontal(vec![Constraint::Length(12), Constraint::Fill(1)]).areas(middle);

        self.artist_input.render(frame, top_left);
        self.venue_input.render(frame, top_right);
        self.date_input.render(frame, mid_left);
        self.act_input.render(frame, mid_right);
        self.save_button.render(frame, bottom);
    }

    async fn save_value(&self) -> Result<Option<GigFieldError>, Error> {
        let artist_id = match self.artist_input.get_value() {
            Some(artist) => artist.artist_id,
            None => {
                return Ok(Some(GigFieldError::Artist(
                    "Field \"Artist\" cannot be empty".into(),
                )))
            }
        };

        let venue_id = match self.venue_input.get_value() {
            Some(venue) => venue.venue_id,
            None => {
                return Ok(Some(GigFieldError::Venue(
                    "Field \"Venue\" cannot be empty".into(),
                )))
            }
        };

        let date = match self.date_input.get_value() {
            Some(date) => {
                if !Date::is_valid(&date) {
                    return Ok(Some(GigFieldError::Date("Invalid Date Format".into())));
                }

                Date::from(date)
            }
            None => {
                return Ok(Some(GigFieldError::Date(
                    "Field \"Date\" cannot be empty".into(),
                )))
            }
        };

        let act = match self.act_input.get_value() {
            Some(act) => act,
            None => {
                return Ok(Some(GigFieldError::Act(
                    "Field \"Act\" cannot be empty".into(),
                )))
            }
        };

        let gig = Gig::new(artist_id, venue_id, date, act);

        match Gig::save(gig, &self.pool).await {
            Ok(_) => Ok(None),
            Err(err) => Ok(Some(GigFieldError::Save(err.to_string()))),
        }
    }
}
