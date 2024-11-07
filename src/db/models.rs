use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, PartialEq, Clone)]
pub struct LCProblem {
    pub id: String,
    pub lc_number: u32,
    pub problem_name: String,
    pub problem_type: String,
    pub start_date: DateTime<Utc>,
    pub last_practiced: DateTime<Utc>,
    pub times_practiced: u32,
}

impl LCProblem {
    pub fn new(lc_number: u32, problem_name: &str, problem_type: &str) -> Self {
        let current_time = Utc::now();

        Self {
            id: Uuid::new_v4().to_string(),
            lc_number,
            problem_name: problem_name.to_string(),
            problem_type: problem_type.to_string(),
            start_date: current_time,
            last_practiced: current_time,
            times_practiced: 0,
        }
    }
}
