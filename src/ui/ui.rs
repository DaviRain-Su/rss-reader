use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, List, ListItem, Tabs},
    Frame,
};

use super::{app::App, logic::get_titles, DEFAULT_TIEL};

pub fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App) {
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
        // display title
        let items = value
            .items()
            .iter()
            .map(|i| {
                let lines = vec![Spans::from(i.0.title.clone())];
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
        f.render_stateful_widget(items.clone(), chunks[0], &mut app.items[n].state_mut());
        f.render_stateful_widget(items, chunks[1], &mut app.items[n].state_mut());
    }
    // // display rss_url content
    // if let Some(value) = app.items.get(n) {
    //     // display title
    //     let items = value
    //         .items()
    //         .iter()
    //         .map(|i| {
    //             let rss_url = i.0.rss_url.clone();
    //             rss_url
    //         })
    //         .collect::<Vec<String>>();

    //     for item in items.iter() {
    //         let titles = get_titles(item).unwrap_or(vec!["unknow 404".into()]).into_iter().map(|item| {
    //             let lines = vec![Spans::from(item)];
    //             ListItem::new(lines).style(Style::default().fg(Color::Black).bg(Color::White))
    //         }).collect::<Vec<ListItem>>();

    //         // Create a List from all list items and highlight the currently selected one
    //         let items = List::new(titles)
    //             .block(
    //                 Block::default()
    //                     .borders(Borders::ALL)
    //                     .title(title)
    //                     .title_alignment(Alignment::Center),
    //             )
    //             .highlight_style(
    //                 Style::default()
    //                     .bg(Color::LightGreen)
    //                     .add_modifier(Modifier::BOLD),
    //             )
    //             .highlight_symbol(">> ");

    //         // We can now render the item list
    //         f.render_stateful_widget(items, chunks[1], &mut app.items[n].state_mut());
    //     }
    // }
}
