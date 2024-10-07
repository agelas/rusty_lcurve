use rusqlite::{Connection, Result};
use std::fs;

pub fn init_db(db_path: &str) -> Result<()> {
    if !fs::metadata(db_path).is_ok() {
        let conn = Connection::open(db_path)?;
        create_table(&conn)?;
    }
    Ok(())
}

fn create_table(conn: &Connection) -> Result<()> {
    conn.execute(
        "
        CREATE TABLE IF NOT EXISTS problems (
            id TEXT PRIMARY KEY,
            lc_number INTEGER NOT NULL,
            problem_name TEXT NOT NULL,
            problem_type TEXT NOT NULL,
            start_date TEXT NOT NULL,
            last_practiced TEXT NOT NULL,
            times_practiced INTEGER NOT NULL
        );
        ",
        [],
    )?;
    Ok(())
}
