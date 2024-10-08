use crate::db::db;
use crate::tui::tabs::TabsState;
use crate::tui::ui;
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
pub enum AppView {
    Overview,
    Editor,
}

pub enum OverviewEditor {
    Number,
    Name,
    Type,
}

pub struct AppMode {
    pub view: AppView,
    pub editor: OverviewEditor,
}

pub struct App<'a> {
    pub title: &'a str,
    pub should_quit: bool,
    pub tabs: TabsState<'a>,
    pub app_mode: AppMode,
    pub lc_number: Input,
    pub lc_name: Input,
    pub lc_type: Input,
}

impl<'a> App<'a> {
    pub fn new(title: &'a str) -> Self {
        App {
            title,
            should_quit: false,
            tabs: TabsState::new(vec!["Overview", "Editor"]),
            app_mode: AppMode {
                view: AppView::Overview,
                editor: OverviewEditor::Number,
            },
            lc_number: Input::default(),
            lc_name: Input::default(),
            lc_type: Input::default(),
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
                        match key.code {
                            KeyCode::Char('1') | KeyCode::Char('2') | KeyCode::Char('3') => {
                                if let KeyCode::Char(c) = key.code {
                                    self.on_key(c);
                                }
                            }
                            KeyCode::Enter => {
                                // need to validate all inputs before submitting and clearing all input fields.
                            }
                            KeyCode::Left | KeyCode::Char('h') => self.on_left(),
                            KeyCode::Right | KeyCode::Char('l') => self.on_right(),
                            KeyCode::Char('q') => self.should_quit = true,
                            _ => {
                                self.handle_input(key);
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
            self.app_mode.view = AppView::Overview;
        } else {
            self.app_mode.view = AppView::Editor;
        }
    }

    fn on_right(&mut self) {
        self.tabs.next();
        if self.tabs.index == 0 {
            self.app_mode.view = AppView::Overview;
        } else {
            self.app_mode.view = AppView::Editor;
        }
    }

    fn on_key(&mut self, c: char) {
        match c {
            'q' => {
                self.should_quit = true;
            }
            '1' => {
                self.app_mode.editor = OverviewEditor::Number;
            }
            '2' => {
                self.app_mode.editor = OverviewEditor::Name;
            }
            '3' => {
                self.app_mode.editor = OverviewEditor::Type;
            }
            _ => {}
        }
    }

    fn handle_input(&mut self, key: KeyEvent) {
        match self.app_mode.editor {
            OverviewEditor::Number => {
                self.lc_number.handle_event(&Event::Key(key));
            }
            OverviewEditor::Name => {
                self.lc_number.handle_event(&Event::Key(key));
            }
            OverviewEditor::Type => {
                self.lc_type.handle_event(&Event::Key(key));
            }
        }
    }
}
