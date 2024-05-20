use crate::redis_data_type::RedisDataType;

fn test_deserialize(string: &str, expected_result: RedisDataType){
    let d = crate::serialization::deserialize::main(&string);
    eprintln!("test deserialize: {:?}", d);
    eprintln!("expected result: {:?}", expected_result);
    assert_eq!(&d, &expected_result);
}

fn test_serialize(data: RedisDataType, expected_result: &str){
    let s = crate::serialization::serialize::main(data);
    eprintln!("test serialize: {:?}", s);
    eprintln!("expected result: {:?}", expected_result);
    assert_eq!(&s, &expected_result);
}

#[test]
pub fn main(){

    let test_cases: Vec<(&str, RedisDataType, &str)> = vec![
        ("+Hello\r\n", RedisDataType::SimpleString("Hello".to_string()), "simple string base case"),
        ("+Hello World\r\n", RedisDataType::SimpleString("Hello World".to_string()), "simple string with special characters 1"),
        ("+Hello World!\r\n", RedisDataType::SimpleString("Hello World!".to_string()), "simple string with special characters 2"),
        ("+\r\n", RedisDataType::SimpleString("".to_string()),"simple string with empty string"),
        ("-World\r\n", RedisDataType::SimpleError("World".to_string()), "simple error base case"),
        ("-World!\r\n", RedisDataType::SimpleError("World!".to_string()), "simple error with special characters 1"),
        ("-Error: Test\r\n", RedisDataType::SimpleError("Error: Test".to_string()), "simple error with special characters 2"),
        ("- \r\n", RedisDataType::SimpleError(" ".to_string()), "simple error with empty string 1"),
        ("-\r\n", RedisDataType::SimpleError("".to_string()), "simple error with empty string 2"),
        (":123\r\n", RedisDataType::Integer(123), "integer base case"),
        (":-999\r\n", RedisDataType::Integer(-999), "integer with negative value"),
        ("$5\r\nHello\r\n", RedisDataType::BulkString("Hello".to_string()), "bulk string base case"),
        ("$6\r\nHello!\r\n", RedisDataType::BulkString("Hello!".to_string()), "bulk string with special characters 1"),
        ("$11\r\nHello World\r\n", RedisDataType::BulkString("Hello World".to_string()), "bulk string with special characters 2"),
        ("$0\r\n\r\n", RedisDataType::BulkString("".to_string()), "bulk string with empty string"),
        // bulk string with nil value
        // ("$-1\r\n", RedisDataType::Null(), // TODO how to handle nil value 
        ("*2\r\n+Hello\r\n-World\r\n", RedisDataType::Array(vec![RedisDataType::SimpleString("Hello".to_string()), RedisDataType::SimpleError("World".to_string())]), "array base case"),
        ("*3\r\n:1\r\n:2\r\n:3\r\n", RedisDataType::Array(vec![RedisDataType::Integer(1), RedisDataType::Integer(2), RedisDataType::Integer(3)]), "array with integers"),
        ("*2\r\n$5\r\nHello\r\n$5\r\nWorld\r\n", RedisDataType::Array(vec![RedisDataType::BulkString("Hello".to_string()), RedisDataType::BulkString("World".to_string())]), "array with bulk strings"),
        ("*2\r\n$5\r\nHello\r\n-World\r\n", RedisDataType::Array(vec![RedisDataType::BulkString("Hello".to_string()), RedisDataType::SimpleError("World".to_string())]), "array with mixed types"),
        ("*2\r\n*3\r\n:1\r\n:2\r\n:3\r\n*2\r\n+Hello\r\n-World\r\n", RedisDataType::Array(vec![RedisDataType::Array(vec![RedisDataType::Integer(1), RedisDataType::Integer(2), RedisDataType::Integer(3)]), RedisDataType::Array(vec![RedisDataType::SimpleString("Hello".to_string()), RedisDataType::SimpleError("World".to_string())])]), "array with nested arrays")
    ];

    for (string, data, name) in test_cases{
        eprintln!("------------");
        eprintln!("Running test case: {}", name);
        test_deserialize(string, data.clone());
        test_serialize(data, string);
        eprintln!("------------");
    }

    let error_cases_deserialize: Vec<(&str, RedisDataType, &str)> = vec![
        ("+Hello World", RedisDataType::SimpleError("Invalid format. String must end with '\r\n'. Passed string ends with d".to_string()), "simple string no crlf at end"),
        ("=Hello World\r\n", RedisDataType::SimpleError("Invalid start character: =".to_string()), "simple string with no valid start character 1"),
        ("Hello World\r\n", RedisDataType::SimpleError("Invalid start character: H".to_string()), "simple string with no valid start character 2"),
        ("-World", RedisDataType::SimpleError("Invalid format. String must end with '\r\n'. Passed string ends with d".to_string()), "simple error no crlf at end 1"),
        ("-World\r", RedisDataType::SimpleError("Invalid format. String must end with '\r\n'. Passed string ends with \r".to_string()), "simple error no crlf at end 2"),
        ("-World\n", RedisDataType::SimpleError("Invalid format. String must end with '\r\n'. Passed string ends with \n".to_string()), "simple error no crlf at end 3"),
        ("=World\r\n", RedisDataType::SimpleError("Invalid start character: =".to_string()), "simple error with no valid start character 1"),
        ("World\r\n", RedisDataType::SimpleError("Invalid start character: W".to_string()), "simple error with no valid start character 2"),
        (":123", RedisDataType::SimpleError("Invalid format. String must end with '\r\n'. Passed string ends with 3".to_string()), "integer no crlf at end 1"),
        (":123\r", RedisDataType::SimpleError("Invalid format. String must end with '\r\n'. Passed string ends with \r".to_string()), "integer no crlf at end 2"),
        (":123\n", RedisDataType::SimpleError("Invalid format. String must end with '\r\n'. Passed string ends with \n".to_string()), "integer no crlf at end 3"),
        ("=123\r\n", RedisDataType::SimpleError("Invalid start character: =".to_string()), "integer with no valid start character 1"),
        ("123\r\n", RedisDataType::SimpleError("Invalid start character: 1".to_string()), "integer with no valid start character 2"),
        ("$5\r\nHello", RedisDataType::SimpleError("Invalid format. String must end with '\r\n'. Passed string ends with o".to_string()), "bulk string no crlf at end 1"),
        ("$5\r\nHello\r", RedisDataType::SimpleError("Invalid format. String must end with '\r\n'. Passed string ends with \r".to_string()), "bulk string no crlf at end 2"),
        ("$5\r\nHello\n", RedisDataType::SimpleError("Invalid format. String must end with '\r\n'. Passed string ends with \n".to_string()), "bulk string no crlf at end 3"),
        ("@5\r\nHello\r\n", RedisDataType::SimpleError("Invalid start character: @".to_string()), "bulk string with no valid start character 1"),
        ("5\r\nHello\r\n", RedisDataType::SimpleError("Invalid start character: 5".to_string()), "bulk string with no valid start character 2"),
        ("$\r\nHello\r\n", RedisDataType::SimpleError("Error parsing bulk string length as integer: cannot parse integer from empty string".to_string()), "bulk string with no length"),
        ("$5\r\n", RedisDataType::SimpleError("Invalid format. Bulk strings must contain a token for length and a token containing a string of that length".to_string()), "bulk string with no string"),
        ("$2\r\nHello\r\n", RedisDataType::SimpleError(format!("Length of string does not match specified length: {}", 5).to_string()), "bulk string with incorrect length"),

    ];

    for (string, error, name) in error_cases_deserialize{
        eprintln!("------------");
        eprintln!("Running test case: {}", name);
        test_deserialize(string, error);
        eprintln!("------------");
    }

}