use std::fs::{self, File};
use std::io::{Error, ErrorKind, Read};
fn main() {
    a();

    let f = File::open("hello.txt");
    let f = match f {
        Ok(file) => file,
        Err(e) => match e.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {e}"),
            },
            _ => panic!("Problem opening the file: {e}"),
        },
    };
    let f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {error}");
            })
        } else {
            panic!("Problem opening the file: {error}");
        }
    });

    // let f2 = File::open("hello2.txt").expect("Failed to open file hello2");
    let username = read_username_from_file()?;
    println!("username = {username}");
}
fn read_username_from_file() -> Result<String, Error> {
    fs::read_to_string("hello.txt")
}

fn a() {
    b();
}

fn b() {
    c(31);
}

fn c(x: i32) {
    if x == 32 {
        panic!("dont pass 32");
    }
}
