use crate::List::{Cons, Nil};

enum List<T> {
    Cons(T, Box<List<T>>),
    Nil,
}
fn main() {
    let list: List<i32> =  Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}
