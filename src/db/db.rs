use crate::db::models::LCProblem;
use chrono::Utc;
use rand::{rngs::StdRng, seq::SliceRandom, SeedableRng};
use rusqlite::{params, Connection, Result};
use sha2::{Digest, Sha256};
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
    let mut query = conn.prepare("SELECT id, lc_number, problem_name, problem_type, start_date, last_practiced, times_practiced FROM problems")?;
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

pub fn problem_exists(conn: &Connection, lc_number: u32, problem_name: &str) -> Result<bool> {
    let mut stmt =
        conn.prepare("SELECT COUNT(1) FROM problems WHERE lc_number = ?1 OR problem_name = ?2")?;

    let count: u32 = stmt.query_row(params![lc_number, problem_name], |row| row.get(0))?;
    Ok(count > 0)
}

// The necessity of having an LCProblem struct is a bit questionable
pub fn insert_problem(
    conn: &Connection,
    lc_number: u32,
    problem_name: &str,
    problem_type: &str,
) -> Result<()> {
    let lc_problem = LCProblem::new(lc_number, problem_name, problem_type);
    conn.execute(
        "INSERT INTO problems (id, lc_number, problem_name, problem_type, start_date, last_practiced, times_practiced) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
        params![
            lc_problem.id,
            lc_problem.lc_number,
            lc_problem.problem_name,
            lc_problem.problem_type,
            lc_problem.start_date.to_string(),
            lc_problem.last_practiced.to_string(),
            lc_problem.times_practiced,
        ]
    )?;
    Ok(())
}

pub fn select_random_problems(conn: &Connection, limit: usize) -> Result<Vec<LCProblem>> {
    let mut stmt = conn.prepare(
        "SELECT id, lc_number, problem_name, problem_type, start_date, last_practiced, times_practiced
         FROM problems",
    )?;

    let problems_iter = stmt.query_map([], |row| {
        Ok(LCProblem {
            id: row.get(0)?,
            lc_number: row.get(1)?,
            problem_name: row.get(2)?,
            problem_type: row.get(3)?,
            start_date: row.get::<_, String>(4)?.parse().unwrap(),
            last_practiced: row.get::<_, String>(5)?.parse().unwrap(),
            times_practiced: row.get(6)?,
        })
    })?;

    let mut problems = Vec::new();
    for problem in problems_iter {
        problems.push(problem?);
    }

    let now = Utc::now();
    let seed_string = now.format("%Y-%m-%d").to_string();

    let mut hasher = Sha256::new();
    hasher.update(seed_string.as_bytes());
    let seed_hash = hasher.finalize();

    let seed: [u8; 32] = seed_hash
        .as_slice()
        .try_into()
        .expect("Hash output size mismatch");

    let mut rng = StdRng::from_seed(seed);

    problems.shuffle(&mut rng);

    Ok(problems.into_iter().take(limit).collect())
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
