use std::{
    collections::HashMap,
    env,
    io::{self, Stdout},
    time::Duration,
};

use crossterm::event::{self, Event, KeyCode};
use dotenv::dotenv;
use ratatui::{
    layout::{Alignment, Constraint, Layout, Rect},
    prelude::CrosstermBackend,
    style::{Style, Stylize},
    symbols::Marker,
    widgets::{Axis, Block, BorderType, Chart, Dataset, GraphType, Row, Table, TableState},
    Frame, Terminal,
};
use sqlx::{Pool, Sqlite};

const MONTH_LABELS: [&'static str; 12] = [
    "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
];

#[derive(Debug)]
pub enum Error {
    Sqlx(sqlx::Error),
    Io(io::Error),
}

impl From<sqlx::Error> for Error {
    fn from(value: sqlx::Error) -> Self {
        Error::Sqlx(value)
    }
}

impl From<io::Error> for Error {
    fn from(value: io::Error) -> Self {
        Error::Io(value)
    }
}

#[derive(Clone)]
struct Artist {
    artist_id: i64,
    name: String,
    from: String,
}

impl DataSet for Artist {
    async fn load_all(conn: &Pool<Sqlite>) -> Result<Vec<Self>, Error> {
        Ok(sqlx::query_as!(Artist, "SELECT * FROM artist")
            .fetch_all(conn)
            .await?)
    }
}

impl<'a> From<Artist> for Row<'a> {
    fn from(value: Artist) -> Self {
        Row::new(vec![value.artist_id.to_string(), value.name, value.from])
    }
}

#[derive(Clone)]
struct Venue {
    venue_id: i64,
    name: String,
    city: String,
}

impl DataSet for Venue {
    async fn load_all(conn: &Pool<Sqlite>) -> Result<Vec<Self>, Error> {
        Ok(sqlx::query_as!(Venue, "SELECT * FROM venue")
            .fetch_all(conn)
            .await?)
    }
}

impl<'a> From<Venue> for Row<'a> {
    fn from(value: Venue) -> Self {
        Row::new(vec![value.venue_id.to_string(), value.name, value.city])
    }
}

#[derive(Clone)]
struct Date {
    year: u32,
    month: u32,
    date: u32,
}

impl ToString for Date {
    fn to_string(&self) -> String {
        String::from(format!(
            "{:04}/{:02}/{:02}",
            self.year, self.month, self.date
        ))
    }
}

impl From<String> for Date {
    fn from(value: String) -> Self {
        let components: Vec<u32> = value.split("/").map(|s| s.parse().unwrap()).collect();

        assert!(components.len() == 3);

        Self {
            year: components[0],
            month: components[1],
            date: components[2],
        }
    }
}

#[derive(Clone)]
enum Act {
    Main = 1,
    Support,
    Shared,
}

impl From<i64> for Act {
    fn from(value: i64) -> Act {
        match value {
            1 => Act::Main,
            2 => Act::Support,
            3 => Act::Shared,
            _ => Act::Main,
        }
    }
}

impl ToString for Act {
    fn to_string(&self) -> String {
        match self {
            Act::Main => String::from("Main Act"),
            Act::Support => String::from("Support Act"),
            Act::Shared => String::from("Shared Headliner"),
        }
    }
}

#[derive(Clone)]
struct Gig {
    artist_id: i64,
    venue_id: i64,
    date: Date,
    act: Act,
}

impl DataSet for Gig {
    async fn load_all(conn: &Pool<Sqlite>) -> Result<Vec<Self>, Error> {
        Ok(sqlx::query_as!(Gig, "SELECT * FROM gig")
            .fetch_all(conn)
            .await?)
    }
}

impl<'a> From<Gig> for Row<'a> {
    fn from(value: Gig) -> Self {
        Row::new(vec![
            value.artist_id.to_string(),
            value.venue_id.to_string(),
            value.date.to_string(),
            value.act.to_string(),
        ])
    }
}

trait DataSet: Sized {
    async fn load_all(pool: &Pool<Sqlite>) -> Result<Vec<Self>, Error>;
}

struct DataTable<'d> {
    name: &'static str,
    is_focused: bool,
    border_style: Style,
    table: Table<'d>,
    state: TableState,
}

