fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
fn main() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    match five {
        Some(n) => println!("{}", n),
        None => println!("None")
    }

    match six{
        Some(n) => println!("{}", n),
        None => println!("None")
    }

    match none{
        Some(n) => println!("{}", n),
        None => println!("None")
    }
}
