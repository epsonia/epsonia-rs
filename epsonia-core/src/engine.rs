use std::{cell::RefCell, ops::Deref, rc::Rc};

use epsonia_checks::checks::*;

// Check trait needed? idk
use check::{Check, CheckKind};

// Check imports
use file_exists::FileExists;

pub struct Engine {
    image_name: String,
    score: i32,
    max_score: i32,
    checks: Vec<Rc<RefCell<dyn Check>>>,
    // Don't worry about this mess
    all_checks: Vec<Rc<RefCell<dyn Check>>>,
    completed_checks: Vec<Rc<RefCell<dyn Check>>>,
    hidden_completions: Vec<Rc<RefCell<dyn Check>>>,
    penalties: Vec<Rc<RefCell<dyn Check>>>,
    hidden_penalties: Vec<Rc<RefCell<dyn Check>>>,
    checks_len: i32,
}

impl Engine {
    pub fn new(checks: Vec<Rc<RefCell<dyn Check>>>, max_score: i32) -> Self {
        let check_amount = checks
            .iter()
            .filter(|c| match c.borrow().kind() {
                CheckKind::UserHasToExist => false,
                _ => true,
            })
            .count() as i32;

        Engine {
            image_name: String::from(""),
            score: 0,
            max_score,
            checks: checks.clone(),
            all_checks: checks.clone(),
            completed_checks: vec![],
            hidden_completions: vec![],
            penalties: vec![],
            hidden_penalties: vec![],
            checks_len: check_amount,
        }
    }

    pub fn run_engine(&mut self) {
        println!("Running Checks");

        for check_i in self.all_checks.iter() {
            let mut check = check_i.borrow_mut();
            check.run_check();

            if check.kind().eq(&CheckKind::UserHasToExist) {
                if self.hidden_completions.contains(check_i) && !check.is_completed() {
                    self.hidden_completions.remove(
                        self.hidden_completions
                            .iter()
                            .position(|x| x == check_i)
                            .unwrap(),
                    );
                    self.hidden_penalties.push(check_i.clone());
                    self.score -= check.score();

                    // Will be system notification later, using some lib
                    println!("Removed vuln - {}: {}", check.message(), check.score());
                }

                if (check.is_completed() && !self.hidden_completions.contains(check_i))
                    || (check.is_completed() && self.hidden_penalties.contains(check_i))
                {
                    self.score = check.score();
                    self.hidden_completions.push(check_i.clone());

                    if self.hidden_penalties.contains(check_i) {
                        self.hidden_penalties.remove(
                            self.hidden_penalties
                                .iter()
                                .position(|x| x == check_i)
                                .unwrap(),
                        );
                    }

                    // Will be system notification later, using some lib
                    println!("Fixed vuln - {}: {}", check.message(), check.score());
                }

                continue;
            }
        }
    }
}
