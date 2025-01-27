use crossterm::event::{Event, KeyCode, KeyModifiers};
use ratatui::{
    layout::Rect,
    widgets::{Block, BorderType},
    Frame,
};

pub enum TextInputEvent {
    Escape,
    Save,
}

pub struct TextInput<'a> {
    title: &'a str,
    value: String,
    focused: bool,
}

impl<'a> TextInput<'a> {
    pub fn new(title: &'a str) -> Self {
        Self {
            title,
            value: String::new(),
            focused: false,
        }
    }

    pub fn get_value(&self) -> Option<String> {
        if self.value.is_empty() {
            return None;
        }

        Some(self.value.clone())
    }

    pub fn focus(&mut self) {
        self.focused = true;
    }

    pub fn unfocus(&mut self) {
        self.focused = false;
    }

    pub fn handle_event(&mut self, event: &Event) -> Option<TextInputEvent> {
        if let Event::Key(key) = event {
            match key.code {
                KeyCode::Esc => return Some(TextInputEvent::Escape),
                KeyCode::Enter => return Some(TextInputEvent::Save),
                KeyCode::Backspace => {
                    self.value.pop();
                }
                KeyCode::Char(char) => {
                    if key.modifiers == KeyModifiers::CONTROL {
                        return None;
                    }

                    self.value.push(char);
                }
                _ => {}
            }
        }

        None
    }

    pub fn render(&mut self, frame: &mut Frame, area: Rect) {
        let mut block = Block::bordered().title(self.title);

        if self.focused {
            block = block.border_type(BorderType::Double);
        }

        let content_area = block.inner(area);

        frame.render_widget(block, area);
        frame.render_widget(&self.value, content_area);
    }
}
