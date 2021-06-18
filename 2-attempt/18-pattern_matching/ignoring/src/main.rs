fn main() {
    let x = Some(String::from("Hi"));
    let y = String::from("hello");

    match x {
        Some(z) if &z == "Hi" => println!("{}", z),
        Some(ref n) if n == &y => println!("Matched, n = {}", n),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("{}", y);
}
