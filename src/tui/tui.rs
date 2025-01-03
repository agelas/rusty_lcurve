use crate::{
    db::{
        db::{get_all_problems, insert_problem, problem_exists, update_problem_as_completed},
        models::LCProblem,
    },
    tui::{
        stateful_list::StatefulList,
        tabs::TabsState,
        ui,
        validation::{number_validator, CATEGORIES},
    },
    utils::get_todays_problems,
};
use ratatui::{
    backend::CrosstermBackend,
    crossterm::{
        event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
        execute,
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    },
    prelude::Backend,
    widgets::{ScrollbarState, TableState},
    Terminal,
};
use rusqlite::Connection;
use std::{
    error::Error,
    io,
    time::{Duration, Instant},
};
use tui_input::{backend::crossterm::EventHandler, Input};

use super::validation::type_validator;

const ITEM_ROW_HEIGHT: usize = 2;

#[derive(PartialEq)]
pub enum AppMode {
    Normal,
    Input,
    Edit,
    Update,
}

#[derive(PartialEq)]
pub enum AppView {
    Overview,
    Editor,
}

#[derive(PartialEq)]
pub enum OverviewEditor {
    Number,
    Name,
    Type,
}

#[derive(PartialEq)]
pub enum ErrorReason {
    NoError,
    ProblemExists,
    CheckingProblemExists,
    InsertionError,
}

pub struct AppSettings {
    pub mode: AppMode,
    pub view: AppView,
    pub editor: OverviewEditor,
}

pub struct App<'a> {
    pub title: &'a str,
    pub problems: Vec<LCProblem>,
    pub todays_problems: Vec<LCProblem>,
    pub should_quit: bool,
    pub tabs: TabsState<'a>,
    pub app_settings: AppSettings,
    pub show_error_popup: bool,
    pub error_reason: ErrorReason,
    pub lc_number: Input,
    pub lc_name: Input,
    pub categories: StatefulList<&'a str>,
    pub db_connection: Connection,
    pub editor_state: TableState,
    pub editor_scroll_state: ScrollbarState,
    pub todays_problem_index: usize, // index of selected problem in Todays Problems (for updates)
}

impl<'a> App<'a> {
    pub fn new(title: &'a str, db_connection: Connection) -> Self {
        let problems = match get_all_problems(&db_connection) {
            Ok(problems) => problems,
            Err(_) => vec![],
        };
        let problems_len = &problems.len();

        let mut scroll_len = 0;
        if *problems_len > 0 {
            scroll_len = (problems_len - 1) * ITEM_ROW_HEIGHT;
        }

        let todays_problems = match get_todays_problems(&problems) {
            Ok(todays_problems) => todays_problems,
            Err(_) => vec![],
        };

        App {
            title,
            problems,
            todays_problems,
            should_quit: false,
            tabs: TabsState::new(vec!["Overview", "Editor"]),
            app_settings: AppSettings {
                mode: AppMode::Normal,
                view: AppView::Overview,
                editor: OverviewEditor::Number,
            },
            show_error_popup: false,
            error_reason: ErrorReason::NoError,
            lc_number: Input::default(),
            lc_name: Input::default(),
            categories: StatefulList::with_items(CATEGORIES.to_vec()),
            db_connection,
            editor_state: TableState::default().with_selected(0),
            editor_scroll_state: ScrollbarState::new(scroll_len),
            todays_problem_index: 0,
        }
    }

    pub fn start_ui(db_connection: Connection) -> Result<(), Box<dyn Error>> {
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen).expect("Failed to execute EnterAlternateScreen");
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        let mut app = App::new("Rusty LCurve", db_connection);
        let app_result = app.run_app(&mut terminal, Duration::from_millis(250));

        disable_raw_mode()?;
        execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
        terminal.show_cursor()?;

        if let Err(err) = app_result {
            println!("{err:?}");
        }

        terminal.clear()?;

