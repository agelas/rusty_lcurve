use crate::db::db;
use ratatui::{backend::CrosstermBackend, Terminal};
use std::error::Error;
use std::io;

pub fn start_ui(db_path: &str) -> Result<(), Box<dyn Error>> {
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Actual UI logic here

    terminal.clear()?;

    Ok(())
}
