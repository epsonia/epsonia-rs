use epsonia_checks::check::{CheckData, Checks};

use epsonia_checks::file_exists::FileExists;
use serde::{Deserialize, Serialize};

// Note: Completed is a config value.
#[derive(Debug, Serialize, Deserialize)]
pub struct ChecksConfig {
    pub file_exists_check: Vec<FileExists>,
}

pub fn parse_checks_config() -> ChecksConfig {
    let config = std::fs::read_to_string("./config/checks.json").unwrap();
    serde_json::from_str(&config).unwrap()
}

// Run before engine
pub fn get_max_points(checks: &Vec<Checks>) -> i32 {
    let mut max_points = 0;

    for check in checks {
        max_points += check.points();
    }

    max_points
}

pub fn get_checks() -> Vec<Checks> {
    let checks_config = parse_checks_config();

    let mut checks: Vec<Checks> = Vec::new();

    for check in checks_config.file_exists_check {
        checks.push(Checks::FileExists(check));
    }

    checks
}
