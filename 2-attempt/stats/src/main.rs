fn main() {
    let lst: Vec<f32> = vec![12.0, 32.0, 44.0, 15.0, 26.0];

    println!("The mean of lst is: {}", mean(&lst));
}

fn mean(lst: &Vec<f32>) -> f32{
    let mut total: f32 = 0.0;
    let length: f32 = lst.len() as f32;
    for &i in lst.iter() {
        total += i;
    }

    total / length
}
