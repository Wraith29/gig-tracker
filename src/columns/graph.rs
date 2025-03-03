use crate::{
    dataset::DataSet,
    date::{Month, MONTHS},
    error::Error,
    gig::Gig,
};
use ratatui::{
    layout::Rect,
    style::{Style, Stylize},
    text::Line,
    widgets::{Bar, BarChart, BarGroup, Block, BorderType},
    Frame,
};
use sqlx::{Pool, Sqlite};

pub struct GraphColumn {
    pool: Pool<Sqlite>,
    is_focused: bool,
    gigs: Vec<Gig>,
}

impl GraphColumn {
    pub async fn new(pool: Pool<Sqlite>) -> Result<Self, Error> {
        let gigs = Gig::load_all(&pool).await?;

        Ok(Self {
            pool,
            gigs,
            is_focused: false,
        })
    }

    pub async fn reload_data(&mut self) -> Result<(), Error> {
        let gigs = Gig::load_all(&self.pool).await?;
        self.gigs = gigs;

        Ok(())
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

        let dataset: Vec<Bar> = MONTHS
            .iter()
            .map(|month| {
                let count =
                    u64::try_from(self.gigs.iter().filter(|g| g.date.month.eq(month)).count())
                        .expect("Value should be a valid u64");

                create_vertical_bar(month, count)
            })
            .collect();

        let chart = BarChart::default()
            .bar_width(3)
            .data(BarGroup::default().bars(&dataset))
            .block(
                Block::bordered()
                    .title(Line::from("Gig Chart").white().bold().centered())
                    .border_type(BorderType::Double)
                    .border_style(Style::default().magenta()),
            );

        frame.render_widget(chart, content_area);
    }
}

fn create_vertical_bar<'a>(month: &Month, count: u64) -> Bar<'a> {
    Bar::default().value(count).label(month.to_string().into())
}
