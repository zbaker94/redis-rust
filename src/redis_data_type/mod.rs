use std::fmt;

#[derive(Debug)]
pub enum RedisDataType{
    SimpleString(String),
    SimpleError(String),
    Integer(i64),
    BulkString(String),
    Array(Vec<RedisDataType>),
}

impl fmt::Display for RedisDataType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RedisDataType::SimpleString(s) => write!(f, "{}", s),
            RedisDataType::SimpleError(s) => write!(f, "{}", s),
            RedisDataType::Integer(i) => write!(f, "{}", i),
            RedisDataType::BulkString(s) => write!(f, "ERROR: {}", s),
            RedisDataType::Array(a) => {
                let mut result = String::new();
                for item in a {
                    result.push_str(&format!("{}, ", item));
                }
                write!(f, "[{}]", result)
            }
        }
    }
}