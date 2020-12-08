use std::{io, env};
use std::fs::File;
use std::io::Read;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file: Result<String, io::Error> = match args.len() {
        1 => {
            println!("Arguments are required. Pick 1 to read from commandline, 2 to read from file");
            Err(io::Error)
        },
        // one arg
        2 => {
            println!("One argument detected, reading from file: {}", &args[1]);
            load_file(&args[1]).expect("File failed to load")
        },
        _ => {
            println!("Too many arguments");
            Err(io::Error)
        },
    };

}

fn load_file(filename: &str) -> Result<String, io::Error> {
    let mut contents = String::new();
    File::open(filename)?.read_to_string(&mut contents)?;
    Ok(contents)
}