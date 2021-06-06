use std::fs;
use std::io;

fn main() {}

fn read_username_from_file() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt");
}
