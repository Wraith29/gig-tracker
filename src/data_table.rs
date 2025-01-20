use std::borrow::BorrowMut;

use crossterm::event::{Event, KeyCode};
use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::{Color, Style, Styled, Stylize},
    text::Text,
    widgets::{Block, BorderType, Borders, Row, Table, TableState, Widget},
    Frame,
};

pub struct DataTable<'a, T: Into<Row<'a>> + Clone> {
    title: String,
    data: Vec<T>,
    pub focused: bool,
    style: Style,
    table: Table<'a>,
    state: TableState,
}

impl<'a, T: Into<Row<'a>> + Clone> DataTable<'a, T> {
    pub fn new(
        title: impl Into<String>,
        data: Vec<T>,
        constraints: Vec<Constraint>,
        headers: Vec<&'static str>,
    ) -> Self {
        let table = Table::new(data.clone(), constraints)
            .header(Row::new(headers).underlined())
            .row_highlight_style(Style::new().on_gray().dark_gray());

        Self {
            data,
            table,
            focused: false,
            title: title.into(),
            style: Style::new().red(),
            state: TableState::default(),
        }
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

        let [table_area, help_line] =
            Layout::vertical(vec![Constraint::Fill(1), Constraint::Length(1)])
                .areas(block.inner(area));

        frame.render_widget(block, area);
        frame.render_stateful_widget(&self.table, table_area, &mut self.state);
        frame.render_widget(
            Text::from("k - up, j - down, + - new").style(Style::new().on_dark_gray().white()),
            help_line,
        );
    }
}
