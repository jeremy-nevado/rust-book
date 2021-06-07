fn main() {
    let string1 = "abcd";
    let string2 = vec!["a", "b"];

    let string3: Vec<_> = string1.split("").collect();

    let result = longest(&string3, &string2);
    println!("The longest string is {:?}", result);

    let novel = String::from("Call me Ishmael. Some years ago ...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
}

fn longest<'a, T>(x: &'a Vec<T>, y: &'a Vec<T>) -> &'a Vec<T> {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// fn foo<'a> (x: &'a str, y: &'a str) -> str {
// let result = "hello";
// *result
// }

struct ImportantExcerpt<'a> {
    part: &'a str,
}
