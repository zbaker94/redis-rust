mod serialization;
mod redis_data_type;

fn main() {
    let d = serialization::deserialize::main("*2\r\n*3\r\n:1\r\n:2\r\n:3\r\n*2\r\n+Hello\r\n-World\r\n");

    println!("{:?}", d);
}
