use crate::db::models::LCProblem;
use chrono::{DateTime, Utc};
use rand::{rngs::StdRng, seq::SliceRandom, SeedableRng};
use sha2::{Digest, Sha256};
use std::{collections::HashSet, error::Error};

pub fn format_date(date: DateTime<Utc>) -> String {
    date.format("%Y-%m-%d").to_string()
}

pub fn get_todays_problems(
    all_problems: &Vec<LCProblem>,
) -> Result<Vec<LCProblem>, Box<dyn Error>> {
    let problems = select_random_problems(&all_problems, 10)?;

    let mut problems_with_weights: Vec<(f64, LCProblem)> = vec![];
    let mut unique_ids = HashSet::new();
    let now = Utc::now();

    for problem in problems {
        if !unique_ids.contains(&problem.id) {
            let d_last = now.signed_duration_since(problem.last_practiced).num_days() as f64;
            let n_practiced = problem.times_practiced as f64;
            let weight = d_last / (1.0 + n_practiced);

            problems_with_weights.push((weight, problem.clone()));
            unique_ids.insert(problem.id.clone());
        }
    }

    problems_with_weights.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());

    let todays_problems: Vec<LCProblem> = problems_with_weights
        .into_iter()
        .take(3)
        .map(|(_weight, problem)| problem)
        .collect();

    Ok(todays_problems)
}

fn select_random_problems(
    all_problems: &Vec<LCProblem>,
    limit: usize,
) -> Result<Vec<LCProblem>, Box<dyn Error>> {
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

    let mut problems = all_problems.clone();
    problems.shuffle(&mut rng);

    Ok(problems.into_iter().take(limit).collect())
}
