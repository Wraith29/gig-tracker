use crossterm::event::{Event, KeyCode, KeyModifiers};
use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::{Style, Stylize},
    widgets::{Block, BorderType},
    Frame,
};
use sqlx::{Pool, Sqlite};

use crate::{
    artist::Artist, city::City, datatable::DataTable, error::Error, gig::Gig, venue::Venue,
};

#[derive(Hash, PartialEq, Eq, Clone)]
pub enum TableName {
    Artist,
    Gig,
    Venue,
    City,
}

impl TableName {
    fn next(&self) -> Self {
        match self {
            TableName::Artist => TableName::Venue,
            TableName::Venue => TableName::Gig,
            TableName::Gig => TableName::City,
            TableName::City => TableName::City,
        }
    }

    fn prev(&self) -> Self {
        match self {
            TableName::Artist => TableName::Artist,
            TableName::Venue => TableName::Artist,
            TableName::Gig => TableName::Venue,
            TableName::City => TableName::Gig,
        }
    }
}

pub struct DataColumn<'a> {
    is_focused: bool,

    artist_table: DataTable<'a, Artist>,
    venue_table: DataTable<'a, Venue>,
    gig_table: DataTable<'a, Gig>,
    city_table: DataTable<'a, City>,

    pub focused_app: TableName,
}

impl DataColumn<'_> {
    pub async fn new(pool: &Pool<Sqlite>) -> Result<Self, Error> {
        let artist_table = DataTable::new(
            "Artist",
            pool.clone(),
            [Constraint::Length(20); 3].to_vec(),
            vec!["Artist Id", "Name", "City Name"],
        )
        .await?;

        let venue_table = DataTable::new(
            "Venue",
            pool.clone(),
            [Constraint::Length(20); 3].to_vec(),
            vec!["Venue Id", "Name", "City Name"],
        )
        .await?;

        let gig_table = DataTable::new(
            "Gig",
            pool.clone(),
            [Constraint::Length(15); 4].to_vec(),
            vec!["Artist", "Venue", "Date", "Act"],
        )
        .await?;

        let city_table = DataTable::new(
            "City",
            pool.clone(),
            [Constraint::Length(30); 2].to_vec(),
            vec!["City Id", "Name"],
        )
        .await?;

        Ok(Self {
            is_focused: true,
            artist_table,
            venue_table,
            gig_table,
            city_table,
            focused_app: TableName::Artist,
        })
    }

    pub async fn reload_data(&mut self) -> Result<(), Error> {
        self.artist_table.reload_data().await?;
        self.venue_table.reload_data().await?;
        self.gig_table.reload_data().await?;
        self.city_table.reload_data().await?;

        Ok(())
    }

    pub fn focus(&mut self, new_focus: TableName) {
        self.is_focused = true;

        match self.focused_app {
            TableName::Artist => self.artist_table.unfocus(),
            TableName::Gig => self.gig_table.unfocus(),
            TableName::Venue => self.venue_table.unfocus(),
            TableName::City => self.city_table.unfocus(),
        }

        self.focused_app = new_focus;

        match self.focused_app {
            TableName::Artist => self.artist_table.focus(),
            TableName::Gig => self.gig_table.focus(),
            TableName::Venue => self.venue_table.focus(),
            TableName::City => self.city_table.focus(),
        }
    }

    pub fn unfocus(&mut self) {
        self.is_focused = false;

        match self.focused_app {
            TableName::Artist => self.artist_table.focus(),
            TableName::Gig => self.gig_table.focus(),
            TableName::Venue => self.venue_table.focus(),
            TableName::City => self.city_table.focus(),
        }
    }

    pub fn render(&mut self, frame: &mut Frame, area: Rect) {
        let border_style = Style::new().blue();
        let mut block = Block::bordered()
            .border_type(BorderType::Thick)
            .border_style(border_style);

        if self.is_focused {
            block = block.border_style(border_style.yellow());
        }

        let content_area = block.inner(area);
        frame.render_widget(block, area);

        let [artist_area, venue_area, gig_area, city_area] =
            Layout::vertical([Constraint::Fill(1); 4]).areas(content_area);

        self.artist_table.render(frame, artist_area);
        self.venue_table.render(frame, venue_area);
        self.gig_table.render(frame, gig_area);
        self.city_table.render(frame, city_area);
    }

    pub fn handle_event(&mut self, event: Event) {
        if let Event::Key(key) = event {
            match (key.modifiers, key.code) {
                (KeyModifiers::CONTROL, KeyCode::Char('k')) => self.focus(self.focused_app.prev()),
                (KeyModifiers::CONTROL, KeyCode::Char('j')) => self.focus(self.focused_app.next()),
                _ => {}
            }
        }

        match self.focused_app {
            TableName::Artist => self.artist_table.handle_event(event),
            TableName::Venue => self.venue_table.handle_event(event),
            TableName::City => self.city_table.handle_event(event),
            TableName::Gig => self.gig_table.handle_event(event),
        }
    }
}
