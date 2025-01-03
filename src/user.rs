#[allow(unused)]
mod user {
    use std::collections::HashMap;

    #[derive(PartialEq, Eq, Hash, Debug)]
    struct Account<'a> {
        un: &'a str,
        pwd: &'a str,
    }

    struct UserInfo<'a> {
        name: &'a str,
        age: u8,
    }

    impl<'a> Account<'a> {
        fn new(un: &'a str, pwd: &'a str) -> Self {
            Account { un, pwd }
        }
    }

    impl<'a> UserInfo<'a> {
        fn new(name: &'a str, age: u8) -> Self {
            UserInfo { name, age }
        }
    }

    fn signup<'a>(
        users: &mut HashMap<Account<'a>, UserInfo<'a>>,
        un: &'a str,
        pwd: &'a str,
        name: &'a str,
        age: u8,
    ) {
        let account = Account::new(un, pwd);
        let user_info = UserInfo::new(name, age);
        users.insert(account, user_info);
    }

    fn login<'a>(
        users: &'a HashMap<Account<'a>, UserInfo<'a>>,
        un: &'a str,
        pwd: &'a str,
    ) -> Option<&'a UserInfo<'a>> {
        let account = Account::new(un, pwd);
        users.get(&account)
    }

    fn init() {
        let mut users: HashMap<Account, UserInfo> = HashMap::new();

        // Adding users
        signup(&mut users, "user1", "password1", "Alice", 30);
        signup(&mut users, "user2", "password2", "Bob", 25);

        // Simulate user sign-in
        match login(&users, "user1", "password1") {
            Some(user_info) => println!("Welcome, {}!", user_info.name),
            None => println!("Invalid username or password."),
        }

        // Print all users
        for (account, user_info) in &users {
            println!(
                "Username: {}, Name: {}, Age: {}",
                account.un, user_info.name, user_info.age
            );
        }
    }
}
