use crate::gig::Gig;
use ratatui::{
    layout::{Alignment, Rect},
    style::{Style, Stylize},
    symbols::Marker,
    text::Line,
    widgets::{Axis, Block, BorderType, Chart, Dataset, GraphType},
    Frame,
};
use std::collections::HashMap;

const MONTH_LABELS: [&'static str; 12] = [
    "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
];

pub struct GraphColumn {
    is_focused: bool,
    gigs: Vec<Gig>,
}

impl GraphColumn {
    pub fn new(gigs: Vec<Gig>) -> Self {
        Self {
            gigs,
            is_focused: false,
        }
    }

    pub fn focus(&mut self) {
        self.is_focused = true;
    }

    pub fn unfocus(&mut self) {
        self.is_focused = false;
    }

    pub fn render(&mut self, frame: &mut Frame, area: Rect) {
        let border_style = Style::new().blue();
        let mut block = Block::bordered()
            .border_type(BorderType::Thick)
            .border_style(border_style);

        if self.is_focused {
            block = block.border_style(border_style.yellow());
        }

        let content_area = block.inner(area);
        frame.render_widget(block, area);

        let mut gig_data_map: HashMap<u32, u32> = HashMap::new();

        self.gigs.iter().for_each(|gig| {
            if let Some(month) = gig_data_map.get_mut(&gig.date.month) {
                *month = *month + 1;
            } else {
                gig_data_map.insert(gig.date.month, 1);
            }
        });

        let gig_data: Vec<(f64, f64)> = gig_data_map
            .iter()
            .map(|(k, v)| (f64::from(*k), f64::from(*v)))
            .collect();

        let gig_dataset = Dataset::default()
            .marker(Marker::HalfBlock)
            .graph_type(GraphType::Bar)
            .style(Style::new().white())
            .data(&gig_data);

        let chart = Chart::new(vec![gig_dataset])
            .block(
                Block::bordered()
                    .title(Line::from("Gig Chart").white().bold().centered())
                    .border_type(BorderType::Double)
                    .border_style(Style::default().magenta()),
            )
            .x_axis(
                Axis::default()
                    .title("Months".white())
                    .labels(MONTH_LABELS)
                    .bounds([1f64, 12f64])
                    .labels_alignment(Alignment::Right),
            )
            .y_axis(
                Axis::default()
                    .title("Gig Count".white())
                    .bounds([0f64, 100f64])
                    .labels(["0", "50", "100"]),
            );

        frame.render_widget(chart, content_area);
    }
}
