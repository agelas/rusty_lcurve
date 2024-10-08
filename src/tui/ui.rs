use crate::tui::tui::App;
use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::{Color, Style},
    text::{self, Span},
    widgets::{Block, Paragraph, Tabs},
    Frame,
};

pub fn draw(frame: &mut Frame, app: &mut App) {
    let chunks = Layout::vertical([Constraint::Length(3), Constraint::Min(0)]).split(frame.area());
    let tabs = app
        .tabs
        .titles
        .iter()
        .map(|t| text::Line::from(Span::styled(*t, Style::default().fg(Color::Green))))
        .collect::<Tabs>()
        .block(Block::bordered().title(app.title))
        .highlight_style(Style::default().fg(Color::Yellow))
        .select(app.tabs.index);
    frame.render_widget(tabs, chunks[0]);
    match app.tabs.index {
        0 => draw_first_tab(frame, app, chunks[1]),
        1 => draw_second_tab(frame, app, chunks[1]),
        _ => {}
    };
}

fn draw_first_tab(frame: &mut Frame, app: &mut App, area: Rect) {
    let chunks =
        Layout::vertical([Constraint::Percentage(20), Constraint::Percentage(60)]).split(area);
    draw_inputs(frame, app, chunks[0]);
    draw_lists(frame, app, chunks[1]);
}

fn draw_inputs(frame: &mut Frame, app: &mut App, area: Rect) {
    let chunks = Layout::horizontal([
        Constraint::Percentage(10),
        Constraint::Percentage(50),
        Constraint::Percentage(40),
    ])
    .margin(1)
    .split(area);

    let number_block = Block::bordered().title("LC Number");
    let name_block = Block::bordered().title("LC Name");
    let type_block = Block::bordered().title("Type");

    frame.render_widget(number_block, chunks[0]);
    frame.render_widget(name_block, chunks[1]);
    frame.render_widget(type_block, chunks[2]);
}

fn draw_lists(frame: &mut Frame, app: &mut App, area: Rect) {
    let chunks =
        Layout::horizontal([Constraint::Percentage(30), Constraint::Percentage(70)]).split(area);

    let placeholder_1 = Block::bordered().title("Todays Problems");
    let placeholder_2 = Block::bordered().title("All Problems");

    frame.render_widget(placeholder_1, chunks[0]);
    frame.render_widget(placeholder_2, chunks[1]);
}

fn draw_second_tab(frame: &mut Frame, app: &mut App, area: Rect) {
    let chunks = Layout::vertical([Constraint::Length(15)]).split(area);
    let placeholder = Paragraph::new("Placeholder for second tab");
    frame.render_widget(placeholder, chunks[0]);
}
