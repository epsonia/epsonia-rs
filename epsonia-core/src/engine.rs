use epsonia_checks::{check::Check, hidden_check::HiddenPenalty};
use notify_rust::Notification;

use crate::config::Config;

use std::error::Error;

#[derive(Clone, PartialEq)]
struct Penalty {
    deduction: i32,
    message: String,
    active: bool,
}

impl From<Check> for Penalty {
    fn from(value: Check) -> Self {
        Penalty {
            deduction: value.points,
            message: value.penalty_message,
            active: false,
        }
    }
}

impl From<HiddenPenalty> for Penalty {
    fn from(value: HiddenPenalty) -> Self {
        Penalty {
            deduction: value.deduction,
            message: value.message,
            active: false,
        }
    }
}

pub struct Engine {
    image_name: String,
    score: i32,
    max_score: i32,
    // Don't worry about this mess.
    checks: Vec<Check>,
    completed_checks: Vec<Check>,
    penalties: Vec<Penalty>,
    hidden_penalties: Vec<HiddenPenalty>,
    config: Config,
}

enum EpsoniaNotification {
    Goodjob,
    Penalty,
}

impl EpsoniaNotification {
    pub fn show(&self, points: i32) {
        match self {
            EpsoniaNotification::Goodjob => {
                Notification::new()
                    .summary("Good job!")
                    .body(&format!("You gained {} points!", points))
                    .icon("dialog-information")
                    .show()
                    .unwrap();
            }
            EpsoniaNotification::Penalty => {
                Notification::new()
                    .summary("Penalty!")
                    .body(&format!("You lost {} points!", points))
                    .icon("dialog-warning")
                    .show()
                    .unwrap();
            }
        }
    }
}

impl Engine {
    pub fn new(
        checks: Vec<Check>,
        hidden_penalties: Vec<HiddenPenalty>,
        max_score: i32,
        config: Config,
    ) -> Self {
        Engine {
            image_name: String::from(""),
            score: 0,
            max_score,
            checks,
            hidden_penalties,
            penalties: Vec::new(),
            completed_checks: Vec::new(),
            config,
        }
    }

    pub fn run_engine(&mut self) -> Result<(), Box<dyn Error>> {
        // Hidden checks
        for pen in &mut self.hidden_penalties {
            pen.run_check();

            if pen.active && !self.penalties.contains(&Penalty::from(pen.clone())) {
                self.penalties.push(Penalty::from(pen.clone()));
                self.score -= pen.deduction;

                EpsoniaNotification::Penalty.show(pen.deduction);
                continue;
            }

            if !pen.active && self.penalties.contains(&Penalty::from(pen.clone())) {
                self.penalties.remove(
                    self.penalties
                        .iter()
                        .position(|x| x == &Penalty::from(pen.clone()))
                        .unwrap(),
                );
                self.score += pen.deduction;

                EpsoniaNotification::Goodjob.show(pen.deduction);
            }
        }

        // Run check, if completed, add remove it from the
        for check_o in &self.checks {
            let check = check_o.clone().run_check();

            // Penalty
            if self.completed_checks.contains(check_o) && !check.completed {
                self.completed_checks.remove(
                    self.completed_checks
                        .iter()
                        .position(|x| x == check_o)
                        .unwrap(),
                );
                self.penalties.push(Penalty::from(check.clone()));
                self.score -= check.points;

                EpsoniaNotification::Penalty.show(check.points);
                continue;
            }

            if self.penalties.contains(&Penalty::from(check_o.clone()))
                || !self.completed_checks.contains(check_o) && check.completed
                || (check.completed && self.penalties.contains(&Penalty::from(check_o.clone())))
            {
                self.score += check.points;
                self.completed_checks.push(check_o.clone());

                if self.penalties.contains(&Penalty::from(check_o.clone())) {
                    self.penalties.remove(
                        self.penalties
                            .iter()
                            .position(|x| x == &Penalty::from(check_o.clone()))
                            .unwrap(),
                    );
                }

                EpsoniaNotification::Goodjob.show(check.points);
            }
        }

        self.set_scoring_report()?;

        self.completed_checks.iter().for_each(|check| {
            println!(
                "Fixed vulnerability - {} - ({}) points",
                check.message, check.points
            );
        });

        self.penalties
            .iter()
            .for_each(|p| println!("Penalty - {}  -({}) points", p.message, p.deduction));

        Ok(())
    }

    fn set_scoring_report(&self) -> Result<(), Box<dyn Error>> {
        let mut completed_str = String::from("");
        let mut penalty_str = String::from("");

        self.completed_checks.iter().for_each(|check| {
            completed_str.push_str(&format!("- {} - {} points\n", check.message, check.points));
        });

        self.penalties.clone().iter().for_each(|check| {
            penalty_str.push_str(&format!(
                "- {} -{} points\n",
                check.message, check.deduction
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
        )?;

        Ok(())
    }
}
