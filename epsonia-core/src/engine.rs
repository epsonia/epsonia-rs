use epsonia_checks::checks::check::{CheckData, Checks};

pub struct Engine {
    image_name: String,
    score: i32,
    max_score: i32,
    // Don't worry about this mess.
    checks: Vec<Checks>,
    all_checks: Vec<Checks>,
    completed_checks: Vec<Checks>,
    penalties: Vec<Checks>,
    hidden_penalites: Vec<Checks>,
    hidden_completions: Vec<Checks>,
    checks_len: i32,
}

impl Engine {
    pub fn new(checks: Vec<Checks>, max_score: i32) -> Self {
        let check_amount = checks.len() as i32;

        Engine {
            image_name: String::from(""),
            score: 0,
            max_score,
            checks: checks.clone(),
            all_checks: checks.clone(),
            penalties: Vec::new(),
            completed_checks: Vec::new(),
            hidden_completions: Vec::new(),
            hidden_penalites: Vec::new(),
            checks_len: check_amount,
        }
    }

    pub fn set_scoring_report(&self) {
        // Reference: println!("{}", markdown::to_html("# Hello World"));
        let mut completed_str = String::from("");
        let mut penalty_str = String::from("");

        self.completed_checks.iter().for_each(|check| {
            completed_str.push_str(&format!(
                "<li>{} - {} points</li>",
                check.message(),
                check.points()
            ));
        });

        self.penalties.iter().for_each(|check| {
            penalty_str.push_str(&format!(
                "<li>{} - {} points</li>",
                check.penalty_message(),
                check.points()
            ));
        });

        let report = format!(
            "#{}\n\n<h1>Scoring Report</h1><h2>Score: {}/{} points</h2><h2>Completed Checks</h2><ul>{}</ul><h2>Penalties</h2><ul>{}</ul>",
            self.image_name,
            self.score,
            self.max_score,
            completed_str,
            penalty_str
        );

        std::fs::write("./report.html", report).unwrap();
    }

    pub fn run_engine(&mut self) {
        println!("Running Checks");

        // Run check, if completed, add remove it from the
        for check in &mut self.checks {
            check.run_check();

            if self.completed_checks.contains(&check) && !check.is_completed() {
                self.completed_checks.remove(
                    self.completed_checks
                        .iter()
                        .position(|x| x == check)
                        .unwrap(),
                );
                self.penalties.push(check.clone());
                self.score -= check.points();

                // Penalty notification - Will later be sys notif
                println!("Penalty: {}", check.message());
                continue;
            }

            if *check.is_completed() && !self.completed_checks.contains(check)
                || *check.is_completed() && self.penalties.contains(check)
            {
                self.score += check.points();
                self.completed_checks.push(check.clone());

                if self.penalties.contains(&check) {
                    self.penalties
                        .remove(self.penalties.iter().position(|x| x == check).unwrap());
                }

                // Completion notification - Will later be sys notif
                println!("Completed: {}", check.message());
            }
        }

        self.set_scoring_report();

        self.completed_checks.iter().for_each(|check| {
            println!(
                "Fixed vulnerability - {} - ({}) points",
                check.message(),
                check.points()
            );
        });

        self.penalties.iter().for_each(|p| {
            println!(
                "Penalty - {} - ({}) points",
                p.penalty_message(),
                p.points()
            );
        });
    }
}
