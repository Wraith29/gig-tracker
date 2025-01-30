use std::collections::HashMap;

use crossterm::event::{Event, KeyCode};
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

#[derive(Hash, PartialEq, Eq)]
enum TableName {
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
    apps: HashMap<TableName, DataTable<'a>>,
    focused_app: TableName,
}

impl DataColumn<'_> {
    pub async fn new(pool: &Pool<Sqlite>) -> Result<Self, Error> {
        let artist_table = DataTable::new::<Artist>(
            "Artist",
            pool.clone(),
            [Constraint::Length(20); 3].to_vec(),
            vec!["Artist Id", "Name", "City Name"],
        )
        .await?;

        let venue_table = DataTable::new::<Venue>(
            "Venue",
            pool.clone(),
            [Constraint::Length(20); 3].to_vec(),
            vec!["Venue Id", "Name", "City Name"],
        )
        .await?;

        let gig_table = DataTable::new::<Gig>(
            "Gig",
            pool.clone(),
            [Constraint::Length(15); 4].to_vec(),
            vec!["Artist", "Venue", "Date", "Act"],
        )
        .await?;

        let city_table = DataTable::new::<City>(
            "City",
            pool.clone(),
            [Constraint::Length(30); 2].to_vec(),
            vec!["City Id", "Name"],
        )
        .await?;

        Ok(Self {
            is_focused: true,
            apps: HashMap::from([
                (TableName::Artist, artist_table),
                (TableName::Venue, venue_table),
                (TableName::Gig, gig_table),
                (TableName::City, city_table),
            ]),
            focused_app: TableName::Artist,
        })
    }

    pub async fn reload_data(&mut self) -> Result<(), Error> {
        self.apps
            .get_mut(&TableName::Artist)
            .expect("Artist table should be set")
            .reload_data::<Artist>()
            .await?;

        self.apps
            .get_mut(&TableName::Venue)
            .expect("Venue table should be set")
            .reload_data::<Venue>()
            .await?;

        self.apps
            .get_mut(&TableName::Gig)
            .expect("Gig table should be set")
            .reload_data::<Gig>()
            .await?;

        self.apps
            .get_mut(&TableName::City)
            .expect("City table should be set")
            .reload_data::<City>()
            .await?;

        Ok(())
    }

    pub fn focus(&mut self) {
        self.is_focused = true;

        self.apps
            .get_mut(&self.focused_app)
            .expect("focused app should be available")
            .focus();
    }

    pub fn unfocus(&mut self) {
        self.is_focused = false;

        self.apps.iter_mut().for_each(|(_, app)| app.unfocus());
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

        self.apps
            .get_mut(&TableName::Artist)
            .expect("Artist Table should be set")
            .render(frame, artist_area);

        self.apps
            .get_mut(&TableName::Venue)
            .expect("Venue Table should be set")
            .render(frame, venue_area);

        self.apps
            .get_mut(&TableName::Gig)
            .expect("Gig Table should be set")
            .render(frame, gig_area);

        self.apps
            .get_mut(&TableName::City)
            .expect("Gig Table should be set")
            .render(frame, city_area);
    }

    pub fn handle_event(&mut self, event: Event) {
        if let Event::Key(key) = event {
            match key.code {
                KeyCode::Char('K') => {
                    self.apps.get_mut(&self.focused_app).unwrap().unfocus();
                    self.focused_app = self.focused_app.prev();
                    self.apps.get_mut(&self.focused_app).unwrap().focus();
                }
                KeyCode::Char('J') => {
                    self.apps.get_mut(&self.focused_app).unwrap().unfocus();
                    self.focused_app = self.focused_app.next();
                    self.apps.get_mut(&self.focused_app).unwrap().focus();
                }
                _ => {}
            }
        }

        self.apps
            .get_mut(&self.focused_app)
            .unwrap()
            .handle_event(event);
    }
}
