use std::net::{TcpListener, TcpStream};


pub fn create_server(host: &str, port: &str) -> TcpListener{

    let listener = match TcpListener::bind(format!("{}:{}", host, port)) {
        Ok(listener) => Some(listener),
        Err(e) => {
            println!("Error creating server: {}", e);
            None
        }
    };
    println!("Server created at {}:{}", host, port);
    return listener.unwrap();
}

fn handle_client(stream: TcpStream) {
    println!("New client: {}", stream.peer_addr().unwrap());
}

pub fn listen_for_connections(listener: TcpListener) {
    eprintln!("Server listening for connections");
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
                handle_client(stream);
            }
            Err(e) => {
                println!("Error establishing connection: {}", e);
            }
        }
    }

    eprintln!("Server shutting down")
}