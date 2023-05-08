use epsonia_checks::check::{CheckData, Checks};

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
            let check = match check {
                Checks::FileExists(check) => check,
            };
            completed_str.push_str(&format!(
                "<li>{} - {} points</li>",
                check.message, check.points
            ));
        });

        self.penalties.iter().for_each(|check| {
            let check = match check {
                Checks::FileExists(check) => check,
            };
            penalty_str.push_str(&format!(
                "<li>{} - {} points</li>",
                check.penalty_message, check.points
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
        for check_o in &mut self.checks {
            let check = match check_o {
                Checks::FileExists(check) => check,
            };

            check_o.run_check();

            if self.completed_checks.contains(check_o) && !check.completed {
                self.completed_checks.remove(
                    self.completed_checks
                        .iter()
                        .position(|x| x == check_o)
                        .unwrap(),
                );
                self.penalties
                    .push(epsonia_checks::check::Checks::FileExists(check.clone()));
                self.score -= check.points;

                // Penalty notification - Will later be sys notif
                println!("Penalty: {}", check.message);
                continue;
            }

            if check.completed && !self.completed_checks.contains(check_o)
                || check.completed && self.penalties.contains(check_o)
            {
                self.score += check.points;
                self.completed_checks.push(check_o.clone());

                if self.penalties.contains(&check_o) {
                    self.penalties
                        .remove(self.penalties.iter().position(|x| x == check_o).unwrap());
                }

                // Completion notification - Will later be sys notif
                println!("Completed: {}", check.message);
            }
        }

        self.set_scoring_report();

        self.completed_checks.iter().for_each(|check| {
            let check = match check {
                Checks::FileExists(check) => check,
            };
            println!(
                "Fixed vulnerability - {} - ({}) points",
                check.message, check.points
            );
        });

        self.penalties.iter().for_each(|p| {
            let p = match p {
                Checks::FileExists(p) => p,
            };
            println!("Penalty - {} - ({}) points", p.penalty_message, p.points);
        });
    }
}
