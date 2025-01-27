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
    values: Vec<T>,
    selected: Option<T>,
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
            focused: false,
            values,
            selected: None,
            state: ListState::default(),
        })
    }

    pub fn focus(&mut self) {
        self.focused = true;
    }

    pub fn unfocus(&mut self) {
        self.focused = false;
    }

    pub fn get_value(&self) -> Option<T> {
        self.selected.clone()
    }

    pub fn handle_event(&mut self, event: &Event) -> Option<ListInputEvent> {
        if let Event::Key(key) = event {
            match key.code {
                KeyCode::Esc => return Some(ListInputEvent::Escape),
                KeyCode::Enter => return Some(ListInputEvent::Select),
                KeyCode::Char('j') => {
                    self.state.select_next();
                    if let Some(idx) = self.state.selected() {
                        self.selected = Some(self.values[idx].clone());
                    }
                }

                KeyCode::Char('k') => {
                    self.state.select_previous();
                    if let Some(idx) = self.state.selected() {
                        self.selected = Some(self.values[idx].clone())
                    }
                }
                _ => {}
            }
        }

        None
    }

    pub fn render(&mut self, frame: &mut Frame, area: Rect) {
        let block = Block::bordered().title(self.title);
        let content_area = block.inner(area);

        if self.focused {
            frame.render_widget(block.clone().border_type(BorderType::Double), area);
            frame.render_stateful_widget(&self.list, content_area, &mut self.state);
        } else if let Some(selected) = &self.selected {
            frame.render_widget(block, area);
            frame.render_widget(selected.to_string(), content_area);
        } else {
            frame.render_widget(block, area);
        }
    }
}
