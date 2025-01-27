use crossterm::event::{Event, KeyCode};
use ratatui::{
    layout::Rect,
    style::Stylize,
    text::Text,
    widgets::{Block, BorderType},
    Frame,
};

pub enum SaveButtonEvent {
    Escape,
    Save,
}

pub struct SaveButton {
    focused: bool,
}

impl SaveButton {
    pub fn new() -> Self {
        Self { focused: false }
    }

    pub fn focus(&mut self) {
        self.focused = true;
    }

    pub fn unfocus(&mut self) {
        self.focused = false;
    }

    pub fn handle_event(&mut self, event: &Event) -> Option<SaveButtonEvent> {
        if let Event::Key(key) = event {
            match key.code {
                KeyCode::Esc => return Some(SaveButtonEvent::Escape),
                KeyCode::Enter => return Some(SaveButtonEvent::Save),
                _ => {}
            }
        }

        None
    }

    pub fn render(&mut self, frame: &mut Frame, area: Rect) {
        let mut block = Block::bordered().white();
        if self.focused {
            block = block.border_type(BorderType::Double);
        }

        let content_area = block.inner(area);

        frame.render_widget(block, area);
        frame.render_widget(Text::from("Save").centered(), content_area);
    }
}
