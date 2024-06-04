use crate::commons::MessageType;

use std::io::{self, Write};
use std::io::{ErrorKind, Read};
use std::net::TcpStream;
use std::path::Path;
use std::str::FromStr; // Import the FromStr trait

use std::time::{SystemTime, UNIX_EPOCH};
use std::{fs, thread};

pub struct Client {
    stream: TcpStream,
}

impl Client {
    pub fn new(server_addr: String) -> io::Result<Client> {
        let stream = TcpStream::connect(server_addr)?;
        Ok(Client { stream })
    }

    fn send(mut stream: &TcpStream, message: &MessageType) -> io::Result<()> {
        let serialized = message.serialize_message();
        let len = serialized.len() as u32;

        match stream.write_all(&len.to_be_bytes()) {
            Ok(_) => (),
            Err(e) => return Err(e),
        };
        match stream.write_all(serialized.as_bytes()) {
            Ok(_) => (),
            Err(e) => return Err(e),
        };

        Ok(())
    }

    fn receive(mut stream: TcpStream) -> io::Result<()> {
        let mut len_bytes = [0u8; 4];
        loop {
            match stream.read_exact(&mut len_bytes) {
                Ok(_) => {
                    let len = u32::from_be_bytes(len_bytes) as usize;
                    let mut buffer = vec![0u8; len];
                    stream.read_exact(&mut buffer).unwrap();
                    match MessageType::deserialize_message(&buffer) {
                        MessageType::Text(message) => println!("{}", message),
                        MessageType::Image(data) => {
                            Client::receive_file(&MessageType::Image(data)).unwrap();
                        }
                        MessageType::File(name, data) => {
                            Client::receive_file(&MessageType::File(name, data)).unwrap();
                        }
                    }
                }
                Err(_) => {
                    break;
                }
            };
        }
        Err(io::Error::new(io::ErrorKind::Other, "Server disconnected."))
    }

    fn receive_file(message: &MessageType) -> io::Result<()> {
        match message {
            MessageType::File(name, data) => {
                let file_path = Client::ensure_file_path("files", name)?;
                let mut file = std::fs::File::create(&file_path)?;
                file.write_all(data)?;
                println!("Receiving file: {}, saving to: {}", name, file_path);
            }
            MessageType::Image(data) => {
                let file_name = SystemTime::now()
                    .duration_since(SystemTime::UNIX_EPOCH)
                    .unwrap()
                    .as_secs()
                    .to_string();
                let file_path = Client::ensure_file_path("images", &file_name)?;
                let mut file = std::fs::File::create(format!("{}.png", &file_path))?;
                file.write_all(data)?;
                println!("Receiving image: {}, saving to: {}", file_name, file_path);
            }
            _ => (),
        }
        Ok(())
    }

    fn ensure_file_path(dir: &str, filename: &str) -> io::Result<String> {
        let pwd = std::env::current_dir()?;
        let full_path = pwd.join(dir);

        // Ensure the directory exists
        if !full_path.exists() {
            fs::create_dir_all(&full_path)?;
        }

        // Handle filename whether it's a path or a simple filename
        let file_name_path = Path::new(filename);
        let file_name = file_name_path
            .file_name()
            .ok_or(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Invalid filename",
            ))?
            .to_string_lossy()
            .into_owned();

        // Construct the full file path
        let mut file_path = full_path.join(file_name);

        // Append timestamp if the file already exists
        if file_path.exists() {
            let timestamp = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs()
                .to_string();
            let file_extension = file_path
                .extension()
                .map_or_else(|| "".to_string(), |e| format!(".{}", e.to_string_lossy()));

            // Remove the extension to append the timestamp directly to the filename
            let file_stem = file_path
                .file_stem()
                .ok_or(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "Invalid file stem",
                ))?
                .to_string_lossy()
                .into_owned();

            let new_file_name = format!("{}_{}{}", file_stem, timestamp, file_extension);
            file_path.set_file_name(new_file_name);
        }

        Ok(file_path.to_string_lossy().into_owned())
    }

    pub fn run(&mut self) -> io::Result<()> {
        let mut input = String::new();
        let send_stream = self.stream.try_clone().unwrap();
        let recv_stream = self.stream.try_clone().unwrap();

        println!("Enter message to send (or '.quit' to exit):");
        let sender_thread = thread::spawn(move || loop {
            input.clear();

            match io::stdin().read_line(&mut input) {
                Ok(_) => (),
                Err(e) => print!("Error reading input: {}", e),
            };
            input = input.trim().to_string();
            if input.is_empty() {
                continue;
            }
            if input.eq(".quit") {
                print!("Quitting...");
                std::process::exit(0);
            }
            match MessageType::from_str(&input) {
                Ok(message) => {
                    match Client::send(&send_stream, &message) {
                        Ok(_) => (),
                        Err(e) if e.kind() == ErrorKind::BrokenPipe => {
                            println!("Server disconnected. Please wait or press Ctrl+C to exit.");
                            std::process::exit(0);
                        }
                        Err(e) => println!("Error sending message: {}", e),
                    };
                }
                Err(e) => println!("Error while reading message: {}", e),
            }
        });

        let receiver_thread = thread::spawn(move || {
            match Client::receive(recv_stream) {
                Ok(_) => (),
                Err(e) => println!("Error receiving message: {}", e),
            };
        });

        receiver_thread.join().unwrap();
        sender_thread.join().unwrap();

        Ok(())
    }
}
