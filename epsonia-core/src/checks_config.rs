use epsonia_checks::check::{Check, CheckKind};
use serde::{Deserialize, Serialize};

use crate::models::{FileContainsContent, FileExists, FileLineContains, ServiceUp};

// Note: Completed is a config value.
#[derive(Debug, Serialize, Deserialize)]
pub struct ChecksConfig {
    pub file_exists: Option<Vec<FileExists>>,
    pub file_line_contains: Option<Vec<FileLineContains>>,
    pub file_contains_content: Option<Vec<FileContainsContent>>,
    pub service_up: Option<Vec<ServiceUp>>,
}

pub fn parse_checks_config() -> ChecksConfig {
    let config = std::fs::read_to_string("./config/checks.json").unwrap();
    serde_json::from_str(&config).unwrap()
}

// Run before engine
pub fn get_max_points(checks: &Vec<Check>) -> i32 {
    let mut max_points = 0;

    for check in checks {
        max_points += check.points;
    }

    max_points
}

pub fn get_checks() -> Vec<Check> {
    let checks_config = parse_checks_config();

    let mut checks: Vec<Check> = Vec::new();

    if let Some(file_exists) = checks_config.file_exists {
        for check in file_exists {
            checks.push(Check::new(
                check.points,
                check.message,
                check.penalty_message,
                false,
                CheckKind::FileExists {
                    file_path: check.file_path,
                    should_exist: check.should_exist,
                },
            ));
        }
    }

    if let Some(file_line_contains) = checks_config.file_line_contains {
        for check in file_line_contains {
            checks.push(Check::new(
                check.points,
                check.message,
                check.penalty_message,
                false,
                CheckKind::FileLineContains {
                    file_path: check.file_path,
                    line: check.line,
                    line_content: check.line_content,
                    should_contain: check.should_contain,
                },
            ));
        }
    }

    if let Some(file_contains_content) = checks_config.file_contains_content {
        for check in file_contains_content {
            checks.push(Check::new(
                check.points,
                check.message,
                check.penalty_message,
                false,
                CheckKind::FileContainsContent {
                    file_path: check.file_path,
                    content: check.content,
                    whitespace_matters: check.whitespace_matters,
                    should_contain: check.should_contain,
                },
            ));
        }
    }

    if let Some(service_up) = checks_config.service_up {
        for check in service_up {
            checks.push(Check::new(
                check.points,
                check.message,
                check.penalty_message,
                false,
                CheckKind::ServiceUp {
                    service_name: check.service_name,
                    should_be_up: check.should_be_up,
                },
            ))
        }
    }
    checks
}
