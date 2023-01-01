struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

fn main() {
    let user = User {
        email: String::from("gainchang620@gmail.com"),
        username: String::from("gainchang"),
        active: true,
        sign_in_count: 1,
    };

    let built_user = build_user(user.email, user.username);

    println!("{}", built_user.email);
}
