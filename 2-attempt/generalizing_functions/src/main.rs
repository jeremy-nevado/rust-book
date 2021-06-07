fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    println!("The largest number is {}", largest(&number_list));

    let number_list = vec![102, 34, 6000, 98, 54, 2, 43, 8];

    println!("The largest number is {}", largest(&number_list));
}

fn largest(lst: &[i32]) -> i32 {
    let mut largest = lst[0];

    for &item in lst {
        if item > largest {
            largest = item;
        }
    }

    largest
}
