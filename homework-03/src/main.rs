use crate::modifications::modifications::{get_modified_input, AVAILABLE_MODIFICATIONS};
use std::env;
use std::error::Error;
mod modifications;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!(
            "Provide the string modification type as an argument:
        {}",
            AVAILABLE_MODIFICATIONS.join(", ")
        );
        std::process::exit(1);
    }

    match get_modified_input(&args[1]) {
        Ok(modified_input) => {
            println!("{}", modified_input);
        }
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    }
    Ok(())
}
