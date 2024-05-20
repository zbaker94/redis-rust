use crate::{redis_data_type::RedisDataType, serialization::deserialize};

fn echo(args: Vec<RedisDataType>) -> RedisDataType {
    if args.len() > 0 {
        // validate that all args are bulk strings or simple strings
        for arg in args.iter() {
            if let RedisDataType::BulkString(_) = arg {
                continue;
            } else if let RedisDataType::SimpleString(_) = arg {
                continue;
            } else {
                return RedisDataType::SimpleError(format!("Invalid argument {} passed tp PING command", arg).to_string());
            }
        }
        let strings: Vec<String> = args.iter().map(|arg| arg.to_string()).collect();
        return RedisDataType::BulkString(strings.join(""));
    }

    return RedisDataType::SimpleString("".to_string());
}

fn ping(args: Vec<RedisDataType>) -> RedisDataType {
    if args.len() > 0 {
        return echo(args);
    }

    return RedisDataType::SimpleString("PONG".to_string());
}

fn parse_command(command: &str, args: Vec<RedisDataType>) -> RedisDataType{
    let uppercase_command = command.to_uppercase();
    return match &uppercase_command[..] {
        "PING" => ping(args),
        "ECHO" => echo(args),
        _ => RedisDataType::SimpleError(format!("Invalid command: {}", command).to_string()),
    }
}

fn array(command_array: Vec<RedisDataType>) -> RedisDataType {
    let command = match &command_array[0] {
        RedisDataType::BulkString(s) => s,
        RedisDataType::SimpleString(s) => s,
        _ => return RedisDataType::SimpleError("Invalid command".to_string()),
    };
    let args = command_array[1..].to_vec(); // Convert the slice to a Vec
    return parse_command(&command, args); // Pass the &str instead of a reference
}

pub fn main(message: &str) -> RedisDataType{
    let command = deserialize::main(message);
    println!("Command received: {:?}", command);
    // parse the message
    if let RedisDataType::SimpleString(_) = command {
        eprintln!("SimpleString");
    } else if let RedisDataType::SimpleError(_) = command {
        eprintln!("SimpleError");
    } else if let RedisDataType::Integer(_) = command {
        eprintln!("Integer");
    } else if let RedisDataType::BulkString(_) = command {
        eprintln!("BulkString");
    } else if let RedisDataType::Array(a) = command.clone() {
        let response = array(a);
        eprintln!("Response from array: {:?}", response);
        return response;
    } else {
        eprintln!("Invalid Redis Data Type sent as command");
    }

    return command;
}