use epsonia_checks::check::{Check, CheckKind};
use notify_rust::Notification;

use crate::config::Config;
pub struct Engine {
    image_name: String,
    score: i32,
    max_score: i32,
    // Don't worry about this mess.
    checks: Vec<Check>,
    completed_checks: Vec<Check>,
    penalties: Vec<Check>,
    hidden_completions: Vec<Check>,
    hidden_penalties: Vec<Check>,
    config: Config,
}

impl Engine {
    pub fn new(checks: Vec<Check>, max_score: i32, config: Config) -> Self {
        Engine {
            image_name: String::from(""),
            score: 0,
            max_score,
            checks,
            penalties: Vec::new(),
            completed_checks: Vec::new(),
            hidden_completions: Vec::new(),
            hidden_penalties: Vec::new(),
            config,
        }
    }

    pub fn set_scoring_report(&self) {
        let mut completed_str = String::from("");
        let mut penalty_str = String::from("");

        self.completed_checks.iter().for_each(|check| {
            completed_str.push_str(&format!("- {} - {} points\n", check.message, check.points));
        });

        self.penalties
            .clone()
            .iter()
            .chain(self.hidden_completions.iter())
            .for_each(|check| {
                penalty_str.push_str(&format!(
                    "- {} -{} points\n",
                    check.penalty_message, check.points
                ));
            });

        let report = format!(
            "# {}\n\n## Scoring Report\n*Score: {}/{} points*\n### Completed Checks:\n{}\n## Penalties:\n{}",
            self.image_name,
            self.score,
            self.max_score,
            completed_str,
            penalty_str
        );

        let auto_refresh_script = format!(
            "<script> function autoRefresh() {{ window.location = window.location.href; }} setInterval('autoRefresh()', {}); </script>",
            self.config.auto_refresh
        );

        std::fs::write(
            format!("{}/report.html", self.config.auto_export_path),
            format!("{} {}", markdown::to_html(&report), auto_refresh_script),
        )
        .unwrap();
    }

    pub fn run_engine(&mut self) {
        // Run check, if completed, add remove it from the
        for check_o in &mut self.checks {
            let check = check_o.clone().run_check();

            // Hidden check
            match &check.kind {
                CheckKind::User {
                    user: _,
                    should_exist: _,
                    does_exist: _,
                    is_primary_user: _,
                }
                | CheckKind::UserIsAdminstrator {
                    user: _,
                    should_be: _,
                    initial_admin: _,
                } => {
                    if self.hidden_completions.contains(check_o) && !check.completed {
                        self.hidden_completions.remove(
                            self.hidden_completions
                                .iter()
                                .position(|x| x == check_o)
                                .unwrap(),
                        );
                        self.hidden_penalties.push(check.clone());
                        self.score -= check.points;

                        Notification::new()
                            .summary("Penalty!")
                            .body(&format!("You lost {} points!", check.points))
                            .icon("dialog-warning")
                            .show()
                            .unwrap();
                    }

                    if (check.completed && !self.hidden_completions.contains(check_o))
                        || (check.completed && self.hidden_penalties.contains(check_o))
                    {
                        self.score += check.points;
                        self.hidden_completions.push(check_o.clone());

                        if self.hidden_penalties.contains(check_o) {
                            self.hidden_penalties.remove(
                                self.hidden_penalties
                                    .iter()
                                    .position(|x| x == check_o)
                                    .unwrap(),
                            );
                        }

                        Notification::new()
                            .summary("Good Job!")
                            .body(&format!("You gained {} points!", check.points))
                            .icon("info")
                            .show()
                            .unwrap();
                    }

                    continue;
                }
                _ => {}
            }

            // Penalty
            if self.completed_checks.contains(check_o) && !check.completed {
                self.completed_checks.remove(
                    self.completed_checks
                        .iter()
                        .position(|x| x == check_o)
                        .unwrap(),
                );
                self.penalties.push(check.clone());
                self.score -= check.points;

                Notification::new()
                    .summary("Penalty!")
                    .body(&format!("You lost {} points!", check.points))
                    .icon("dialog-warning")
                    .show()
                    .unwrap();

                continue;
            }

            if (check.completed && !self.completed_checks.contains(check_o))
                || (check.completed && self.penalties.contains(check_o))
            {
                self.score += check.points;
                self.completed_checks.push(check_o.clone());

                if self.penalties.contains(check_o) {
                    self.penalties
                        .remove(self.penalties.iter().position(|x| x == check_o).unwrap());
                }

                Notification::new()
                    .summary("Good Job!")
                    .body(&format!("You gained {} points!", check.points))
                    .icon("info")
                    .show()
                    .unwrap();
            }
        }

        self.set_scoring_report();

        self.completed_checks.iter().for_each(|check| {
            println!(
                "Fixed vulnerability - {} - ({}) points",
                check.message, check.points
            );
        });

        // "Chains" the hidden penalties and the normal penalties together
        for p in self.penalties.iter().chain(self.hidden_penalties.iter()) {
            println!("Penalty - {} - ({}) points", p.penalty_message, p.points);
        }
    }
}
