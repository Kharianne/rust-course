use std::io;

fn main() {
    let mut input = String::new();

    loop {
        println!("Enter your name: ");
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read user input");
        if input.trim().is_empty() {
            println!("Please enter some name...");
            continue;
        } else {
            println!("Hello, {}! Nice to meet you!", input.trim());
            break;
        }
    }
}
