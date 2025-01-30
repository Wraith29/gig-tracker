use crate::error::Error;
use artist::ArtistForm;
use city::CityForm;
use crossterm::event::{Event, KeyCode, KeyModifiers};
use gig::GigForm;
use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::Stylize,
    widgets::{Block, Borders, Clear, Tabs},
    Frame,
};
use sqlx::{Pool, Sqlite};
use venue::VenueForm;

mod actinput;
mod artist;
mod avfield;
mod city;
mod gig;
mod listinput;
mod savebutton;
mod textinput;
mod venue;

const FORM_TABS: [&str; 4] = ["Artist", "Venue", "Gig", "City"];

enum FormTabs {
    Artist = 0,
    Venue,
    Gig,
    City,
}

impl FormTabs {
    fn next(&self) -> Self {
        match self {
            FormTabs::Artist => FormTabs::Venue,
            FormTabs::Venue => FormTabs::Gig,
            FormTabs::Gig => FormTabs::City,
            FormTabs::City => FormTabs::City,
        }
    }

    fn prev(&self) -> Self {
        match self {
            FormTabs::Artist => FormTabs::Artist,
            FormTabs::Venue => FormTabs::Artist,
            FormTabs::Gig => FormTabs::Venue,
            FormTabs::City => FormTabs::Gig,
        }
    }
}

impl From<&FormTabs> for Option<usize> {
    fn from(value: &FormTabs) -> Self {
        Some(match value {
            FormTabs::Artist => 0,
            FormTabs::Venue => 1,
            FormTabs::Gig => 2,
            FormTabs::City => 3,
        })
    }
}

pub struct Form<'a> {
    pool: Pool<Sqlite>,
    current_tab: FormTabs,
    tabs: Tabs<'a>,

    artist_form: ArtistForm<'a>,
    venue_form: VenueForm<'a>,
    gig_form: GigForm<'a>,
    city_form: CityForm<'a>,
}

impl Form<'_> {
    pub async fn new(pool: Pool<Sqlite>) -> Result<Self, Error> {
        let tabs = Tabs::new(FORM_TABS);

        let artist_form = ArtistForm::new(pool.clone()).await?;
        let venue_form = VenueForm::new(pool.clone()).await?;
        let gig_form = GigForm::new(pool.clone()).await?;
        let city_form = CityForm::new(pool.clone());

        Ok(Self {
            pool,
            tabs,
            current_tab: FormTabs::Artist,
            artist_form,
            venue_form,
            gig_form,
            city_form,
        })
    }

    pub async fn reset_and_reload(&mut self) -> Result<(), Error> {
        self.artist_form = ArtistForm::new(self.pool.clone()).await?;
        self.venue_form = VenueForm::new(self.pool.clone()).await?;
        self.gig_form = GigForm::new(self.pool.clone()).await?;
        self.city_form = CityForm::new(self.pool.clone());

        Ok(())
    }

    pub async fn handle_event(&mut self, event: Event) -> Result<bool, Error> {
        if let Event::Key(key) = event {
            match (key.modifiers, key.code) {
                (KeyModifiers::CONTROL, KeyCode::Char('l')) => {
                    self.current_tab = self.current_tab.next();
                    self.tabs = self.tabs.clone().select(&self.current_tab);
                }
                (KeyModifiers::CONTROL, KeyCode::Char('h')) => {
                    self.current_tab = self.current_tab.prev();
                    self.tabs = self.tabs.clone().select(&self.current_tab);
                }
                _ => {}
            }
        }

        Ok(match self.current_tab {
            FormTabs::Artist => self.artist_form.handle_event(event).await?,
            FormTabs::Venue => self.venue_form.handle_event(event).await?,
            FormTabs::Gig => self.gig_form.handle_event(event).await?,
            FormTabs::City => self.city_form.handle_event(event).await?,
        })
    }

    pub fn render(&mut self, frame: &mut Frame, area: Rect) {
        let [_, mid_col, _] = Layout::horizontal(vec![
            Constraint::Fill(1),
            Constraint::Percentage(50),
            Constraint::Fill(1),
        ])
        .areas(area);
        let [_, mid_area, _] = Layout::vertical(vec![
            Constraint::Fill(1),
            Constraint::Percentage(50),
            Constraint::Fill(1),
        ])
        .areas(mid_col);

        let block = Block::bordered().white().title("Create New");

        let [tabs_area, content_area] =
            Layout::vertical([Constraint::Length(2), Constraint::Fill(1)])
                .areas(block.inner(mid_area));

        frame.render_widget(Clear {}, mid_area);

        frame.render_widget(block, mid_area);

        frame.render_widget(
            self.tabs
                .clone()
                .block(Block::new().borders(Borders::BOTTOM)),
            tabs_area,
        );

        match self.current_tab {
            FormTabs::Artist => self.artist_form.render(frame, content_area),
            FormTabs::Venue => self.venue_form.render(frame, content_area),
            FormTabs::Gig => self.gig_form.render(frame, content_area),
            FormTabs::City => self.city_form.render(frame, content_area),
        }
    }
}
