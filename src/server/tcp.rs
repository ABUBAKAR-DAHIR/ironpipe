use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use crate::config;

pub fn start(){
    let listener = TcpListener::bind(config::SERVER_ADDRESS).expect("Failed to bind to the address!");

    println!("Server is listening on: {}", config::SERVER_ADDRESS);

    for stream in listener.incoming(){
        match stream {
            Ok(stream) => {
                println!("New client connected ✅");
                handle_client(stream);
            }
            Err(e) => {
                eprintln!("Error occurted in connecting to new client: {}", e);
            }
        }
    }
}

fn handle_client(mut stream: TcpStream){
    let mut buffer = [0; config::BUFFER_SIZE];

    match stream.read(&mut buffer){
        Ok(bytes_read) => {
            println!("Received {} bytes", bytes_read);
            println!("RECEIVED: {}", String::from_utf8_lossy(&buffer[..bytes_read]));

            let response = b"Hello from the TCP server";
            stream.write_all(response).expect("Couldnt send a response to the client!");


        }
        Err(e) => {
            eprintln!("Error occurred in reading the data from the client, {}", e)
        }
    }
}