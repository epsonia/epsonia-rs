use epsonia_util::util::get_users;

#[derive(Clone, PartialEq)]
pub struct HiddenPenalty {
    // Points deducted
    pub deduction: i32,
    // Penalty message
    pub message: String,
    // If the peanlty is active
    pub active: bool,
    pub kind: HiddenPenaltyKind,
}

#[derive(Clone, PartialEq)]
pub enum HiddenPenaltyKind {
    UserMustExist { user: String, should_exist: bool },
}

impl HiddenPenalty {
    pub fn new(deduction: i32, message: String, kind: HiddenPenaltyKind) -> Self {
        HiddenPenalty {
            deduction,
            message,
            active: false,
            kind,
        }
    }

    pub fn run_check(&mut self) -> Self {
        // If true then the penalty is active
        self.active = match &self.kind {
            HiddenPenaltyKind::UserMustExist { user, should_exist } => {
                // Ou is option user
                let user_exists = get_users().iter().any(|ou| {
                    if let Some(u) = ou {
                        u.name == *user
                    } else {
                        false
                    }
                });

                if user_exists == *should_exist {
                    false
                } else {
                    true
                }
            }
        };

        self.clone()
    }
}
