use crossterm::event::{Event, KeyCode};
use ratatui::{layout::Rect, widgets::Block, Frame};

pub enum InputEvent {
    Save,
    Unfocus,
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

    pub fn focus(&mut self) {
        self.focused = true;
    }

    pub fn unfocus(&mut self) {
        self.focused = false;
    }

    pub fn handle_event(&mut self, event: Event) -> Option<InputEvent> {
        if let Event::Key(key) = event {
            match key.code {
                KeyCode::Esc => return Some(InputEvent::Unfocus),
                KeyCode::Enter => return Some(InputEvent::Save),
                KeyCode::Backspace => {
                    self.value.pop();
                }
                KeyCode::Char(char) => {
                    self.value.push(char);
                }
                _ => {}
            }
        }

        None
    }

    pub fn render(&mut self, frame: &mut Frame, area: Rect) {
        let block = Block::bordered().title(self.title);
        let content_area = block.inner(area);

        frame.render_widget(block, area);
        frame.render_widget(&self.value, content_area);
    }
}
