use crate::db::db;
use crate::tui::stateful_list::StatefulList;
use crate::tui::tabs::TabsState;
use crate::tui::ui;
use crate::tui::validation::CATEGORIES;
use ratatui::{
    backend::CrosstermBackend,
    crossterm::{
        event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
        execute,
        terminal::{disable_raw_mode, enable_raw_mode},
    },
    prelude::Backend,
    Terminal,
};
use std::{
    error::Error,
    io,
    time::{Duration, Instant},
};
use tui_input::{backend::crossterm::EventHandler, Input};

#[derive(PartialEq)]
pub enum AppMode {
    Normal,
    Input,
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

pub struct AppSettings {
    pub mode: AppMode,
    pub view: AppView,
    pub editor: OverviewEditor,
}

pub struct App<'a> {
    pub title: &'a str,
    pub should_quit: bool,
    pub tabs: TabsState<'a>,
    pub app_settings: AppSettings,
    pub lc_number: Input,
    pub lc_name: Input,
    pub categories: StatefulList<&'a str>,
}

impl<'a> App<'a> {
    pub fn new(title: &'a str) -> Self {
        App {
            title,
            should_quit: false,
            tabs: TabsState::new(vec!["Overview", "Editor"]),
            app_settings: AppSettings {
                mode: AppMode::Normal,
                view: AppView::Overview,
                editor: OverviewEditor::Number,
            },
            lc_number: Input::default(),
            lc_name: Input::default(),
            categories: StatefulList::with_items(CATEGORIES.to_vec()),
        }
    }

    // Remember to put db_path: &str as a param later
    pub fn start_ui() -> Result<(), Box<dyn Error>> {
        enable_raw_mode()?;
        let stdout = io::stdout();
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        let mut app = App::new("Rusty LCurve");
        let app_result = app.run_app(&mut terminal, Duration::from_millis(250));

        disable_raw_mode()?;
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
                                KeyCode::Char('e') => self.app_settings.mode = AppMode::Input,
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
                                    // need to validate all inputs before submitting and clearing all input fields.
                                }
                                _ => {
                                    self.handle_input(key);
                                }
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
        if (self.app_settings.editor == OverviewEditor::Type) {
            self.categories.previous();
        }
    }

    fn on_down(&mut self) {
        if (self.app_settings.editor == OverviewEditor::Type) {
            self.categories.next();
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
}
