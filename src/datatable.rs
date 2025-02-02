use crossterm::event::{Event, KeyCode, KeyModifiers};
use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::{Style, Stylize},
    widgets::{Block, BorderType, Paragraph, Row, Table, TableState},
    Frame,
};
use sqlx::{Pool, Sqlite};

use crate::{dataset::DataSet, error::Error};

pub struct DataTable<'d, T: Into<Row<'d>> + DataSet + Clone + Ord> {
    name: &'static str,
    searching: bool,
    search_text: String,

    data: Vec<T>,
    filtered_data: Vec<T>,

    pool: Pool<Sqlite>,
    is_focused: bool,
    border_style: Style,
    table: Table<'d>,
    state: TableState,
}

impl<'d, T: Into<Row<'d>> + DataSet + Clone + Ord> DataTable<'d, T> {
    pub async fn new(
        name: &'static str,
        pool: Pool<Sqlite>,
        constraints: Vec<Constraint>,
        headers: Vec<&'static str>,
    ) -> Result<Self, Error> {
        let mut data = T::load_all(&pool).await?;
        data.sort_unstable_by(|l, r| l.key().cmp(&r.key()));

        let filtered_data = data.clone();

        let table = Table::new(data.clone(), constraints)
            .header(Row::new(headers))
            .style(Style::new().white())
            .row_highlight_style(Style::new().dark_gray().on_gray());

        Ok(Self {
            name,
            pool,
            data,
            filtered_data,
            searching: false,
            search_text: String::new(),
            is_focused: false,
            border_style: Style::new().red(),
            table,
            state: TableState::default(),
        })
    }

    pub async fn reload_data(&mut self) -> Result<(), Error> {
        let mut data = T::load_all(&self.pool).await?;
        data.sort_unstable_by(|l, r| l.key().cmp(&r.key()));

        self.data = data.clone();
        self.filtered_data = data
            .iter()
            .filter(|row| row.contains(self.search_text.clone()))
            .cloned()
            .collect();

        let rows: Vec<Row<'d>> = data.iter().map(|row| row.clone().into()).collect();

        self.table = self.table.clone().rows(rows);

        Ok(())
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

        if self.searching {
            let [search_bar, list_view] =
                Layout::vertical(vec![Constraint::Length(3), Constraint::Fill(1)])
                    .areas(content_area);

            frame.render_widget(
                Paragraph::new(self.search_text.clone()).block(Block::bordered().title("Search")),
                search_bar,
            );
            frame.render_stateful_widget(&self.table, list_view, &mut self.state);
        } else {
            frame.render_stateful_widget(&self.table, content_area, &mut self.state);
        }
    }

    pub fn handle_event(&mut self, event: Event) {
        if let Event::Key(key) = event {
            match (key.modifiers, key.code) {
                (KeyModifiers::NONE, KeyCode::Backspace) => {
                    self.search_text.pop();
                    self.update_filter();
                }
                (KeyModifiers::NONE, KeyCode::Esc) => {
                    self.searching = false;
                    self.search_text = String::new();
                    self.update_filter();
                }

                (KeyModifiers::NONE, KeyCode::Enter) => {
                    self.searching = false;
                }

                (KeyModifiers::NONE, KeyCode::Char('/')) => {
                    self.searching = true;
                }

                (KeyModifiers::NONE, KeyCode::Char('k')) => {
                    if self.searching {
                        self.search_text.push('k');
                        self.update_filter();
                        return;
                    }

                    self.state.select_previous();
                }

                (KeyModifiers::NONE, KeyCode::Char('j')) => {
                    if self.searching {
                        self.search_text.push('j');
                        self.update_filter();
                        return;
                    }

                    self.state.select_next();
                }

                (_, KeyCode::Char(char)) => {
                    if self.searching {
                        self.search_text.push(char);
                        self.update_filter();
                    }
                }

                _ => {}
            }
        }
    }

    fn update_filter(&mut self) {
        self.filtered_data = self
            .data
            .iter()
            .filter(|row| row.contains(self.search_text.clone()))
            .cloned()
            .collect();

        let rows: Vec<Row<'d>> = self
            .filtered_data
            .iter()
            .map(|row| row.clone().into())
            .collect();

        self.table = self.table.clone().rows(rows);
    }
}
