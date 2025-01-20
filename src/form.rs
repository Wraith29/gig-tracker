use crossterm::event::{Event, KeyCode};
use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::{Style, Styled, Stylize},
    widgets::{Block, BorderType, Tabs},
    Frame,
};

struct TabArtist {
    name: String,
    from: String,
}

struct TabVenue {
    name: String,
    city: String,
}

struct TabGig {
    artist_id: String,
    venue_id: String,
    date: String,
    act: String,
}

enum Tab {
    Artists(TabArtist),
    Venue(TabVenue),
    Gig(TabGig),
}

pub struct Form<'a> {
    tabs: Tabs<'a>,
    selected_tab: usize,
    current_tab: Tab,
}

impl<'a> Form<'a> {
    pub fn new() -> Self {
        Self {
            tabs: Tabs::new(vec!["Artist", "Venue", "Gig"]),
            selected_tab: 0,
            current_tab: Tab::Artists(TabArtist {
                name: "".into(),
                from: "".into(),
            }),
        }
    }

    pub fn handle_events(&mut self, event: Event) {
        match event {
            Event::Key(key) => match key.code {
                KeyCode::Char('L') => {
                    if self.selected_tab == 2 {
                        self.selected_tab = 0;
                    }
                    self.tabs = self.tabs.clone().select(Some(self.selected_tab));
                }
                _ => {}
            },
            _ => {}
        }
    }

    pub fn render(&mut self, frame: &mut Frame, area: Rect) {
        let block = Block::bordered()
            .title("Create New")
            .border_type(BorderType::Rounded)
            .border_style(Style::new().blue());

        let content_area = block.inner(area);
        frame.render_widget(block, area);

        let [header_area, form_area, submit_area] = Layout::vertical([
            Constraint::Length(1),
            Constraint::Fill(1),
            Constraint::Length(1),
        ])
        .areas(content_area);

        frame.render_widget(&self.tabs, header_area);

        match &self.current_tab {
            Tab::Artists(artist) => self.render_artist(artist, frame, form_area),
            Tab::Venue(venue) => self.render_venue(venue, frame, form_area),
            Tab::Gig(gig) => self.render_gig(gig, frame, form_area),
        }
    }

    fn render_artist(&self, artist: &TabArtist, frame: &mut Frame, area: Rect) {
        let [vertical_area] = Layout::vertical([Constraint::Length(3)]).areas(area);
        let [top, bottom] = Layout::horizontal([Constraint::Fill(1); 2]).areas(vertical_area);

        let name_block = Block::bordered().title("Name").style(Style::new().yellow());
        let name_area = name_block.inner(top);
        let from_block = Block::bordered()
            .title("From")
            .style(Style::new().light_cyan());
        let from_area = from_block.inner(bottom);

        frame.render_widget(name_block, top);
        frame.render_widget(&artist.name, name_area);

        frame.render_widget(from_block, bottom);
        frame.render_widget(&artist.from, from_area);
    }

    fn render_venue(&self, venue: &TabVenue, frame: &mut Frame, area: Rect) {}

    fn render_gig(&self, gig: &TabGig, frame: &mut Frame, area: Rect) {}
}
