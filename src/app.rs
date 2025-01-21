use std::{collections::HashMap, io::Stdout, time::Duration};

use crossterm::event::{self, Event, KeyCode};
use ratatui::{
    layout::{Constraint, Layout},
    prelude::CrosstermBackend,
    Terminal,
};
use sqlx::SqlitePool;

use crate::{
    artist::{Artist, ARTIST_HEADERS},
    column::{Column, FocusedApp},
    datatable::DataTable,
    error::GTError,
    form::Form,
    gig::{Gig, GIG_HEADERS},
    venue::{Venue, VENUE_HEADERS},
};

pub struct App<'a> {
    running: bool,
    term: Terminal<CrosstermBackend<Stdout>>,
    apps: HashMap<FocusedApp, DataTable<'a>>,
    column: Column,
    form: Form<'a>,
}

impl App<'_> {
    pub async fn new(db_url: &str) -> Result<App, GTError> {
        let pool = SqlitePool::connect(db_url).await?;
        let mut term = ratatui::init();

        term.clear()?;

        let mut artists = DataTable::new::<Artist>(
            "Artists",
            &pool,
            [Constraint::Length(20); 3].to_vec(),
            ARTIST_HEADERS.to_vec(),
        )
        .await?;
        artists.focus();

        let venues = DataTable::new::<Venue>(
            "Venues",
            &pool,
            [Constraint::Length(20); 3].to_vec(),
            VENUE_HEADERS.to_vec(),
        )
        .await?;

        let gigs = DataTable::new::<Gig>(
            "Gigs",
            &pool,
            [Constraint::Length(15); 4].to_vec(),
            GIG_HEADERS.to_vec(),
        )
        .await?;

        Ok(App {
            running: true,
            term,
            apps: HashMap::from([
                (FocusedApp::Artists, artists),
                (FocusedApp::Venues, venues),
                (FocusedApp::Gigs, gigs),
            ]),
            column: Column::Left(FocusedApp::Artists),
            form: Form::new(),
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
                KeyCode::Char('J') => {
                    if let Column::Left(focused_app) = &self.column {
                        self.apps.get_mut(focused_app).unwrap().unfocus();
                    }

                    self.column = self.column.down();

                    if let Column::Left(focused_app) = &self.column {
                        self.apps.get_mut(focused_app).unwrap().focus();
                    }
                }
                KeyCode::Char('K') => {
                    if let Column::Left(focused_app) = &self.column {
                        self.apps.get_mut(focused_app).unwrap().unfocus();
                    }

                    self.column = self.column.up();

                    if let Column::Left(focused_app) = &self.column {
                        self.apps.get_mut(focused_app).unwrap().focus();
                    }
                }
                KeyCode::Char('H') => {
                    self.column = self.column.left();

                    if let Column::Left(focused_app) = &self.column {
                        self.apps.get_mut(focused_app).unwrap().focus();
                    }
                }
                KeyCode::Char('L') => {
                    self.column = self.column.right();

                    if !matches!(self.column, Column::Left(_)) {
                        self.apps.iter_mut().for_each(|app| app.1.unfocus());
                    }
                }
                _ => {}
            },
            _ => {}
        }

        match &self.column {
            Column::Left(focused_app) => {
                self.apps
                    .get_mut(&focused_app)
                    .unwrap()
                    .handle_event(&event);
            }
            _ => {}
        }

        Ok(false)
    }

    fn draw(&mut self) -> Result<(), GTError> {
        self.term.draw(|f| {
            let [left, middle, right] =
                Layout::horizontal([Constraint::Fill(1); 3]).areas(f.area());

            let [left_top, left_mid, left_btm] =
                Layout::vertical([Constraint::Fill(1); 3]).areas(left);

            let [mid_top, mid_btm] = Layout::vertical([Constraint::Fill(1); 2]).areas(middle);

            self.apps
                .get_mut(&FocusedApp::Artists)
                .unwrap()
                .render(f, left_top);
            self.apps
                .get_mut(&FocusedApp::Venues)
                .unwrap()
                .render(f, left_mid);
            self.apps
                .get_mut(&FocusedApp::Gigs)
                .unwrap()
                .render(f, left_btm);

            self.form.render(f, mid_top);
        })?;

        Ok(())
    }
}
