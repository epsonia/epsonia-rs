pub mod util {
    #[derive(PartialEq)]
    pub struct User {
        pub name: String,
    }

    // Make a function that reads /etc/passwd and returns a vec of User
    pub fn get_users() -> Vec<Option<User>> {
        let mut users: Vec<Option<User>> = Vec::new();
        let passwd_content =
            std::fs::read_to_string("/etc/passwd").unwrap_or_else(|_| String::new());

        passwd_content.split('\n').for_each(|line| {
            let user_info: Vec<&str> = line.split(':').collect();
            users.push(Some(User {
                name: user_info[0].to_string(),
            }));
        });

        users
    }
}
