use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, List, ListItem, Paragraph, Tabs, Wrap},
    Frame,
};

use crate::{config::TitleAndRssUrl, db::titles::Titles};

use super::{app::App, logic::get_titles, DEFAULT_TIEL};

pub fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    // predraw
    let chunks = predraw(f);

    let block = Block::default().style(Style::default().bg(Color::White).fg(Color::Black));
    f.render_widget(block, f.size());

    // draw tabs
    draw_tabs(f, app, chunks[0]);

    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(25), Constraint::Percentage(75)].as_ref())
        .split(chunks[1]);

    // draw entry title
    draw_entry_title(f, app, chunks[0])
}

pub fn predraw<B: Backend>(f: &Frame<B>) -> Vec<Rect> {
    Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(10), Constraint::Percentage(80)].as_ref())
        .split(f.size())
}

pub fn draw_tabs<B: Backend>(f: &mut Frame<B>, app: &mut App, area: Rect) {
    let titles = app
        .tabs_titles
        .iter()
        .map(|t| {
            let (first, rest) = t.split_at(1);
            Spans::from(vec![
                Span::styled(first, Style::default().fg(Color::Yellow)),
                Span::styled(rest, Style::default().fg(Color::Green)),
            ])
        })
        .collect();

    // display tabs
    let tabs = Tabs::new(titles)
        .block(Block::default().borders(Borders::ALL).title("Tabs"))
        .select(app.current_tabs_index)
        .style(Style::default().fg(Color::Cyan))
        .highlight_style(
            Style::default()
                .add_modifier(Modifier::BOLD)
                .bg(Color::Black),
        );
    f.render_widget(tabs, area);
}

pub fn draw_entry_title<B: Backend>(f: &mut Frame<B>, app: &mut App, area: Rect) {
    // get title todo need to get current tabs title
    let title = app.current_tabs_title.clone();

    // Iterate through all elements in the `items` app and append some debug text to it.
    // display title
    let items = app
        .current_tab_items
        .items()
        .iter()
        .map(|i| {
            let lines = vec![Spans::from(i.title.clone())];
            ListItem::new(lines).style(Style::default().fg(Color::Black).bg(Color::White))
        })
        .collect::<Vec<ListItem>>();

    // Create a List from all list items and highlight the currently selected one
    let items = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(title)
                .title_alignment(Alignment::Center),
        )
        .highlight_style(
            Style::default()
                .bg(Color::LightGreen)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(">> ");

    // We can now render the item list
    f.render_stateful_widget(items.clone(), area, &mut app.current_tab_items.state_mut());
}
