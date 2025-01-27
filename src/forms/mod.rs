use crate::error::Error;
use artist::ArtistForm;
use crossterm::event::{Event, KeyCode};
use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::Stylize,
    widgets::{Block, Borders, Clear, Tabs},
    Frame,
};
use sqlx::{Pool, Sqlite};

mod artist;
mod listinput;
mod savebutton;
mod textinput;

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
    current_tab: FormTabs,
    tabs: Tabs<'a>,

    artist_form: ArtistForm<'a>,
}

impl<'a> Form<'a> {
    pub async fn new(pool: Pool<Sqlite>) -> Result<Self, Error> {
        let tabs = Tabs::new(FORM_TABS);

        let artist_form = ArtistForm::new(pool.clone()).await?;

        Ok(Self {
            tabs,
            current_tab: FormTabs::Artist,
            artist_form,
        })
    }

    pub async fn handle_event(&mut self, event: Event) -> Result<(), Error> {
        if let Event::Key(key) = event {
            match key.code {
                KeyCode::Right => {
                    self.current_tab = self.current_tab.next();
                    self.tabs = self.tabs.clone().select(&self.current_tab);
                }
                KeyCode::Left => {
                    self.current_tab = self.current_tab.prev();
                    self.tabs = self.tabs.clone().select(&self.current_tab);
                }
                _ => {}
            }
        }

        match self.current_tab {
            FormTabs::Artist => self.artist_form.handle_event(event).await?,
            FormTabs::Venue => todo!(),
            FormTabs::Gig => todo!(),
            FormTabs::City => todo!(),
        }

        Ok(())
    }

    pub fn render(&mut self, frame: &mut Frame, area: Rect) {
        let [_, mid_col, _] = Layout::horizontal([Constraint::Fill(1); 3]).areas(area);
        let [_, mid_area, _] = Layout::vertical([Constraint::Fill(1); 3]).areas(mid_col);

        let block = Block::bordered().white().title("Create New");

        let [tabs_area, content_area] =
            Layout::vertical([Constraint::Length(2), Constraint::Fill(1)])
                .areas(block.inner(mid_area));

        frame.render_widget(Clear {}, mid_area);

        frame.render_widget(block, mid_area);

        frame.render_widget(
            &self
                .tabs
                .clone()
                .block(Block::new().borders(Borders::BOTTOM)),
            tabs_area,
        );

        self.artist_form.render(frame, content_area);
    }
}
