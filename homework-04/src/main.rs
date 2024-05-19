use crate::operations::operations::{get_modified_input, StringOperation, AVAILABLE_OPERATIONS};
use std::env;
use std::io;
use std::thread;
mod operations;
use std::io::BufRead;
use std::str::FromStr;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!(
            "Entrering interactive mode...\n\tExpected format: <command> <input>\n\tAvailable commands: {}\nTo exit, press Ctrl+C\n",
            AVAILABLE_OPERATIONS.join(", ")
        );
        let (tx, rx) = std::sync::mpsc::channel();

        let input_thread = thread::spawn(move || {
            let stdin = io::stdin();
            for line in stdin.lock().lines() {
                let line = line.unwrap();
                // Handle the rest of the input as one string
                // slugify Hello World --> hello-world
                let mut parts = line.splitn(2, char::is_whitespace);
                let command = parts.next().unwrap_or("");
                let input = parts.next().unwrap_or("").to_string();

                if !command.is_empty() {
                    if input.trim().is_empty() {
                        eprintln!("Input for command is missing. Expected: <command> <input>");
                        continue;
                    }
                    let command = StringOperation::from_str(command);
                    match command {
                        Ok(cmd) => tx.send((cmd, input)).unwrap(),
                        Err(err) => eprintln!("Erro: {}", err),
                    }
                } else {
                    eprintln!("Invalid input format. Expected: <command> <input>");
                }
            }
        });

        let processing_thread = thread::spawn(move || {
            while let Ok((command, input)) = rx.recv() {
                match get_modified_input(Some(&input), command) {
                    Ok(modified_input) => {
                        println!("{}", modified_input);
                    }
                    Err(e) => {
                        eprintln!("{}", e);
                    }
                }
            }
        });

        input_thread.join().unwrap();
        processing_thread.join().unwrap();
    } else {
        match get_modified_input(None, StringOperation::from_str(&args[1]).unwrap()) {
            Ok(modified_input) => {
                println!("{}", modified_input);
            }
            Err(e) => {
                eprintln!("{}", e);
                std::process::exit(1);
            }
        }
    }
}
