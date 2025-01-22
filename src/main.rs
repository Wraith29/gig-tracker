mod act;
mod artist;
mod city;
mod columns;
mod dataset;
mod datatable;
mod date;
mod error;
mod gig;
mod venue;

use columns::{data::DataColumn, graph::GraphColumn, ColumnName};
use crossterm::event::{self, Event, KeyCode};
use dataset::DataSet;
use dotenv::dotenv;
use error::Error;
use gig::Gig;
use ratatui::{
    layout::{Constraint, Layout},
    prelude::CrosstermBackend,
    Terminal,
};
use sqlx::Pool;
use std::{env, io::Stdout, time::Duration};

struct App<'a> {
    terminal: Terminal<CrosstermBackend<Stdout>>,
    focused_column: ColumnName,
    data_column: DataColumn<'a>,
    graph_column: GraphColumn,
}

impl<'a> App<'a> {
    async fn new(db_url: &'a str) -> Result<Self, Error> {
        let pool = Pool::connect(db_url).await?;

        let mut terminal = ratatui::init();
        terminal.clear()?;

        let mut data_column = DataColumn::new(&pool).await?;
        data_column.focus();

        let gigs = Gig::load_all(&pool).await?;
        let graph_column = GraphColumn::new(gigs);

        Ok(Self {
            terminal,
            data_column,
            graph_column,
            focused_column: ColumnName::Data,
        })
    }

    async fn run(&mut self) -> Result<(), Error> {
        loop {
            if self.handle_events()? {
                break;
            }

            self.render()?;
        }

        Ok(())
    }

    fn handle_events(&mut self) -> Result<bool, Error> {
        if !event::poll(Duration::from_secs(0))? {
            return Ok(false);
        }

        let event = event::read()?;

        if let Event::Key(key) = event {
            match key.code {
                KeyCode::Char('q') => return Ok(true),
                KeyCode::Char('H') => {
                    if matches!(self.focused_column, ColumnName::Graph) {
                        self.focused_column = ColumnName::Data;
                        self.graph_column.unfocus();
                        self.data_column.focus();
                    }
                }
                KeyCode::Char('L') => {
                    if matches!(self.focused_column, ColumnName::Data) {
                        self.focused_column = ColumnName::Graph;
                        self.data_column.unfocus();
                        self.graph_column.focus();
                    }
                }
                _ => {}
            }
        }

        match self.focused_column {
            ColumnName::Data => self.data_column.handle_event(event),
            _ => {}
        }

        Ok(false)
    }

    fn render(&mut self) -> Result<(), Error> {
        self.terminal.draw(|frame| {
            let [left, right] = Layout::horizontal([Constraint::Fill(1); 2]).areas(frame.area());

            self.data_column.render(frame, left);
            self.graph_column.render(frame, right);
        })?;

        Ok(())
    }
}

#[async_std::main]
async fn main() -> Result<(), Error> {
    dotenv().ok();

    let db_url = env::var("DATABASE_URL").expect("env var DATABASE_URL should be set");

    let mut app = App::new(db_url.as_str()).await?;

    let result = app.run().await;

    ratatui::restore();

    result
}
