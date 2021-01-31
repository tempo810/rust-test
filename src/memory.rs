use std::io;
use std::process;

pub fn string_test() {
    println!("start with pid {}", process::id());
    let mut input= String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    {
        println!("before declare variable");
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let mut s = String::from("not hello");
        println!("{}", s);
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        s.push_str(", world!");
        println!("{}", s);
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
    }

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
}

