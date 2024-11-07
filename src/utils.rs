use crate::db::db::select_random_problems;
use crate::db::models::LCProblem;
use chrono::{DateTime, Utc};
use rusqlite::Connection;
use std::error::Error;

pub fn format_date(date: DateTime<Utc>) -> String {
    date.format("%Y-%m-%d %H:%M:%S").to_string()
}

pub fn get_todays_problems(conn: &Connection) -> Result<Vec<LCProblem>, Box<dyn Error>> {
    let problems = select_random_problems(conn, 10)?;

    let mut problems_with_weights: Vec<(f64, LCProblem)> = vec![];
    let now = Utc::now();

    for problem in problems {
        let d_last = now.signed_duration_since(problem.last_practiced).num_days() as f64;
        let n_practiced = problem.times_practiced as f64;
        let weight = d_last / (1.0 + n_practiced);
        problems_with_weights.push((weight, problem));
    }

    problems_with_weights.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());

    let todays_problems: Vec<LCProblem> = problems_with_weights
        .into_iter()
        .take(3)
        .map(|(_weight, problem)| problem)
        .collect();

    Ok(todays_problems)
}
