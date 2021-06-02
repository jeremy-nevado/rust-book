fn main() {
    let coin = Coin::Penny;
    let coin2 = Coin::Dime;

    println!("coin is worth {}.", value_in_cents(coin));
    println!("coin2 is worth {}.", value_in_cents(coin2));
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,

    }
}
