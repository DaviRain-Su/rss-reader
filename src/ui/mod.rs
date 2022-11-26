use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io;
use tui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};

use crate::Config;

pub mod app;
pub mod stateful_list;
use app::App;
pub mod ui;
use ui::ui;
pub mod logic;
pub mod modes;

const DEFAULT_TIEL: &str = "Default title";

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
    let app = App::new(config.clone(), category)?;
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

fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App<'_>) -> anyhow::Result<()> {
    loop {
        terminal.draw(|f| {
            ui(f, &mut app);
        })?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => return Ok(()),
                KeyCode::Right => app.next(),
                KeyCode::Left => app.previous(),
                KeyCode::Up => {
                    app.current_tab_items
                        .get_mut(app.tabs_index)
                        .unwrap()
                        .previous();
                }
                KeyCode::Down => {
                    app.current_tab_items
                        .get_mut(app.tabs_index)
                        .unwrap()
                        .next();
                }
                _ => {}
            }
        }
    }
}