        Ok(())
    }

    fn run_app<B: Backend>(
        &mut self,
        terminal: &mut Terminal<B>,
        tick_rate: Duration,
    ) -> io::Result<()> {
        let last_tick = Instant::now();
        loop {
            terminal.draw(|frame| ui::draw(frame, self))?;
            let timeout = tick_rate.saturating_sub(last_tick.elapsed());
            if event::poll(timeout)? {
                if let Event::Key(key) = event::read()? {
                    if key.kind == KeyEventKind::Press {
                        if self.app_settings.mode == AppMode::Normal {
                            match key.code {
                                KeyCode::Left | KeyCode::Char('h') => self.on_left(),
                                KeyCode::Right | KeyCode::Char('l') => self.on_right(),
                                KeyCode::Char('i') => self.app_settings.mode = AppMode::Input,
                                KeyCode::Char('e') => self.app_settings.mode = AppMode::Edit,
                                KeyCode::Char('u') => self.app_settings.mode = AppMode::Update,
                                KeyCode::Char('q') => self.should_quit = true,
                                _ => {}
                            }
                        } else if self.app_settings.mode == AppMode::Input {
                            match key.code {
                                KeyCode::Left => self.switch_editor_left(),
                                KeyCode::Right => self.switch_editor_right(),
                                KeyCode::Up => self.on_up(),
                                KeyCode::Down => self.on_down(),
                                KeyCode::Esc => self.app_settings.mode = AppMode::Normal,
                                KeyCode::Enter => {
                                    if self.show_error_popup {
                                        self.show_error_popup = false;
                                        self.error_reason = ErrorReason::NoError;
                                    } else {
                                        self.on_enter();
                                    }
                                }
                                _ => {
                                    self.handle_input(key);
                                }
                            }
                        } else if self.app_settings.mode == AppMode::Edit {
                            match key.code {
                                KeyCode::Up => self.previous_row(),
                                KeyCode::Down => self.next_row(),
                                KeyCode::Esc => self.app_settings.mode = AppMode::Normal,
                                _ => {}
                            }
                        } else if self.app_settings.mode == AppMode::Update {
                            match key.code {
                                KeyCode::Up => self.todays_problems_index_up(),
                                KeyCode::Down => self.todays_problems_index_down(),
                                KeyCode::Enter => self.mark_problem_as_complete(),
                                KeyCode::Esc => self.app_settings.mode = AppMode::Normal,
                                _ => {}
                            }
                        }
                    }
                }
            }
            if self.should_quit {
                return Ok(());
            }
        }
    }

    fn on_left(&mut self) {
        self.tabs.previous();
        if self.tabs.index == 0 {
            self.app_settings.view = AppView::Overview;
        } else {
            self.app_settings.view = AppView::Editor;
        }
    }

    fn on_right(&mut self) {
        self.tabs.next();
        if self.tabs.index == 0 {
            self.app_settings.view = AppView::Overview;
        } else {
            self.app_settings.view = AppView::Editor;
        }
    }

    fn on_up(&mut self) {
        if self.app_settings.editor == OverviewEditor::Type {
            self.categories.previous();
        }
    }

    fn on_down(&mut self) {
        if self.app_settings.editor == OverviewEditor::Type {
            self.categories.next();
        }
    }

    pub fn next_row(&mut self) {
        if self.tabs.index == 1 {
            let i = match self.editor_state.selected() {
                Some(i) => {
                    if i >= self.problems.len() - 1 {
                        0
                    } else {
                        i + 1
                    }
                }
                None => 0,
            };
            self.editor_state.select(Some(i));
            self.editor_scroll_state = self.editor_scroll_state.position(i * ITEM_ROW_HEIGHT);
        }
    }

    pub fn previous_row(&mut self) {
        if self.tabs.index == 1 {
            let i = match self.editor_state.selected() {
                Some(i) => {
                    if i == 0 {
                        self.problems.len() - 1
                    } else {
                        i - 1
                    }
                }
                None => 0,
            };
            self.editor_state.select(Some(i));
            self.editor_scroll_state = self.editor_scroll_state.position(i * ITEM_ROW_HEIGHT);
        }
    }

    fn on_enter(&mut self) {
        let number_valid = number_validator(&self.lc_number);
        let name_valid = !self.lc_name.value().is_empty();
        let category_valid = if let Some(selected_index) = self.categories.state.selected() {
            type_validator(CATEGORIES[selected_index])
        } else {
            false
        };

        if !number_valid || !name_valid || !category_valid {
            self.show_error_popup = !number_valid || !name_valid || !category_valid;
            self.error_reason = ErrorReason::InsertionError;
            return;
        }

        let lc_number = self
            .lc_number
            .value()
            .parse::<u32>()
            .expect("Invalid LC number");
        let problem_name = self.lc_name.value();
        if !self.show_error_popup {
            match problem_exists(&self.db_connection, lc_number, problem_name) {
                Ok(true) => {
                    self.show_error_popup = true;
                    self.error_reason = ErrorReason::ProblemExists;
                }
                Ok(false) => {
                    let problem_type = CATEGORIES[self.categories.state.selected().unwrap()];
                    if let Err(_err) =
                        insert_problem(&self.db_connection, lc_number, &problem_name, problem_type)
                    {
                        self.show_error_popup = true;
                        self.error_reason = ErrorReason::InsertionError;
                    } else {
                        self.lc_number.reset();
                        self.lc_name.reset();
                        self.categories.state.select(None);
                        self.problems = match get_all_problems(&self.db_connection) {
                            Ok(problems) => problems,
                            Err(_) => vec![],
                        }
                    }
                }
                Err(_err) => {
                    self.show_error_popup = true;
                    self.error_reason = ErrorReason::CheckingProblemExists;
                }
            }
        }
    }

    fn switch_editor_left(&mut self) {
        match self.app_settings.editor {
            OverviewEditor::Name => self.app_settings.editor = OverviewEditor::Number,
            OverviewEditor::Type => self.app_settings.editor = OverviewEditor::Name,
            _ => {}
        }
    }

    fn switch_editor_right(&mut self) {
        match self.app_settings.editor {
            OverviewEditor::Number => self.app_settings.editor = OverviewEditor::Name,
            OverviewEditor::Name => self.app_settings.editor = OverviewEditor::Type,
            _ => {}
        }
    }

    fn handle_input(&mut self, key: KeyEvent) {
        match self.app_settings.editor {
            OverviewEditor::Number => {
                self.lc_number.handle_event(&Event::Key(key));
            }
            OverviewEditor::Name => {
                self.lc_name.handle_event(&Event::Key(key));
            }
            _ => {}
        }
    }

    fn todays_problems_index_up(&mut self) {
        if self.todays_problem_index > 0 {
            self.todays_problem_index -= 1;
        } else {
            self.todays_problem_index = 2;
        }
    }

    fn todays_problems_index_down(&mut self) {
        if self.todays_problem_index < 2 {
            self.todays_problem_index += 1;
        } else {
            self.todays_problem_index = 0;
        }
    }

    fn mark_problem_as_complete(&mut self) {
        if let Some(problem) = get_todays_problems(&self.problems)
            .unwrap()
            .get(self.todays_problem_index)
        {
            match update_problem_as_completed(&self.db_connection, &problem.id) {
                Ok(_) => {
                    self.problems = match get_all_problems(&self.db_connection) {
                        Ok(problems) => problems,
                        Err(_) => vec![],
                    }
                }
                Err(_) => {}
            }
        }
    }
}
