use std::io;
fn main() {
    let mut temp = String::new();

    io::stdin()
        .read_line(&mut temp)
        .expect("Failed to read line");

    let mut fahr: i32 = match temp.trim().parse() {
        Ok(num) => num,
        Err(_) => println!("Please input a number."),
    };

    let fahr = fahrenheit_celsius(fahr);
    println!("{} Celsius is equal to {} Fahrenheit!", temp, fahr);
}

fn fahrenheit_celsius(fahr: i32) -> i32 {
    fahr * 9 / 5 + 32
}
