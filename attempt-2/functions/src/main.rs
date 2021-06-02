fn main() {
    let n = 12;

    let num = fib(n);

    println!("{}", num)
}

fn fib(n: i32) -> i32 {
    if n == 1 || n == 2{
        return 1
    }
    fib(n - 1) + fib(n - 2)
}
