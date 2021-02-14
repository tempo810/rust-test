mod memory;
mod random;
mod ownership;
mod word;
mod user;
mod rectangle;

fn main() {
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2;
    println!("{}", s3);
}
