fn main() {
    let user1 = build_user(String::from("someone@example.com"), String::from("username123"));
    
    println!("{}", user1.email);
}

fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}
