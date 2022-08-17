use std::fs::File;
use std::io::{Error, ErrorKind, Read};

fn main() {
    let filename = "/tmp/customer.json";
    match File::open(filename) {
        Ok(_file) => {
            println!("File Exists");
        }
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create(filename) {
                Ok(_file) => {
                    println!("File created");
                }
                Err(error) => {
                    println!("{:#?}", error);
                }
            },
            _ => {
                println!("{:#?}", error);
            }
        },
    }

    // Another way we can use a function
    let file_name = "/tmp/temporary.json";
    let file_data = read_file(file_name);

    match file_data {
        Ok(data) => {
            println!("{data}");
        }
        Err(_) => {}
    }
}

fn read_file(filename: &str) -> Result<String, Error> {
    let mut file_handle = File::open(filename)?;
    let mut file_data = String::new();
    file_handle.read_to_string(&mut file_data)?;
    Ok(file_data)
}
