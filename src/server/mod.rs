use std::{collections::HashMap, io::{Read, Write}, net::{TcpListener, TcpStream}, thread};

use crate::{command, redis_data_type::RedisDataType, serialization::serialize}; 


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
fn handle_client(mut stream: TcpStream) -> HashMap<String, RedisDataType>{
    let mut buffer = [0; 512]; // A buffer to hold the incoming data
    let mut message = String::new(); // String to accumulate the message

    loop {
        match stream.read(&mut buffer) {
            Ok(0) => {
                // Connection was closed
                break;
            }
            Ok(n) => {
                // Convert the buffer into a String and append to the message
                if let Ok(text) = std::str::from_utf8(&buffer[..n]) {
                    message.push_str(text);
                    eprintln!("Received message: {}", message);
                }
                // terminate the loop if the message ends with a newline
                if message.ends_with('\n') {
                    eprintln!("Received message: {}", message);
                    break;
                }
            }
            Err(e) => {
                eprintln!("Failed to read from stream: {}", e);
                return HashMap::from([("ERROR".to_string(), RedisDataType::SimpleError("Failed to read from stream".to_string()))]);
            }
        }
    }

    let response = command::main(&message); // TODO return the atomic update to the global hashmap if relevant
    eprintln!("Response: {:?}", response);
    let response_string = serialize::main(response.message);
    eprintln!("Response string: {:?}", response_string);

    stream.write_all(response_string.as_bytes()).unwrap();

    return response.hashmap;
}

pub fn listen_for_connections(listener: TcpListener) {
    eprintln!("Server listening for connections");
    for stream in listener.incoming() {
        let mut mutations = HashMap::new();
        match stream {
            Ok(stream) => {
                thread::spawn(move || {
                    println!("New connection: {}", stream.peer_addr().unwrap());
                    let mutation = handle_client(stream); 
                    mutations.extend(mutation);
                });
                // TODO apply the atomic updates to the global hashmap
            }
            Err(e) => {
                println!("Error establishing connection: {}", e);
            }
        }
    }

    eprintln!("Server shutting down")
}