impl<'d> DataTable<'d> {
    async fn new<T: Into<Row<'d>> + DataSet + Clone>(
        name: &'static str,
        conn: &Pool<Sqlite>,
        constraints: Vec<Constraint>,
        headers: Vec<&'static str>,
    ) -> Result<Self, Error> {
        let data = T::load_all(conn).await?;

        let table = Table::new(data, constraints)
            .header(Row::new(headers))
            .style(Style::new().white())
            .row_highlight_style(Style::new().dark_gray().on_gray());

        Ok(Self {
            name,
            is_focused: false,
            border_style: Style::new().red(),
            table,
            state: TableState::default(),
        })
    }

    fn focus(&mut self) {
        self.is_focused = true;
        self.border_style = self.border_style.green();
    }

    fn unfocus(&mut self) {
        self.is_focused = false;
        self.border_style = self.border_style.red();
    }

    fn render(&mut self, frame: &mut Frame, area: Rect) {
        let block = Block::bordered()
            .border_type(BorderType::Rounded)
            .title(self.name)
            .style(self.border_style);

        let content_area = block.inner(area);

        frame.render_widget(block, area);
        frame.render_stateful_widget(&self.table, content_area, &mut self.state);
    }

    fn handle_event(&mut self, event: Event) {
        match event {
            Event::Key(key) => match key.code {
                KeyCode::Char('k') => self.state.select_previous(),
                KeyCode::Char('j') => self.state.select_next(),
                _ => {}
            },
            _ => {}
        }
    }
}

#[derive(Hash)]
enum ColumnName {
    Data,
    Graph,
}

#[derive(Hash, PartialEq, Eq)]
enum TableName {
    Artist,
    Gig,
    Venue,
}

impl TableName {
    fn next(&self) -> Self {
        match self {
            TableName::Artist => TableName::Venue,
            TableName::Venue => TableName::Gig,
            TableName::Gig => TableName::Gig,
        }
    }

    fn prev(&self) -> Self {
        match self {
            TableName::Artist => TableName::Artist,
            TableName::Venue => TableName::Artist,
            TableName::Gig => TableName::Venue,
        }
    }
}

struct DataColumn<'a> {
    is_focused: bool,
    apps: HashMap<TableName, DataTable<'a>>,
    focused_app: TableName,
}

impl<'a> DataColumn<'a> {
    async fn new(pool: &Pool<Sqlite>) -> Result<Self, Error> {
        let artist_table = DataTable::new::<Artist>(
            "Artist",
            pool,
            [Constraint::Length(20); 3].to_vec(),
            vec!["Artist Id", "Name", "From"],
        )
        .await?;

        let venue_table = DataTable::new::<Venue>(
            "Venue",
            pool,
            [Constraint::Length(20); 3].to_vec(),
            vec!["Venue Id", "Name", "City"],
        )
        .await?;

        let gig_table = DataTable::new::<Gig>(
            "Gig",
            pool,
            [Constraint::Length(15); 4].to_vec(),
            vec!["Artist Id", "Venue Id", "Date", "Act"],
        )
        .await?;

        Ok(Self {
            is_focused: true,
            apps: HashMap::from([
                (TableName::Artist, artist_table),
                (TableName::Venue, venue_table),
                (TableName::Gig, gig_table),
            ]),
            focused_app: TableName::Artist,
        })
    }

    fn focus(&mut self) {
        self.is_focused = true;

        self.apps
            .get_mut(&self.focused_app)
            .expect("focused app should be available")
            .focus();
    }

    fn unfocus(&mut self) {
        self.is_focused = false;

        self.apps.iter_mut().for_each(|(_, app)| app.unfocus());
    }

    fn render(&mut self, frame: &mut Frame, area: Rect) {
        let border_style = Style::new().blue();
        let mut block = Block::bordered()
            .border_type(BorderType::Thick)
            .border_style(border_style);

        if self.is_focused {
            block = block.border_style(border_style.yellow());
        }

        let content_area = block.inner(area);
        frame.render_widget(block, area);

        let [top, middle, bottom] = Layout::vertical([Constraint::Fill(1); 3]).areas(content_area);

        self.apps
            .get_mut(&TableName::Artist)
            .expect("Artist Table should be set")
            .render(frame, top);

        self.apps
            .get_mut(&TableName::Venue)
            .expect("Venue Table should be set")
            .render(frame, middle);

        self.apps
            .get_mut(&TableName::Gig)
            .expect("Gig Table should be set")
            .render(frame, bottom);
    }

