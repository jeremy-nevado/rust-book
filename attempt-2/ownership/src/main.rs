fn main() {
    let s1 = String::from("hello");
    let world = second_word(&s1);
    println!("{}", world);
}

fn second_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    let mut start = 0;

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' && start == 0{
            start = i;
        } else if item == b' ' && start > 0 {
            return &s[start..i];
        }
    }

    &s[start..]
}
