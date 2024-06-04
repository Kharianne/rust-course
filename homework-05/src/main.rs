mod client;
mod commons;
mod server;

use clap::Parser;
use client::Client;
use server::Server;

#[derive(Parser)]
#[command(about = "Simple client-server application in Rust.")]
enum Commands {
    /// Creates a client and connects to the server
    Client {
        /// Server hostname to connect to
        #[arg(short = 's', long, default_value = "localhost")]
        hostname: String,

        /// Server port to connect to
        #[arg(short, long, default_value_t = 11111)]
        port: u16,
    },
    /// Creates a server and listens for connections
    Server {
        /// Server hostname
        #[arg(short = 's', long, default_value = "localhost")]
        hostname: String,

        /// Server port
        #[arg(short, long, default_value_t = 11111)]
        port: u16,
    },
}

fn main() {
    let command = Commands::parse();

    match command {
        Commands::Server { hostname, port } => {
            println!("Starting server...");
            println!("Listening on {}:{}", hostname, port);
            let mut server = Server::new(format!("{hostname}:{port}"));
            let _ = server.start();
        }
        Commands::Client { hostname, port } => {
            println!("Starting client...");
            println!("Connecting to server at {}:{}", hostname, port);
            let client = Client::new(format!("{hostname}:{port}"));

            // Connect to the server and send messages
            if let Ok(mut client) = client {
                let _ = client.run();
            } else {
                println!("Failed to create client. Is server running?");
            }
        }
    }
}
