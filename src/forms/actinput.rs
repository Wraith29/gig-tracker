use crate::act::Act;
use crossterm::event::{Event, KeyCode};
use ratatui::{
    layout::Rect,
    style::{Style, Stylize},
    widgets::{Block, BorderType, Tabs},
    Frame,
};

const TAB_HEADERS: [&str; 3] = ["Main Act", "Support Act", "Shared Headliner"];

pub enum ActInputEvent {
    Escape,
    Select,
}

pub struct ActInput {
    focused: bool,
    selected: Option<Act>,
    error: Option<String>,

    current_tab: Act,
}

impl ActInput {
    pub fn new() -> Self {
        Self {
            focused: false,
            selected: None,
            current_tab: Act::Main,
            error: None,
        }
    }

    pub fn focus(&mut self) {
        self.focused = true;
    }

    pub fn unfocus(&mut self) {
        self.focused = false;
    }

    pub fn get_value(&self) -> Option<Act> {
        self.selected.clone()
    }

    pub fn set_err(&mut self, err: String) {
        self.error = Some(err);
    }

    pub fn handle_event(&mut self, event: &Event) -> Option<ActInputEvent> {
        if let Event::Key(key) = event {
            match key.code {
                KeyCode::Esc => return Some(ActInputEvent::Escape),
                KeyCode::Enter => {
                    self.selected = Some(self.current_tab.clone());
                    return Some(ActInputEvent::Select);
                }
                KeyCode::Char('j') => {
                    self.current_tab = self.current_tab.next();
                }
                KeyCode::Char('k') => {
                    self.current_tab = self.current_tab.prev();
                }
                _ => {}
            }
        }

        None
    }

    pub fn render(&mut self, frame: &mut Frame, area: Rect) {
        let mut block = Block::bordered().title_top("Act Type");
        let content_area = block.inner(area);

        if self.focused {
            block = block.border_type(BorderType::Double)
        }

        if let Some(err) = self.error.clone() {
            block = block.border_style(Style::new().red()).title_bottom(err);
        }

        let tabs = Tabs::new(TAB_HEADERS).select(self.current_tab.clone());

        frame.render_widget(block, area);
        frame.render_widget(tabs, content_area);
    }
}
