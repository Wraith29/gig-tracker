use crate::{dataset::DataSet, error::Error};
use crossterm::event::{Event, KeyCode};
use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::{Style, Stylize},
    widgets::{Block, BorderType, List, ListItem, ListState, Paragraph},
    Frame,
};
use sqlx::{Pool, Sqlite};

pub enum ListInputEvent {
    Select,
    Escape,
}

pub struct ListInput<'a, T> {
    title: &'a str,
    error: Option<String>,
    searching: bool,
    search_text: String,

    focused: bool,
    selected: Option<T>,
    selected_idx: usize,

    values: Vec<T>,
    filtered_values: Vec<T>,
    list: List<'a>,
    state: ListState,
}

impl<'a, T: DataSet + Into<ListItem<'a>> + Ord> ListInput<'a, T> {
    pub async fn new(title: &'a str, pool: &Pool<Sqlite>) -> Result<Self, Error> {
        let mut values = T::load_all(pool).await?;
        values.sort_unstable_by(|l, r| l.key().cmp(&r.key()));
        let filtered_values = values.clone();

        let list = List::new(values.clone()).highlight_style(Style::new().on_gray());

        Ok(Self {
            title,
            error: None,
            searching: false,
            search_text: String::new(),
            focused: false,
            selected: None,
            selected_idx: 0,
            values,
            filtered_values,
            list,
            state: ListState::default(),
        })
    }

    pub fn handle_event(&mut self, event: &Event) -> Option<ListInputEvent> {
        if let Event::Key(key) = event {
            match key.code {
                KeyCode::Esc => {
                    if self.searching {
                        self.searching = false;
                        self.search_text = String::new();
                        self.update_filter();

                        return None;
                    }

                    return Some(ListInputEvent::Escape);
                }
                KeyCode::Enter => {
                    if self.searching {
                        self.searching = false;
                        self.update_filter();

                        return None;
                    }

                    if let Some(idx) = self.state.selected() {
                        if idx < self.filtered_values.len() {
                            self.selected = Some(self.filtered_values[idx].clone());
                            self.selected_idx = idx;
                        }
                    }

                    return Some(ListInputEvent::Select);
                }

                KeyCode::Char('/') => {
                    self.searching = true;
                    self.update_filter();
                }

                KeyCode::Char('j') => {
                    if self.searching {
                        self.search_text.push('j');
                        self.update_filter();

                        return None;
                    }

                    self.state.select_next();
                    if let Some(idx) = self.state.selected() {
                        if idx < self.filtered_values.len() {
                            self.selected = Some(self.filtered_values[idx].clone());
                            self.selected_idx = idx;
                        }
                    }
                }

                KeyCode::Char('k') => {
                    if self.searching {
                        self.search_text.push('k');
                        self.update_filter();

                        return None;
                    }

                    self.state.select_previous();
                    if let Some(idx) = self.state.selected() {
                        if idx < self.filtered_values.len() {
                            self.selected = Some(self.filtered_values[idx].clone());
                            self.selected_idx = idx;
                        }
                    }
                }

                KeyCode::Char(char) => {
                    if self.searching {
                        self.search_text.push(char);
                        self.update_filter();
                    }
                }

                KeyCode::Backspace => {
                    if self.searching {
                        self.search_text.pop();
                        self.update_filter();
                    }
                }

                _ => {}
            }
        }

        None
    }

    pub fn render(&mut self, frame: &mut Frame, area: Rect) {
        let mut block = Block::bordered().title_top(self.title);
        let content_area = block.inner(area);

        if let Some(err) = self.error.clone() {
            block = block.border_style(Style::new().red()).title_bottom(err);
        }

        if self.focused && self.searching {
            let [search_bar, list_area] =
                Layout::vertical([Constraint::Length(3), Constraint::Fill(1)]).areas(content_area);

            frame.render_widget(block.clone().border_type(BorderType::Double), area);
            frame.render_widget(
                Paragraph::new(self.search_text.as_str()).block(Block::bordered().title("Search")),
                search_bar,
            );

            frame.render_stateful_widget(&self.list, list_area, &mut self.state);
        } else if self.focused {
            frame.render_widget(block.clone().border_type(BorderType::Double), area);
            frame.render_stateful_widget(&self.list, content_area, &mut self.state);
        } else if let Some(selected) = &self.selected {
            frame.render_widget(block, area);
            frame.render_widget(selected.to_string(), content_area);
        } else {
            frame.render_widget(block, area);
            frame.render_widget(&self.list, content_area);
        }
    }

    pub fn get_value(&self) -> Option<T> {
        self.selected.clone()
    }

    pub fn set_err(&mut self, err: String) {
        self.error = Some(err);
    }

    pub fn focus(&mut self) {
        if self.selected.is_none() {
            self.state.select(Some(0));
        } else {
            self.state.select(Some(self.selected_idx));
        }

        self.focused = true;
    }

    pub fn unfocus(&mut self) {
        self.focused = false;
    }

    fn update_filter(&mut self) {
        self.filtered_values = self
            .values
            .iter()
            .filter(|row| row.contains(self.search_text.clone()))
            .cloned()
            .collect();

        self.list = self.list.clone().items(self.filtered_values.clone());
    }
}
