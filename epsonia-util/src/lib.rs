#[derive(PartialEq)]
pub struct User {
    pub name: String,
    pub uid: u32,
    pub gid: u32,
    pub home: String,
    pub shell: String,
}

// Make a function that reads /etc/passwd and returns a vec of User
pub fn get_users() -> Vec<User> {
    let mut users: Vec<User> = Vec::new();
    let passwd_content = std::fs::read_to_string("/etc/passwd").unwrap_or_else(|_| String::new());

    passwd_content.split("\n").for_each(|line| {
        let user_info: Vec<&str> = line.split(":").collect();
        users.push(User {
            name: user_info[0].to_string(),
            uid: user_info[2].parse::<u32>().unwrap(),
            gid: user_info[3].parse::<u32>().unwrap(),
            home: user_info[5].to_string(),
            shell: user_info[6].to_string(),
        });
    });

    users
}
