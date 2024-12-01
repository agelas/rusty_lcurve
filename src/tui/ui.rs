use crate::{
    db::{db::get_all_problems, models::LCProblem},
    tui::tui::{App, AppView, ErrorReason, OverviewEditor},
    utils::{format_date, get_todays_problems},
};
use ratatui::{
    layout::{Constraint, Flex, Layout, Margin, Position, Rect},
    style::{Color, Modifier, Style, Stylize},
    text::{self, Span, Text},
    widgets::{
        Block, Borders, Cell, Clear, List, ListItem, Paragraph, Row, Scrollbar,
        ScrollbarOrientation, ScrollbarState, Table, TableState, Tabs, Wrap,
    },
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
    if app.show_error_popup {
        draw_error_popup(frame, &app.error_reason, chunks[1]);
    }
}

fn draw_first_tab(frame: &mut Frame, app: &mut App, area: Rect) {
    let chunks =
        Layout::vertical([Constraint::Percentage(20), Constraint::Percentage(60)]).split(area);
    draw_inputs(frame, app, chunks[0]);
    draw_lists(frame, app, chunks[1]);
}

fn draw_second_tab(frame: &mut Frame, app: &mut App, area: Rect) {
    let chunks = Layout::vertical([Constraint::Length(90)]).split(area);

    draw_editor_table(frame, app, chunks[0]);
    draw_scrollbar(frame, app, chunks[0]);
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
        .style(
            if matches!(app.app_settings.editor, OverviewEditor::Number) {
                Style::default().fg(Color::Yellow)
            } else {
                Style::default()
            },
        );

    let lc_name_paragraph = Paragraph::new(app.lc_name.value())
        .block(Block::bordered().title("LC Name"))
        .style(if matches!(app.app_settings.editor, OverviewEditor::Name) {
            Style::default().fg(Color::Yellow)
        } else {
            Style::default()
        });

    let lc_categories: Vec<ListItem> = app
        .categories
        .items
        .iter()
        .map(|i| ListItem::new(vec![text::Line::from(Span::raw(*i))]))
        .collect();
    let lc_type_list = List::new(lc_categories)
        .block(Block::bordered().title("Categories"))
        .highlight_style(Style::default().add_modifier(Modifier::BOLD))
        .highlight_symbol(">")
        .style(if matches!(app.app_settings.editor, OverviewEditor::Type) {
            Style::default().fg(Color::Yellow)
        } else {
            Style::default()
        });

    frame.render_widget(lc_number_paragraph, chunks[0]);
    frame.render_widget(lc_name_paragraph, chunks[1]);
    frame.render_stateful_widget(lc_type_list, chunks[2], &mut app.categories.state);

    if app.app_settings.view == AppView::Editor {
        match app.app_settings.editor {
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
            _ => {}
        }
    }
}

fn draw_lists(frame: &mut Frame, app: &mut App, area: Rect) {
    let chunks =
        Layout::horizontal([Constraint::Percentage(30), Constraint::Percentage(70)]).split(area);

    let todays_problems = match get_todays_problems(&app.problems) {
        Ok(todays_problems) => todays_problems,
        Err(_) => vec![],
    };

    let todays_problems_list = create_problem_lists("Todays Problems", &todays_problems, true);
    let problem_list = create_problem_lists("All Problems", &app.problems, false);

    frame.render_widget(todays_problems_list, chunks[0]);
    frame.render_widget(problem_list, chunks[1]);
}

fn draw_editor_table(frame: &mut Frame, app: &mut App, area: Rect) {
    let problems = app.problems.clone();

    let headers = Row::new(vec![
        Cell::from("Number"),
        Cell::from("Name"),
        Cell::from("Type"),
        Cell::from("Start"),
        Cell::from("Last Practiced"),
        Cell::from("Times"),
    ])
    .style(Style::default().fg(Color::Yellow));

    let rows: Vec<Row> = problems
        .iter()
        .map(|problem| {
            Row::new(vec![
                Cell::from(problem.lc_number.to_string()),
                Cell::from(problem.problem_name.as_str()),
                Cell::from(problem.problem_type.as_str()),
                Cell::from(format_date(problem.start_date)),
                Cell::from(format_date(problem.last_practiced)),
                Cell::from(problem.times_practiced.to_string()),
            ])
        })
        .collect();

    let widths = [
        Constraint::Length(10),
        Constraint::Length(20),
        Constraint::Length(15),
        Constraint::Length(20),
        Constraint::Length(20),
        Constraint::Length(10),
    ];
    let table = Table::new(rows, widths)
        .header(headers)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Leetcode Problems"),
        )
        .highlight_style(Style::default().fg(Color::Green))
        .highlight_symbol(">");

    frame.render_widget(table, area);
}

fn draw_scrollbar(frame: &mut Frame, app: &mut App, area: Rect) {
    frame.render_stateful_widget(
        Scrollbar::default()
            .orientation(ScrollbarOrientation::VerticalRight)
            .begin_symbol(None)
            .end_symbol(None),
        area.inner(Margin {
            vertical: 1,
            horizontal: 1,
        }),
        &mut app.editor_scroll_state,
    );
}

fn create_problem_lists<'a>(title: &'a str, problems: &'a [LCProblem], truncate: bool) -> List<'a> {
    let problem_items: Vec<ListItem> = problems
        .iter()
        .map(|problem| {
            let mut content = format!(
                "{}: {} ({})",
                problem.lc_number, problem.problem_name, problem.problem_type
            );
            if truncate {
                content.truncate(20);
            }
            ListItem::new(content)
        })
        .collect();

    List::new(problem_items).block(Block::default().borders(Borders::ALL).title(title))
}

fn draw_error_popup(frame: &mut Frame, error_reason: &ErrorReason, area: Rect) {
    let popup_area = popup_area(area, 60, 40);

    let block = Block::bordered().title("Error").on_yellow();

    let error_message = match error_reason {
        ErrorReason::ProblemExists => "The problem already exists in the database. Please enter a unique problem.",
        ErrorReason::CheckingProblemExists => "There was an error checking if the problem already exists. Please try again.",
        ErrorReason::InsertionError => "1. Check your number input is numeric.\n2. Make sure you selected an input category.\nPress Enter to close the popup.",
        ErrorReason::NoError => "",
    };

    let text = Text::from(error_message);

    let paragraph = Paragraph::new(text)
        .block(block)
        .style(Style::default().fg(Color::DarkGray))
        .wrap(Wrap { trim: true });

    frame.render_widget(Clear, area);
    frame.render_widget(paragraph, popup_area);
}

fn popup_area(area: Rect, percent_x: u16, percent_y: u16) -> Rect {
    let vertical = Layout::vertical([Constraint::Percentage(percent_y)]).flex(Flex::Center);
    let horizontal = Layout::horizontal([Constraint::Percentage(percent_x)]).flex(Flex::Center);
    let [area] = vertical.areas(area);
    let [area] = horizontal.areas(area);
    area
}
