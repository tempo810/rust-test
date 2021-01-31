struct User {
    email: String,
    username: String,
    active: bool,
    sign_in_count: u64,
}

pub fn main() {
    let user1 = User {
        email: String::from("tempo810@gmail.com"),
        username: String::from("tempo"),
        sign_in_count: 1,
        active: true,
    };

    let mut user2 = User { ..user1 };

    user2.email.push_str("test");
    println!("{}", user2.email);
}
