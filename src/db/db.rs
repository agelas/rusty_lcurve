use crate::db::models::LCProblem;
use rusqlite::{Connection, Result};
use std::fs;

pub fn init_db(db_path: &str) -> Result<()> {
    if !fs::metadata(db_path).is_ok() {
        let conn = Connection::open(db_path)?;
        create_table(&conn)?;
    }
    Ok(())
}

pub fn get_connection(db_path: &str) -> Result<Connection> {
    Connection::open(db_path)
}

pub fn get_all_problems(conn: &Connection) -> Result<Vec<LCProblem>> {
    let mut query = conn.prepare("SELECT id, lc_number, problem_name, problem_type, start_date, last_practiced, times_practed FROM problems")?;
    let problems = query
        .query_map([], |row| {
            Ok(LCProblem {
                id: row.get(0)?,
                lc_number: row.get(1)?,
                problem_name: row.get(2)?,
                problem_type: row.get(3)?,
                start_date: row.get::<_, String>(4)?.parse().unwrap(),
                last_practiced: row.get::<_, String>(5)?.parse().unwrap(),
                times_practiced: row.get(6)?,
            })
        })?
        .collect::<Result<Vec<_>, _>>()?;
    Ok(problems)
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
