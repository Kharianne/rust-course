use crate::commons::MessageType;
use std::collections::HashMap;
use std::io::{Read, Result, Write};
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;

pub struct Server {
    address: String,
    clients: Arc<Mutex<HashMap<String, TcpStream>>>,
}

impl Server {
    pub fn new(address: String) -> Server {
        Server {
            address,
            clients: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn start(&mut self) -> Result<()> {
        let address = self.address.clone();
        let listener = TcpListener::bind(address).unwrap();

        for stream in listener.incoming() {
            let stream = stream.unwrap();
            let addr = stream.peer_addr().unwrap();

            let clients = Arc::clone(&self.clients);
            clients
                .lock()
                .unwrap()
                .insert(addr.to_string(), stream.try_clone().unwrap());

            println!("New client connected: {}", addr);
            thread::spawn(move || Server::handle_client(addr, stream, clients));
        }
        Ok(())
    }

    fn handle_client(
        addr: SocketAddr,
        mut stream: TcpStream,
        clients: Arc<Mutex<HashMap<String, TcpStream>>>,
    ) {
        loop {
            let mut len_bytes = [0u8; 4];
            match stream.read_exact(&mut len_bytes) {
                Ok(_) => {
                    let len = u32::from_be_bytes(len_bytes) as usize;
                    let mut buffer = vec![0u8; len];
                    stream.read_exact(&mut buffer).unwrap();
                    let message = MessageType::deserialize_message(&buffer);
                    println!("{:?}", message);
                    let clients_clone = Arc::clone(&clients);
                    thread::spawn(move || {
                        Server::send_to_other_clients(message, clients_clone, &addr);
                    });

                    //stream.write_all(b"Ok\n").unwrap();
                }
                // Remove stream from clients
                Err(_) => {
                    clients.lock().unwrap().remove(&addr.to_string());
                    println!("Client disconnected: {}", addr);
                    break;
                }
            };
        }
    }

    fn send_to_other_clients(
        message: MessageType,
        clients: Arc<Mutex<HashMap<String, TcpStream>>>,
        sender: &SocketAddr,
    ) {
        let clients = clients.lock().unwrap();
        for (key, mut stream) in clients.iter() {
            println!("Sending message to: {}", key);
            if key == &sender.to_string() {
                continue;
            }
            let message = message.serialize_message();
            let len = message.len() as u32;
            match stream.write_all(&len.to_be_bytes()) {
                Ok(_) => (),
                Err(_e) => (),
            };
            match stream.write_all(message.as_bytes()) {
                Ok(_) => (),
                Err(_e) => (),
            };
        }
    }
}
