use crate::tui::tui::{App, AppView, OverviewEditor};
use ratatui::{
    layout::{Constraint, Layout, Position, Rect},
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
        Constraint::Percentage(20),
        Constraint::Percentage(50),
        Constraint::Percentage(30),
    ])
    .split(area);

    let lc_number_paragraph = Paragraph::new(app.lc_number.value())
        .block(Block::bordered().title("LC Number"))
        .style(if matches!(app.app_mode.editor, OverviewEditor::Number) {
            Style::default().fg(Color::Yellow)
        } else {
            Style::default()
        });

    let lc_name_paragraph = Paragraph::new(app.lc_name.value())
        .block(Block::bordered().title("LC Name"))
        .style(if matches!(app.app_mode.editor, OverviewEditor::Name) {
            Style::default().fg(Color::Yellow)
        } else {
            Style::default()
        });

    let lc_type_paragraph = Paragraph::new(app.lc_type.value())
        .block(Block::bordered().title("Type"))
        .style(if matches!(app.app_mode.editor, OverviewEditor::Type) {
            Style::default().fg(Color::Yellow)
        } else {
            Style::default()
        });

    frame.render_widget(lc_number_paragraph, chunks[0]);
    frame.render_widget(lc_name_paragraph, chunks[1]);
    frame.render_widget(lc_type_paragraph, chunks[2]);

    if app.app_mode.view == AppView::Editor {
        match app.app_mode.editor {
            OverviewEditor::Number => {
                frame.set_cursor_position(Position::new(
                    chunks[0].x + app.lc_number.visual_cursor() as u16 + 1,
                    chunks[0].y + 1,
                ));
            }
            OverviewEditor::Name => {
                frame.set_cursor_position(Position::new(
                    chunks[1].x + app.lc_name.visual_cursor() as u16 + 1,
                    chunks[1].y + 1,
                ));
            }
            OverviewEditor::Type => {
                frame.set_cursor_position(Position::new(
                    chunks[2].x + app.lc_type.visual_cursor() as u16 + 1,
                    chunks[2].y + 1,
                ));
            }
        }
    }
}

fn draw_lists(frame: &mut Frame, _app: &mut App, area: Rect) {
    let chunks =
        Layout::horizontal([Constraint::Percentage(30), Constraint::Percentage(70)]).split(area);

    let placeholder_1 = Block::bordered().title("Todays Problems");
    let placeholder_2 = Block::bordered().title("All Problems");

    frame.render_widget(placeholder_1, chunks[0]);
    frame.render_widget(placeholder_2, chunks[1]);
}

fn draw_second_tab(frame: &mut Frame, _app: &mut App, area: Rect) {
    let chunks = Layout::vertical([Constraint::Length(15)]).split(area);
    let placeholder = Paragraph::new("Placeholder for second tab");
    frame.render_widget(placeholder, chunks[0]);
}
