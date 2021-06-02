fn main() {
    let user1 = build_user(String::from("jeremy@nevado.xyz"), String::from("jeremy"));
    let user2 = User {
        email: String::from("another@exmaple.com"),
        username: String::from("anotherusername"),
        active: user1.active,
        sign_in_count: user1.sign_in_count,
    };
    
}

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