    fn handle_event(&mut self, event: Event) {
        match event {
            Event::Key(key) => match key.code {
                KeyCode::Char('K') => {
                    self.apps.get_mut(&self.focused_app).unwrap().unfocus();
                    self.focused_app = self.focused_app.prev();
                    self.apps.get_mut(&self.focused_app).unwrap().focus();
                }
                KeyCode::Char('J') => {
                    self.apps.get_mut(&self.focused_app).unwrap().unfocus();
                    self.focused_app = self.focused_app.next();
                    self.apps.get_mut(&self.focused_app).unwrap().focus();
                }
                _ => {}
            },
            _ => {}
        }

        self.apps
            .get_mut(&self.focused_app)
            .unwrap()
            .handle_event(event);
    }
}

struct GraphColumn {
    is_focused: bool,
    gigs: Vec<Gig>,
}

impl GraphColumn {
    fn new(gigs: Vec<Gig>) -> Self {
        Self {
            gigs,
            is_focused: false,
        }
    }

    fn focus(&mut self) {
        self.is_focused = true;
    }

    fn unfocus(&mut self) {
        self.is_focused = false;
    }

    fn render(&mut self, frame: &mut Frame, area: Rect) {
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
            .name("Gig Dates".white())
            .marker(Marker::HalfBlock)
            .graph_type(GraphType::Bar)
            .style(Style::new().white())
            .data(&gig_data);

        let chart = Chart::new(vec![gig_dataset])
            .block(
                Block::bordered()
                    .border_type(BorderType::Double)
                    .border_style(Style::default().magenta()),
            )
            .x_axis(
                Axis::default()
                    .title("Months".white())
                    .bounds([1f64, 12f64])
                    .labels(MONTH_LABELS)
                    .labels_alignment(Alignment::Center),
            )
            .y_axis(
                Axis::default()
                    .title("Gig Count".white())
                    .bounds([0f64, 2f64])
                    .labels(["0.0", "1.0", "2.0"]),
            )
            .hidden_legend_constraints((Constraint::Ratio(1, 2), Constraint::Ratio(1, 2)));

        frame.render_widget(chart, content_area);
    }

    fn handle_event(&mut self, event: Event) {}
}

struct App<'a> {
    terminal: Terminal<CrosstermBackend<Stdout>>,
    focused_column: ColumnName,
    data_column: DataColumn<'a>,
    graph_column: GraphColumn,
}

impl<'a> App<'a> {
    async fn new(db_url: &'a str) -> Result<Self, Error> {
        let pool = Pool::connect(db_url).await?;

        let mut terminal = ratatui::init();
        terminal.clear()?;

        let mut data_column = DataColumn::new(&pool).await?;
        data_column.focus();

        let gigs = Gig::load_all(&pool).await?;
        let graph_column = GraphColumn::new(gigs);

        Ok(Self {
            terminal,
            data_column,
            graph_column,
            focused_column: ColumnName::Data,
        })
    }

    async fn run(&mut self) -> Result<(), Error> {
        loop {
            if self.handle_events()? {
                break;
            }

            self.render()?;
        }

        Ok(())
    }

    fn handle_events(&mut self) -> Result<bool, Error> {
        if !event::poll(Duration::from_secs(0))? {
            return Ok(false);
        }

        let event = event::read()?;

        match event {
            Event::Key(key) => match key.code {
                KeyCode::Char('q') => return Ok(true),
                KeyCode::Char('H') => {
                    if matches!(self.focused_column, ColumnName::Graph) {
                        self.focused_column = ColumnName::Data;
                        self.graph_column.unfocus();
                        self.data_column.focus();
                    }
                }
                KeyCode::Char('L') => {
                    if matches!(self.focused_column, ColumnName::Data) {
                        self.focused_column = ColumnName::Graph;
                        self.data_column.unfocus();
                        self.graph_column.focus();
                    }
                }
                _ => {}
            },
            _ => {}
        }

        match self.focused_column {
            ColumnName::Data => self.data_column.handle_event(event),
            ColumnName::Graph => self.graph_column.handle_event(event),
        }

        Ok(false)
    }

    fn render(&mut self) -> Result<(), Error> {
        self.terminal.draw(|frame| {
            let [left, right] = Layout::horizontal([Constraint::Fill(1); 2]).areas(frame.area());

            self.data_column.render(frame, left);
            self.graph_column.render(frame, right);
        })?;

        Ok(())
    }
}

#[async_std::main]
async fn main() -> Result<(), Error> {
    dotenv().ok();

    let db_url = env::var("DATABASE_URL").expect("env var DATABASE_URL should be set");

    let mut app = App::new(db_url.as_str()).await?;

    let result = app.run().await;

    ratatui::restore();

    result
}
