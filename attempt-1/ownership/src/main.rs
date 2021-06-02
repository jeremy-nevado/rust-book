fn main() {
    let s = String::from("hello world I am jeremy");

    let hello = first_word(&s);
    let world = second_word(&s);

    println!("{}, {}", hello, world);
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}

fn second_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    let mut j = 0;
    let mut counter = 0;
    
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            if counter == 1 {
                return &s[j..i];
            } else {
                j = i;
                counter = counter + 1;
            } 
        }
    }
    return &s[j..]
}
