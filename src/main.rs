mod memory;
mod random;
mod ownership;
mod word;
mod user;
mod rectangle;
mod reader;

fn main() {
    let s = reader::read_from_file("test_v2.txt").unwrap_or_else(|error| error.to_string());
    println!("{}", s);
    reader::read_v2("test_v2.txt");
    let s = reader::read_from_file("test_v2.txt").unwrap_or_else(|error| error.to_string());
    println!("{}", s);
}
