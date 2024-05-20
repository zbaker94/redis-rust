use crate::redis_data_type::RedisDataType;

use crate::serialization::VALID_START_CHARS;
use crate::serialization::AGG_TYPE_CHARS;



// Define a trait for SimpleString variant
    fn simple_string(tokens: Vec<&str>) -> RedisDataType {

        return RedisDataType::SimpleString(tokens[0].to_string());
    }

    fn simple_error(tokens: Vec<&str>) -> RedisDataType {

        return RedisDataType::SimpleError(tokens[0].to_string());
    }

    fn integer(tokens: Vec<&str>) -> RedisDataType {
        let parsed_int = tokens[0].parse::<i64>();
        match parsed_int {
            Ok(i) => return RedisDataType::Integer(i),
            Err(e) => return RedisDataType::SimpleError(format!("Error parsing integer: {}", e))
        }
    }

    fn bulk_string(tokens: Vec<&str>) -> RedisDataType {
        let length = tokens[0].parse::<i64>();

        if length.is_err() {
            return RedisDataType::SimpleError(format!("Error parsing bulk string length as integer: {}", length.err().unwrap()));
        }

        let length = length.unwrap();
        if length < 1 {
            return RedisDataType::BulkString("".to_string()); //TODO how to handle returning null when length == -1
        }

        let string = tokens[1].to_string();
        if string.len() != length as usize {
            return RedisDataType::SimpleError(format!("Length of string does not match specified length: {}", string.len()));
        }

        return RedisDataType::BulkString(string);
    }

    fn array(tokens: Vec<&str>) -> RedisDataType {
        let array_length = tokens[0].trim().parse::<i64>();
        if array_length.is_err() {
            return RedisDataType::SimpleError(format!("Error parsing array length as integer: {}", array_length.err().unwrap()));
        }

        // get each sub element of the array
        let mut array: Vec<RedisDataType> = vec![];
        let mut i = 1;
        while i < tokens.len() {
            // combine sub array tokens into a single string starting from i and ending at i + array_length or the next array start
            let mut element = tokens[i].to_string();
            if AGG_TYPE_CHARS.contains(&element.chars().next().unwrap()){
                let agg_char = element.chars().next().unwrap();
                element += "\r\n";
                let mut j = i + 1;
                while j < tokens.len() && !&tokens[j].starts_with(agg_char){
                    eprintln!("Token: {:?}", tokens[j]);
                    element.push_str(&(tokens[j].to_owned() + "\r\n"));
                    j += 1;
                }
                i = j;
            }else {
                i += 1;
            }
            eprintln!("Element: {:?}", element);
            array.push(redis_data_factory(&element));
            eprintln!("Array: {:?}", array);
            
        }

        if array.len() != array_length.unwrap() as usize {
            return RedisDataType::SimpleError(format!("Array length does not match specified length: {}", array.len()));
        }

        return RedisDataType::Array(array);
    }



// Main function that returns a RedisDataType based on encoded string
fn redis_data_factory(encoded_string: &str) -> RedisDataType {
    if !VALID_START_CHARS.contains(&encoded_string.chars().next().unwrap()) {
        return RedisDataType::SimpleError(format!("Invalid start character: {}", encoded_string.chars().next().unwrap()))
    }

    eprintln!("Encoded string: {:?}", encoded_string);
    let tokens = encoded_string[1..].lines().collect::<Vec<&str>>();

    eprintln!("Tokens: {:?}", tokens);

    if encoded_string.starts_with("+") {
        return simple_string(tokens)
    } else if encoded_string.starts_with("-") {
        return simple_error(tokens)
    } else if encoded_string.starts_with(":") {
        return integer(tokens)
    } else if encoded_string.starts_with("$") {
        return bulk_string(tokens)
    } else if encoded_string.starts_with("*") {
        return array(tokens)
    } 
    else {
        return RedisDataType::SimpleError(format!("Unknown type: {}", encoded_string))
    }
}

pub fn main(string: &str) -> RedisDataType {
    return redis_data_factory(string);
}

