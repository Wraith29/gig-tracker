use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::Stylize,
    widgets::{Block, Borders, Clear, Tabs},
    Frame,
};

mod artist;
mod textinput;

const FORM_TABS: [&str; 4] = ["Artist", "Venue", "Gig", "City"];

pub struct Form<'a> {
    tabs: Tabs<'a>,
}

impl Form<'_> {
    pub fn new() -> Self {
        let tabs = Tabs::new(FORM_TABS);

        Self { tabs }
    }

    pub fn render(&mut self, frame: &mut Frame, area: Rect) {
        let [_, mid_col, _] = Layout::horizontal([Constraint::Fill(1); 3]).areas(area);
        let [_, mid_area, _] = Layout::vertical([Constraint::Fill(1); 3]).areas(mid_col);

        let block = Block::bordered().white().title("Create New");

        let [tabs_area, content_area] =
            Layout::vertical([Constraint::Length(2), Constraint::Fill(1)])
                .areas(block.inner(mid_area));

        frame.render_widget(Clear {}, mid_area);

        frame.render_widget(block, mid_area);

        frame.render_widget(
            &self
                .tabs
                .clone()
                .block(Block::new().borders(Borders::BOTTOM)),
            tabs_area,
        );

        frame.render_widget("Content", content_area);
    }
}
