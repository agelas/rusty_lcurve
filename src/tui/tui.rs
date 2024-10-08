use crate::db::db;
use crate::tui::tabs::TabsState;
use crate::tui::ui;
use ratatui::{
    backend::CrosstermBackend,
    crossterm::{
        event::{self, Event, KeyCode, KeyEventKind},
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

pub struct App<'a> {
    pub title: &'a str,
    pub should_quit: bool,
    pub tabs: TabsState<'a>,
}

impl<'a> App<'a> {
    pub fn new(title: &'a str) -> Self {
        App {
            title,
            should_quit: false,
            tabs: TabsState::new(vec!["Overview", "Editor"]),
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
                            KeyCode::Left | KeyCode::Char('h') => self.on_left(),
                            KeyCode::Right | KeyCode::Char('l') => self.on_right(),
                            KeyCode::Char('q') => self.should_quit = true,
                            _ => {}
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
    }

    fn on_right(&mut self) {
        self.tabs.next();
    }

    fn on_key(&mut self, c: char) {
        match c {
            'q' => {
                self.should_quit = true;
            }
            _ => {}
        }
    }
}
