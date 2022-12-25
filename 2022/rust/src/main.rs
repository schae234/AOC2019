use std::env;
use std::io::Error;

mod days;

fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();

    // Create a new args vector for the module main function
    let day_args: Vec<String> = args[2..].iter().cloned().collect();

    match args[1].as_str() as &str {
        "day01" => days::day01::mod_main(day_args)?,
        _ => {
            panic!("{:?} not a valid AOC day", args[1])
        }
    }

    Ok(())
}