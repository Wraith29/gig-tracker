use crate::{dataset::DataSet, error::Error};
use crossterm::event::{Event, KeyCode};
use ratatui::{
    layout::Rect,
    style::{Style, Stylize},
    widgets::{Block, BorderType, List, ListItem, ListState},
    Frame,
};
use sqlx::{Pool, Sqlite};

pub enum ListInputEvent {
    Select,
    Escape,
}

pub struct ListInput<'a, T> {
    title: &'a str,
    focused: bool,
    error: Option<String>,

    values: Vec<T>,
    selected: Option<T>,
    selected_idx: usize,
    list: List<'a>,
    state: ListState,
}

impl<'a, T: DataSet + Into<ListItem<'a>>> ListInput<'a, T> {
    pub async fn new(title: &'a str, pool: &Pool<Sqlite>) -> Result<Self, Error> {
        let values = T::load_all(pool).await?;
        let list = List::new(values.clone()).highlight_style(Style::new().on_gray());

        Ok(Self {
            title,
            list,
            error: None,
            focused: false,
            values,
            selected: None,
            selected_idx: 0,
            state: ListState::default(),
        })
    }

    pub fn focus(&mut self) {
        if let None = self.selected {
            self.state.select(Some(0));
        } else {
            self.state.select(Some(self.selected_idx));
        }

        self.focused = true;
    }

    pub fn unfocus(&mut self) {
        self.focused = false;
    }

    pub fn set_err(&mut self, err: String) {
        self.error = Some(err);
    }

    pub fn get_value(&self) -> Option<T> {
        self.selected.clone()
    }

    pub fn handle_event(&mut self, event: &Event) -> Option<ListInputEvent> {
        if let Event::Key(key) = event {
            match key.code {
                KeyCode::Esc => return Some(ListInputEvent::Escape),
                KeyCode::Enter => {
                    if let Some(idx) = self.state.selected() {
                        if idx < self.values.len() {
                            self.selected = Some(self.values[idx].clone());
                        }
                    }

                    return Some(ListInputEvent::Select);
                }
                KeyCode::Char('j') => {
                    self.state.select_next();
                    if let Some(idx) = self.state.selected() {
                        if idx < self.values.len() {
                            self.selected = Some(self.values[idx].clone());
                            self.selected_idx = idx;
                        }
                    }
                }

                KeyCode::Char('k') => {
                    self.state.select_previous();
                    if let Some(idx) = self.state.selected() {
                        if idx < self.values.len() {
                            self.selected = Some(self.values[idx].clone());
                            self.selected_idx = idx;
                        }
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

        if self.focused {
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
}
