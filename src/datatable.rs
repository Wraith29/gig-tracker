use crossterm::event::{Event, KeyCode};
use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::{Style, Stylize},
    text::Text,
    widgets::{Block, BorderType, Row, Table, TableState},
    Frame,
};
use sqlx::SqlitePool;

use crate::{dataset::DataSet, error::GTError};

const HELP_TEXT: &str = "j - down, k - up, + - create, <- back";

pub struct DataTable<'a> {
    title: String,
    focused: bool,
    style: Style,
    table: Table<'a>,
    state: TableState,
}

impl<'a> DataTable<'a> {
    pub async fn new<T: Into<Row<'a>> + DataSet>(
        title: impl Into<String>,
        pool: &SqlitePool,
        constraints: Vec<Constraint>,
        headers: Vec<&'static str>,
    ) -> Result<Self, GTError> {
        let data = T::load_all(pool).await?;

        let table = Table::new(data, constraints)
            .header(Row::new(headers).underlined())
            .row_highlight_style(Style::new().on_gray().dark_gray());

        Ok(Self {
            table,
            focused: false,
            title: title.into(),
            style: Style::new().red(),
            state: TableState::default(),
        })
    }

    pub fn handle_event(&mut self, event: &Event) {
        match event {
            Event::Key(key) => match key.code {
                KeyCode::Char('k') => self.state.select_previous(),
                KeyCode::Char('j') => self.state.select_next(),
                _ => {}
            },
            _ => {}
        }
    }

    pub fn focus(&mut self) {
        self.focused = true;
        self.style = self.style.green();
        self.table = self.table.clone().slow_blink();
    }

    pub fn unfocus(&mut self) {
        self.focused = false;
        self.style = self.style.red();
    }

    pub fn render(&mut self, frame: &mut Frame, area: Rect) {
        let block = Block::bordered()
            .title(self.title.clone())
            .border_type(BorderType::Rounded)
            .border_style(self.style);

        let [content_area, help_area] =
            Layout::vertical(vec![Constraint::Fill(1), Constraint::Length(1)])
                .areas(block.inner(area));

        frame.render_widget(block, area);
        frame.render_stateful_widget(&self.table, content_area, &mut self.state);
        frame.render_widget(
            Text::from(HELP_TEXT).style(Style::new().on_dark_gray().white()),
            help_area,
        );
    }
}
