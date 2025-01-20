use std::{io::Stdout, time::Duration};

use crossterm::event::{self, Event, KeyCode};
use ratatui::{
    layout::{Constraint, Layout},
    prelude::CrosstermBackend,
    Terminal,
};
use sqlx::SqlitePool;

use crate::{
    artist::{get_all_artists, Artist, ARTIST_HEADERS},
    data_table::DataTable,
    error::GTError,
    gig::{get_all_gigs, Gig, GIG_HEADERS},
    venue::{get_all_venues, Venue, VENUE_HEADERS},
};

enum FocusedApp {
    Artists,
    Venues,
    Gigs,
}

impl FocusedApp {
    fn down(&self) -> Self {
        match self {
            FocusedApp::Artists => FocusedApp::Venues,
            FocusedApp::Venues => FocusedApp::Gigs,
            FocusedApp::Gigs => FocusedApp::Artists,
        }
    }

    fn up(&self) -> Self {
        match self {
            FocusedApp::Artists => FocusedApp::Gigs,
            FocusedApp::Venues => FocusedApp::Artists,
            FocusedApp::Gigs => FocusedApp::Venues,
        }
    }
}

pub struct App<'a> {
    running: bool,
    pool: SqlitePool,
    term: Terminal<CrosstermBackend<Stdout>>,

    focused_app: FocusedApp,
    artists: DataTable<'a, Artist>,
    venues: DataTable<'a, Venue>,
    gigs: DataTable<'a, Gig>,
}

impl App<'_> {
    pub async fn new(db_url: &str) -> Result<App, GTError> {
        let pool = SqlitePool::connect(db_url).await?;
        let mut term = ratatui::init();

        term.clear()?;

        let artist_data = get_all_artists(&pool).await?;
        let mut artists = DataTable::new(
            "Artists",
            artist_data,
            [Constraint::Length(20); 3].to_vec(),
            ARTIST_HEADERS.to_vec(),
        );
        artists.focus();

        let venue_data = get_all_venues(&pool).await?;
        let venues = DataTable::new(
            "Venues",
            venue_data,
            [Constraint::Length(20); 3].to_vec(),
            VENUE_HEADERS.to_vec(),
        );

        let gig_data = get_all_gigs(&pool).await?;
        let gigs = DataTable::new(
            "Gigs",
            gig_data,
            [Constraint::Length(15); 4].to_vec(),
            GIG_HEADERS.to_vec(),
        );

        Ok(App {
            running: true,
            focused_app: FocusedApp::Artists,
            pool,
            term,
            artists,
            venues,
            gigs,
        })
    }

    pub async fn run(mut self) -> Result<(), GTError> {
        while self.running {
            self.draw()?;

            if self.handle_events()? {
                return Ok(());
            }
        }

        Ok(())
    }

    // True means quit
    fn handle_events(&mut self) -> Result<bool, GTError> {
        if !event::poll(Duration::from_secs(0))? {
            return Ok(false);
        }

        let event = event::read()?;

        match event {
            Event::Key(key) => match key.code {
                KeyCode::Char('q') => return Ok(true),
                KeyCode::Char('J') => self.focus(self.focused_app.down()),
                KeyCode::Char('K') => self.focus(self.focused_app.up()),
                _ => {}
            },
            _ => {}
        }

        match self.focused_app {
            FocusedApp::Artists => self.artists.handle_event(&event),
            FocusedApp::Venues => self.venues.handle_event(&event),
            FocusedApp::Gigs => self.gigs.handle_event(&event),
        }

        Ok(false)
    }

    fn focus(&mut self, new_focus: FocusedApp) {
        self.artists.unfocus();
        self.venues.unfocus();
        self.gigs.unfocus();

        match new_focus {
            FocusedApp::Artists => self.artists.focus(),
            FocusedApp::Venues => self.venues.focus(),
            FocusedApp::Gigs => self.gigs.focus(),
        }

        self.focused_app = new_focus;
    }

    fn draw(&mut self) -> Result<(), GTError> {
        self.term.draw(|f| {
            let [top, middle, bottom] = Layout::vertical([Constraint::Fill(1); 3]).areas(f.area());

            self.artists.render(f, top);
            self.venues.render(f, middle);
            self.gigs.render(f, bottom);
        })?;

        Ok(())
    }
}
