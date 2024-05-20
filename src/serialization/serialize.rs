use crate::redis_data_type::RedisDataType;

fn simple_string(s: String) -> String {
    return format!("+{}\r\n", s);
}

fn simple_error(s: String) -> String {
    return format!("-{}\r\n", s);
}

fn integer(i: i64) -> String {
    return format!(":{}\r\n", i);
}

fn bulk_string(s: String) -> String {
    return format!("${}\r\n{}\r\n", s.len(), s);
}

fn array(a: Vec<RedisDataType>) -> String {
    let mut result = String::new();
    let mut index = 0;
    for item in &a {
        let string = redis_string_factory(item.clone());
        if index == a.len() - 1{
            result.push_str(&string);
            break;
        }
        result.push_str(&string);
        index += 1;
    }
    return format!("*{}\r\n{}", a.len(), result);
}

fn redis_string_factory(data: RedisDataType) -> String{
    if let RedisDataType::SimpleString(s) = data {
        return simple_string(s)
    } else if let RedisDataType::SimpleError(s) = data{
        return simple_error(s);
    } else if let RedisDataType::Integer(i) = data{
        return integer(i);
    } else if let RedisDataType::BulkString(s) = data{
        return bulk_string(s);
    } else if let RedisDataType::Array(a) = data{
        return array(a);
    } else {
        return "Error: Invalid Redis Data Type".to_string()
    }
}

pub fn main(data: RedisDataType) -> String {
    return redis_string_factory(data)
}