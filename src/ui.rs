use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{error::Error, io};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, BorderType, Borders, List, ListItem, ListState, Paragraph, Tabs, Wrap},
    Frame, Terminal,
};

use crate::Config;

const DEFAULT_TIEL: &str = "Default title";

struct StatefulList<T> {
    state: ListState,
    items: Vec<T>,
}

impl<T> StatefulList<T> {
    fn with_items(items: Vec<T>) -> StatefulList<T> {
        StatefulList {
            state: ListState::default(),
            items,
        }
    }

    fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    fn unselect(&mut self) {
        self.state.select(None);
    }
}

struct App<'a> {
    pub config: Config,
    pub titles: Vec<&'a str>,
    pub index: usize,
    pub items: Vec<StatefulList<(String, usize)>>,
}

impl<'a> App<'a> {
    fn new(config: Config, categoryes: Vec<&'a str>) -> App<'a> {
        let mut items = vec![];

        let item = config
            .outlines(0)
            .into_iter()
            .enumerate()
            .map(|(idx, value)| (value, idx))
            .collect::<Vec<(String, usize)>>();

        items.push(StatefulList::with_items(item));

        App {
            config,
            titles: categoryes,
            index: 0,
            items,
        }
    }

    pub fn next(&mut self) {
        self.index = (self.index + 1) % self.titles.len();

        if let Some(_value) = self.items.get(self.index) {
        } else {
            let item = self
                .config
                .outlines(self.index)
                .into_iter()
                .enumerate()
                .map(|(idx, value)| (value, idx))
                .collect::<Vec<(String, usize)>>();

            self.items.push(StatefulList::with_items(item));
        }
    }

    pub fn previous(&mut self) {
        if self.index > 0 {
            self.index -= 1;
        } else {
            self.index = self.titles.len() - 1;
        }

        if let Some(_value) = self.items.get(self.index) {
        } else {
            let item = self
                .config
                .outlines(self.index)
                .into_iter()
                .enumerate()
                .map(|(idx, value)| (value, idx))
                .collect::<Vec<(String, usize)>>();

            self.items.push(StatefulList::with_items(item));
        }
    }
}

pub fn run_ui(config: &Config) -> anyhow::Result<()> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let binding = config.category();
    let category = binding.iter().map(|value| value.as_str()).collect();
    // create app and run it
    let app = App::new(config.clone(), category);
    let res = run_app(&mut terminal, app);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, &mut app))?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => return Ok(()),
                KeyCode::Right => app.next(),
                KeyCode::Left => app.previous(),
                KeyCode::Up => {
                    app.items.get_mut(app.index).unwrap().previous();
                }
                KeyCode::Down => {
                    app.items.get_mut(app.index).unwrap().next();
                }
                _ => {}
            }
        }
    }
}

fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let size = f.size();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(10), Constraint::Percentage(80)].as_ref())
        .split(f.size());

    let block = Block::default().style(Style::default().bg(Color::White).fg(Color::Black));
    f.render_widget(block, size);

    let titles = app
        .titles
        .iter()
        .map(|t| {
            let (first, rest) = t.split_at(1);
            Spans::from(vec![
                Span::styled(first, Style::default().fg(Color::Yellow)),
                Span::styled(rest, Style::default().fg(Color::Green)),
            ])
        })
        .collect();

    let tabs = Tabs::new(titles)
        .block(Block::default().borders(Borders::ALL).title("Tabs"))
        .select(app.index)
        .style(Style::default().fg(Color::Cyan))
        .highlight_style(
            Style::default()
                .add_modifier(Modifier::BOLD)
                .bg(Color::Black),
        );
    f.render_widget(tabs, chunks[0]);

    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(chunks[1]);

    let n = app.index;
    
    // get title
    let title = app.titles.get(n).unwrap_or(&DEFAULT_TIEL).clone();

    // Iterate through all elements in the `items` app and append some debug text to it.
    if let Some(value) = app.items.get(n) {
        let items = value
            .items
            .iter()
            .map(|i| {
                let lines = vec![Spans::from(i.0.clone())];
                ListItem::new(lines).style(Style::default().fg(Color::Black).bg(Color::White))
            })
            .collect::<Vec<ListItem>>();

        // Create a List from all list items and highlight the currently selected one
        let items = List::new(items)
            .block(Block::default().borders(Borders::ALL).title(title))
            .highlight_style(
                Style::default()
                    .bg(Color::LightGreen)
                    .add_modifier(Modifier::BOLD),
            )
            .highlight_symbol(">> ");

        // We can now render the item list
        f.render_stateful_widget(items, chunks[0], &mut app.items[n].state);
    }
}
