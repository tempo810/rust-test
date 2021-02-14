use std::fs::File;
use std::io::{Error, Read};
use std::io::ErrorKind;

pub fn read(file_name: &str) {
    let f = File::open(&file_name);
    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create(&file_name) {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error)
            }
        }
    };
}


pub fn read_v2(file_name: &str) {
    let f = File::open(file_name).unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create(file_name).unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
}

pub fn read_from_file(file_name: &str) -> Result<String, Error> {
    let mut result = String::new();
    File::open(file_name)?.read_to_string(&mut result)?;
    Ok(result)
}
