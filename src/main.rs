mod db;
mod tui;
mod utils;

use crate::tui::tui::App;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let rusty_lcurve_tui = App::start_ui();
    Ok(())
}
