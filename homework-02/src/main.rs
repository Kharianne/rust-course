use slug::slugify;
use std::env;
use std::str::FromStr;

// Defined the enum with allowed values from string modifications
enum StringModification {
    Lowercase,
    Uppercase,
    NoSpaces,
    Slugify,
    Reverse,
    TitleCase,
}

impl FromStr for StringModification {
    // Read the string from arg and convert it to the enum
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "uppercase" => Ok(StringModification::Uppercase),
            "lowercase" => Ok(StringModification::Lowercase),
            "reverse" => Ok(StringModification::Reverse),
            "no-spaces" => Ok(StringModification::NoSpaces),
            "slugify" => Ok(StringModification::Slugify),
            "title-case" => Ok(StringModification::TitleCase),
            _ => Err(format!("Unknown modification: {}", s)),
        }
    }
}

// Define the functions for each modification
fn lowercase(input: String) -> String {
    input.to_lowercase()
}

fn uppercase(input: String) -> String {
    input.to_uppercase()
}

fn no_spaces(input: String) -> String {
    input.replace(" ", "")
}

fn slugify_input(input: String) -> String {
    slugify(input)
}

fn reverse(input: String) -> String {
    input.chars().rev().collect()
}

fn title_case(input: String) -> String {
    input
        .split_whitespace()
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                None => String::new(),
                Some(first_char) => first_char
                    .to_uppercase()
                    .chain(chars.flat_map(|c| c.to_lowercase()))
                    .collect(),
            }
        })
        .collect::<Vec<String>>()
        .join(" ")
}

fn get_modified_input(modification: StringModification, input: String) -> String {
    // Match the enum variant and call the corresponding function
    match modification {
        StringModification::Lowercase => lowercase(input),
        StringModification::Uppercase => uppercase(input),
        StringModification::NoSpaces => no_spaces(input),
        StringModification::Slugify => slugify_input(input),
        StringModification::Reverse => reverse(input),
        StringModification::TitleCase => title_case(input),
    }
}

fn main() {
    // To not repeat the available modifications in the code
    let available_modifications = "lowercase, uppercase, no-spaces, slugify, reverse or title-case";

    // Get the arguments from the command line
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!(
            "Provide the string modification type as an argument: 
        {}",
            available_modifications
        );
        std::process::exit(1);
    }

    // Match the modification from the command line
    let modification = match StringModification::from_str(&args[1]) {
        Ok(modification) => modification,
        Err(e) => {
            eprintln!("Error: {}", e);
            eprintln!("Available modifications: {}", available_modifications);
            std::process::exit(1);
        }
    };

    println!(
        "Selected modification: {}\nProvide the string to modify: ",
        &args[1]
    );
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read user input");
    if input.trim().is_empty() {
        println!("Please enter some text...");
        std::process::exit(1);
    }

    // Print the modified input
    println!(
        "Modified input: {}",
        get_modified_input(modification, input)
    );
}
