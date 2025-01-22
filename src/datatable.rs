use crossterm::event::{Event, KeyCode};
use ratatui::{
    layout::{Constraint, Rect},
    style::{Style, Stylize},
    widgets::{Block, BorderType, Row, Table, TableState},
    Frame,
};
use sqlx::{Pool, Sqlite};

use crate::{dataset::DataSet, error::Error};

pub struct DataTable<'d> {
    name: &'static str,
    is_focused: bool,
    border_style: Style,
    table: Table<'d>,
    state: TableState,
}

impl<'d> DataTable<'d> {
    pub async fn new<T: Into<Row<'d>> + DataSet + Clone>(
        name: &'static str,
        conn: &Pool<Sqlite>,
        constraints: Vec<Constraint>,
        headers: Vec<&'static str>,
    ) -> Result<Self, Error> {
        let data = T::load_all(conn).await?;

        let table = Table::new(data, constraints)
            .header(Row::new(headers))
            .style(Style::new().white())
            .row_highlight_style(Style::new().dark_gray().on_gray());

        Ok(Self {
            name,
            is_focused: false,
            border_style: Style::new().red(),
            table,
            state: TableState::default(),
        })
    }

    pub fn focus(&mut self) {
        self.is_focused = true;
        self.border_style = self.border_style.green();
    }

    pub fn unfocus(&mut self) {
        self.is_focused = false;
        self.border_style = self.border_style.red();
    }

    pub fn render(&mut self, frame: &mut Frame, area: Rect) {
        let block = Block::bordered()
            .border_type(BorderType::Rounded)
            .title(self.name)
            .style(self.border_style);

        let content_area = block.inner(area);

        frame.render_widget(block, area);
        frame.render_stateful_widget(&self.table, content_area, &mut self.state);
    }

    pub fn handle_event(&mut self, event: Event) {
        match event {
            Event::Key(key) => match key.code {
                KeyCode::Char('k') => self.state.select_previous(),
                KeyCode::Char('j') => self.state.select_next(),
                _ => {}
            },
            _ => {}
        }
    }
}
