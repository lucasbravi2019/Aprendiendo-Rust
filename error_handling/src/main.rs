use std::fs::File;
use std::io::{self, ErrorKind, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let mut username = String::new();

    File::open("file.txt")?.read_to_string(&mut username)?;

    Ok(username)   
}

fn main() {
    // let file = File::open("file.txt");

    // let file = match file {
    //     Ok(element) => element,
    //     Err(error) => panic!("{error:?}"),
    // };

    // println!("{file:?}");

    // let file = File::open("file.txt");
    // let file = match file {
    //     Ok(element) => element,
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound => match File::create("file.txt") {
    //             Ok(file_created) => file_created,
    //             Err(error) => panic!("Error while creating file: {error:?}"),
    //         },
    //         other_error => {
    //             panic!("Error opening the file");
    //         }
    //     }
    // };

    // println!("{file:?}");

    let username = read_username_from_file().unwrap();
    println!("Username: {username}");
}
