fn main() {
    let email = String::from("12345@gmail.com");
    let name = String::from("12345@gmail.com");

    let user1 = build_user(email, name);
    let user2 = User {
        email: String::from("12"),
        ..user1
    };
    println!("{:?}", user2);
}

#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
