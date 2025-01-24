use crossterm::event::Event;
use ratatui::{layout::Rect, Frame};

use super::textinput::{InputEvent, TextInput};

pub struct ArtistForm<'a> {
    name: TextInput<'a>,
}

impl ArtistForm<'_> {
    pub fn new() -> Self {
        Self {
            name: TextInput::new("Name"),
        }
    }

    pub fn handle_event(&mut self, event: Event) {
        if let Some(event) = self.name.handle_event(event) {
            match event {
                InputEvent::Save => self.name.unfocus(),
                InputEvent::Unfocus => self.name.unfocus(),
            }
        }
    }

    pub fn render(&mut self, frame: &mut Frame, area: Rect) {
        self.name.render(frame, area);
    }
}
