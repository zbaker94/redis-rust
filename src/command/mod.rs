use std::collections::HashMap;

use crate::{redis_data_type::RedisDataType, serialization::deserialize};

#[derive(Debug)]
pub struct CommandReturn {
    pub message: RedisDataType,
    pub hashmap: HashMap<String, RedisDataType>,
}

impl CommandReturn {
    fn new(message: RedisDataType, hashmap: HashMap<String, RedisDataType>) -> CommandReturn {
        CommandReturn {
            message,
            hashmap,
        }
    }
}

fn echo(args: Vec<RedisDataType>) -> CommandReturn {
    let empty_hashmap = HashMap::new();
    if args.len() > 0 {
        // validate that all args are bulk strings or simple strings
        for arg in args.iter() {
            if let RedisDataType::BulkString(_) = arg {
                continue;
            } else if let RedisDataType::SimpleString(_) = arg {
                continue;
            } else {
                let message = RedisDataType::SimpleError(format!("Invalid argument {} passed tp ECHO command", arg).to_string());
                
                return CommandReturn::new(message, empty_hashmap);
            }
        }
        return CommandReturn::new(RedisDataType::Array(args), empty_hashmap);
    }
    let message = RedisDataType::SimpleString("".to_string());
    return CommandReturn::new(message, empty_hashmap);
}

fn ping(args: Vec<RedisDataType>) -> CommandReturn {
    if args.len() > 0 {
        return echo(args);
    }

    return CommandReturn::new(RedisDataType::SimpleString("PONG".to_string()), HashMap::new());
}

fn parse_command(command: &str, args: Vec<RedisDataType>) -> CommandReturn{
    let uppercase_command = command.to_uppercase();
    return match &uppercase_command[..] {
        "PING" => ping(args),
        "ECHO" => echo(args),
        _ => {
            let message = RedisDataType::SimpleError(format!("Unknown command: {}", command).to_string());
            CommandReturn::new(message, HashMap::new())
        }
    }
}

fn array(command_array: Vec<RedisDataType>) -> CommandReturn {
    let command = match &command_array[0] {
        RedisDataType::BulkString(s) => s,
        RedisDataType::SimpleString(s) => s,
        _ => return CommandReturn::new(RedisDataType::SimpleError("Invalid command".to_string()), HashMap::new()),
    };
    let args = command_array[1..].to_vec(); // Convert the slice to a Vec
    return parse_command(&command, args); // Pass the &str instead of a reference
}

pub fn main(message: &str) -> CommandReturn{
    let command = deserialize::main(message);
    println!("Command received: {:?}", command);
    // parse the message
    if let RedisDataType::SimpleString(_) = command {
        let message = "SimpleString commands not supported yet.";
        return CommandReturn::new(RedisDataType::SimpleError(message.to_string()), HashMap::new());
    } else if let RedisDataType::SimpleError(_) = command {
        let message = "SimpleError commands not supported yet.";
        return CommandReturn::new(RedisDataType::SimpleError(message.to_string()), HashMap::new());
    } else if let RedisDataType::Integer(_) = command {
        let message = "Integer commands not supported yet.";
        return CommandReturn::new(RedisDataType::SimpleError(message.to_string()), HashMap::new());
    } else if let RedisDataType::BulkString(_) = command {
        let message = "BulkString commands not supported yet.";
        return CommandReturn::new(RedisDataType::SimpleError(message.to_string()), HashMap::new());
    } else if let RedisDataType::Array(a) = command.clone() {
        let response = array(a);
        eprintln!("Response from array: {:?}", response);
        return response;
    } else {
        let message = format!("Unknown command type {}", command);
        return CommandReturn::new(RedisDataType::SimpleError(message), HashMap::new());
    }
}