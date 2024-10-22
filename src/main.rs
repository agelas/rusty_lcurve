mod db;
mod tui;
mod utils;

use db::db::{get_connection, init_db};

use crate::tui::tui::App;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let db_path = "rusty_l_db";
    if let Err(err) = init_db(db_path) {
        println!("Error initializing database: {:?}", err);
    }

    let db_connection = get_connection(db_path).unwrap();
    let rusty_lcurve_tui = App::start_ui(db_connection);
    Ok(())
}
