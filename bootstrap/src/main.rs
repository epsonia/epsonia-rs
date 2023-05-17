use std::fs::File;
use std::io::prelude::*;

use epsonia_checks::check::CheckKind;
use epsonia_core::checks_config::get_checks;

fn main() -> std::io::Result<()> {
    let (checks, hidden_checks) = get_checks();
    let mut script: String = String::new();
    let mut out = File::create("out.sh").expect("joe");

    for check in checks {
        println!("{}", check.message);

        match check.kind {
            CheckKind::FileExists {
                file_path,
                should_exist,
            } => {
                // Not checking for should_exist because the user would create that file.
                // Use a hidden penalty instead
                if !should_exist {
                    script.push_str(
                        format!(
                            "# {} - {} points\ntouch {}\n",
                            check.message, check.points, file_path
                        )
                        .as_str(),
                    );
                }
            }
            CheckKind::ServiceUp {
                service_name,
                should_be_up,
            } => {
                if !should_be_up {
                    script.push_str(
                        format!(
                            "# {} - {} points\nsystemctl stop {}",
                            check.message, check.points, service_name
                        )
                        .as_str(),
                    );
                } else {
                    script.push_str(
                        format!(
                            "# {} - {} points\nsystemctl start {} ",
                            check.message, check.points, service_name
                        )
                        .as_str(),
                    );
                }
            }
            CheckKind::UserInGroup {
                user,
                group,
                should_be,
            } => {
                if should_be {
                    script.push_str(
                        format!(
                            "# {} - {} points\ndeluser {} {}",
                            check.message, check.points, user, group
                        )
                        .as_str(),
                    );
                }
            },
            _ => {}
        }
    }

    for check in hidden_checks {
        println!("{}", check.message);
    }

    out.write_all(
        format!(
            r"
#!/bin/bash
#  ______                       _                         
# |  ____|                     (_)                        
# | |__   _ __  ___  ___  _ __  _  __ _                   
# |  __| | '_ \/ __|/ _ \| '_ \| |/ _` |                  
# | |____| |_) \__ \ (_) | | | | | (_| |                  
# |______| .__/|___/\___/|_| |_|_|\__,_|                  
#        | |  _ \            | |     | |                  
#        |_| |_) | ___   ___ | |_ ___| |_ _ __ __ _ _ __  
#          |  _ < / _ \ / _ \| __/ __| __| '__/ _` | '_ \ 
#          | |_) | (_) | (_) | |_\__ \ |_| | | (_| | |_) |
#          |____/ \___/ \___/ \__|___/\__|_|  \__,_| .__/ 
#                                                   | |    
#                                                   |_|   

# Script -

{}
",
            script
        )
        .as_bytes(),
    )?;

    Ok(())
}
